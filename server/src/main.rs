use axum::{
    extract::{State, WebSocketUpgrade, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};
use serde::{Deserialize, Serialize};

mod auth;
mod streaming;
mod container;
mod security;

use auth::AuthService;
use streaming::StreamingService;
use container::ContainerService;

#[derive(Clone)]
pub struct AppState {
    auth_service: Arc<AuthService>,
    streaming_service: Arc<StreamingService>,
    container_service: Arc<ContainerService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Iniciando ERP Virtualization Server");

    // Inicializar servicios
    let auth_service = Arc::new(AuthService::new().await?);
    let streaming_service = Arc::new(StreamingService::new().await?);
    let container_service = Arc::new(ContainerService::new().await?);

    let state = AppState {
        auth_service,
        streaming_service,
        container_service,
    };

    // Configurar rutas
    let app = Router::new()
        .route("/", get(health_check))
        .route("/auth/login", post(login))
        .route("/stream/connect", get(websocket_handler))
        .route("/container/start", post(start_container))
        .route("/container/stop", post(stop_container))
        .route("/applications/list", get(list_applications))
        .route("/applications/categories", get(list_categories))
        .route("/container/start-app", get(start_container_with_query))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Iniciar servidor
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    info!("🌐 Servidor ejecutándose en http://0.0.0.0:8080");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "erp-virtualization-server",
        "version": "0.1.0"
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    // Implementar autenticación FIDO2/WebAuthn
    match state.auth_service.authenticate(payload).await {
        Ok(token) => Ok(Json(serde_json::json!({
            "token": token,
            "expires_in": 3600
        }))),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        state.streaming_service.handle_connection(socket).await;
    })
}

async fn start_container(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.container_service.start_erp_container(payload).await {
        Ok(container_id) => Ok(Json(serde_json::json!({
            "container_id": container_id,
            "status": "starting"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn stop_container(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.container_service.stop_container(payload).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "stopped"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_applications(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let applications = state.container_service.list_available_applications().await;
    Ok(Json(serde_json::json!({
        "applications": applications,
        "total_count": applications.len()
    })))
}

async fn list_categories(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let applications = state.container_service.list_available_applications().await;
    let mut categories: std::collections::HashMap<String, Vec<&container::ApplicationInfo>> = std::collections::HashMap::new();
    
    for app in &applications {
        categories.entry(app.category.clone())
            .or_insert_with(Vec::new)
            .push(app);
    }
    
    Ok(Json(serde_json::json!({
        "categories": categories,
        "category_count": categories.len()
    })))
}

#[derive(Deserialize)]
struct StartContainerQuery {
    app_type: String,
    user_id: String,
}

async fn start_container_with_query(
    State(state): State<AppState>,
    Query(params): Query<StartContainerQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let payload = serde_json::json!({
        "erp_type": params.app_type,
        "user_id": params.user_id,
        "session_id": uuid::Uuid::new_v4().to_string(),
        "resources": {
            "cpu_limit": "2000m",
            "memory_limit": "4Gi",
            "storage_limit": "20Gi"
        }
    });
    
    match state.container_service.start_erp_container(payload).await {
        Ok(container_id) => {
            info!("✅ Container iniciado: {}", container_id);
            Ok(Json(serde_json::json!({
                "container_id": container_id,
                "status": "starting",
                "app_type": params.app_type,
                "user_id": params.user_id
            })))
        }
        Err(e) => {
            error!("❌ Error iniciando container: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}