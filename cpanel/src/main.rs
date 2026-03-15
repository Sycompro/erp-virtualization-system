use axum::{
    extract::{State, WebSocketUpgrade, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use serde::{Deserialize, Serialize};

mod streaming;
mod container;
mod auth_client;
mod screen_capture;
mod test_endpoints;

use streaming::StreamingService;
use container::ContainerService;
use auth_client::AuthClient;
use screen_capture::ScreenCaptureService;

#[derive(Clone)]
pub struct AppState {
    streaming_service: Arc<StreamingService>,
    container_service: Arc<ContainerService>,
    auth_client: Arc<AuthClient>,
    screen_capture: Arc<ScreenCaptureService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("🏠 Iniciando ERP Local Streaming Server");

    // Inicializar servicios
    let auth_client = Arc::new(AuthClient::new().await?);
    let container_service = Arc::new(ContainerService::new().await?);
    let screen_capture = Arc::new(ScreenCaptureService::new());
    let streaming_service = Arc::new(StreamingService::new(screen_capture.clone()).await?);

    let state = AppState {
        streaming_service,
        container_service,
        auth_client,
        screen_capture,
    };

    // Configurar rutas
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/stream/connect", get(websocket_handler))
        .route("/stream/start", post(start_streaming))
        .route("/stream/stop", post(stop_streaming))
        .route("/container/start", post(start_container))
        .route("/container/stop", post(stop_container))
        .route("/container/status", get(container_status))
        .route("/webrtc/offer", post(webrtc_offer))
        .route("/webrtc/answer", post(webrtc_answer))
        .route("/webrtc/ice", post(webrtc_ice_candidate))
        .route("/test/health", get(test_endpoints::test_system_health))
        .route("/test/container", post(test_endpoints::test_container_lifecycle))
        .route("/test/webrtc", get(test_endpoints::test_webrtc_signaling))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Iniciar servidor local
    let port = std::env::var("LOCAL_SERVER_PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    info!("🌐 Servidor Local ejecutándose en http://{}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "erp-local-server",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now()
    }))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        state.streaming_service.handle_connection(socket).await;
    })
}

#[derive(Deserialize)]
struct StartContainerRequest {
    token: String,
    app_type: String,
    user_id: String,
}

async fn start_container(
    State(state): State<AppState>,
    Json(payload): Json<StartContainerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validar token con Railway API
    match state.auth_client.validate_token(&payload.token).await {
        Ok(user) => {
            if user.id.to_string() != payload.user_id {
                return Err(StatusCode::FORBIDDEN);
            }
            
            // Iniciar container
            match state.container_service.start_container(&payload.app_type, &user.id.to_string()).await {
                Ok(container_info) => {
                    info!("✅ Container iniciado para usuario {}: {}", user.username, container_info.container_id);
                    Ok(Json(serde_json::json!({
                        "container_id": container_info.container_id,
                        "status": "starting",
                        "vnc_port": container_info.vnc_port,
                        "rdp_port": container_info.rdp_port,
                        "webrtc_url": format!("ws://localhost:8080/stream/connect?container_id={}", container_info.container_id)
                    })))
                }
                Err(e) => {
                    error!("❌ Error iniciando container: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct StopContainerRequest {
    token: String,
    container_id: String,
}

async fn stop_container(
    State(state): State<AppState>,
    Json(payload): Json<StopContainerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validar token
    match state.auth_client.validate_token(&payload.token).await {
        Ok(_user) => {
            match state.container_service.stop_container(&payload.container_id).await {
                Ok(_) => {
                    info!("🛑 Container detenido: {}", payload.container_id);
                    Ok(Json(serde_json::json!({
                        "status": "stopped",
                        "container_id": payload.container_id
                    })))
                }
                Err(e) => {
                    error!("❌ Error deteniendo container: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct ContainerStatusQuery {
    token: String,
    container_id: String,
}

async fn container_status(
    State(state): State<AppState>,
    Query(params): Query<ContainerStatusQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validar token
    match state.auth_client.validate_token(&params.token).await {
        Ok(_user) => {
            match state.container_service.get_container_status(&params.container_id).await {
                Ok(status) => Ok(Json(status)),
                Err(_) => Err(StatusCode::NOT_FOUND),
            }
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct WebRTCOfferRequest {
    token: String,
    container_id: String,
    offer: serde_json::Value,
}

async fn webrtc_offer(
    State(state): State<AppState>,
    Json(payload): Json<WebRTCOfferRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.auth_client.validate_token(&payload.token).await {
        Ok(_user) => {
            match state.streaming_service.handle_webrtc_offer(&payload.container_id, payload.offer).await {
                Ok(answer) => Ok(Json(serde_json::json!({
                    "answer": answer
                }))),
                Err(e) => {
                    error!("❌ Error procesando WebRTC offer: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct WebRTCAnswerRequest {
    token: String,
    container_id: String,
    answer: serde_json::Value,
}

async fn webrtc_answer(
    State(state): State<AppState>,
    Json(payload): Json<WebRTCAnswerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.auth_client.validate_token(&payload.token).await {
        Ok(_user) => {
            match state.streaming_service.handle_webrtc_answer(&payload.container_id, payload.answer).await {
                Ok(_) => Ok(Json(serde_json::json!({
                    "status": "answer_processed"
                }))),
                Err(e) => {
                    error!("❌ Error procesando WebRTC answer: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct WebRTCIceRequest {
    token: String,
    container_id: String,
    candidate: serde_json::Value,
}

async fn webrtc_ice_candidate(
    State(state): State<AppState>,
    Json(payload): Json<WebRTCIceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.auth_client.validate_token(&payload.token).await {
        Ok(_user) => {
            match state.streaming_service.handle_ice_candidate(&payload.container_id, payload.candidate).await {
                Ok(_) => Ok(Json(serde_json::json!({
                    "status": "ice_candidate_added"
                }))),
                Err(e) => {
                    error!("❌ Error procesando ICE candidate: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct StartStreamingRequest {
    token: String,
    container_id: String,
    config: Option<serde_json::Value>,
}

async fn start_streaming(
    State(state): State<AppState>,
    Json(payload): Json<StartStreamingRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.auth_client.validate_token(&payload.token).await {
        Ok(user) => {
            // Configuración por defecto o personalizada
            let config = if let Some(config_json) = payload.config {
                serde_json::from_value(config_json).unwrap_or_default()
            } else {
                screen_capture::CaptureConfig::default()
            };

            match state.screen_capture.start_capture(payload.container_id.clone(), config).await {
                Ok(mut frame_receiver) => {
                    // Crear sesión de streaming
                    let session_id = state.streaming_service.create_streaming_session(
                        payload.container_id.clone(),
                        user.id.to_string(),
                        "webrtc".to_string(),
                    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                    // Iniciar task para enviar frames via WebSocket
                    let streaming_service = state.streaming_service.clone();
                    let container_id = payload.container_id.clone();
                    
                    tokio::spawn(async move {
                        while let Some(frame) = frame_receiver.recv().await {
                            // Convertir frame a base64 para WebSocket
                            let frame_b64 = base64::encode(&frame.data);
                            
                            let message = streaming::StreamingMessage::VncFrame {
                                container_id: container_id.clone(),
                                frame_data: frame_b64,
                            };

                            if let Err(e) = streaming_service.broadcast_to_session(&session_id, message).await {
                                error!("❌ Error enviando frame: {}", e);
                                break;
                            }
                        }
                    });

                    Ok(Json(serde_json::json!({
                        "status": "streaming_started",
                        "session_id": session_id,
                        "container_id": payload.container_id,
                        "websocket_url": format!("ws://localhost:8081/stream/connect?session_id={}", session_id)
                    })))
                }
                Err(e) => {
                    error!("❌ Error iniciando captura: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct StopStreamingRequest {
    token: String,
    container_id: String,
}

async fn stop_streaming(
    State(state): State<AppState>,
    Json(payload): Json<StopStreamingRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.auth_client.validate_token(&payload.token).await {
        Ok(_user) => {
            // Detener captura de pantalla
            if let Err(e) = state.screen_capture.stop_capture(&payload.container_id).await {
                error!("❌ Error deteniendo captura: {}", e);
            }

            // Terminar sesiones de streaming
            let sessions = state.streaming_service.get_active_sessions().await;
            for session in sessions {
                if session.container_id == payload.container_id {
                    if let Err(e) = state.streaming_service.end_streaming_session(&session.session_id).await {
                        error!("❌ Error terminando sesión: {}", e);
                    }
                }
            }

            Ok(Json(serde_json::json!({
                "status": "streaming_stopped",
                "container_id": payload.container_id
            })))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}