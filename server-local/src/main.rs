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

use streaming::StreamingService;
use container::ContainerService;
use auth_client::AuthClient;

#[derive(Clone)]
pub struct AppState {
    streaming_service: Arc<StreamingService>,
    container_service: Arc<ContainerService>,
    auth_client: Arc<AuthClient>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("🏠 Iniciando ERP Local Streaming Server");

    // Inicializar servicios
    let auth_client = Arc::new(AuthClient::new().await?);
    let container_service = Arc::new(ContainerService::new().await?);
    let streaming_service = Arc::new(StreamingService::new().await?);

    let state = AppState {
        streaming_service,
        container_service,
        auth_client,
    };

    // Configurar rutas
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/stream/connect", get(websocket_handler))
        .route("/container/start", post(start_container))
        .route("/container/stop", post(stop_container))
        .route("/container/status", get(container_status))
        .route("/webrtc/offer", post(webrtc_offer))
        .route("/webrtc/answer", post(webrtc_answer))
        .route("/webrtc/ice", post(webrtc_ice_candidate))
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