use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use serde::Deserialize;

mod auth;
mod database;
mod models;

use auth::AuthService;
use database::DatabaseService;

#[derive(Clone)]
pub struct AppState {
    auth_service: Arc<AuthService>,
    db_service: Arc<DatabaseService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("🚂 Iniciando ERP Railway API Service");

    // Inicializar servicios
    let db_service = Arc::new(DatabaseService::new().await?);
    let auth_service = Arc::new(AuthService::new(db_service.clone()).await?);

    let state = AppState {
        auth_service,
        db_service,
    };

    // Configurar rutas
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/validate", post(validate_token))
        .route("/users/profile", get(get_user_profile))
        .route("/applications/list", get(list_applications))
        .route("/applications/categories", get(list_categories))
        .route("/sessions/active", get(list_active_sessions))
        .route("/system/stats", get(system_stats))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Puerto desde Railway o 8080 por defecto
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(&addr).await?;
    info!("🌐 Railway API ejecutándose en {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "erp-railway-api",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now()
    }))
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
    device_id: Option<String>,
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.auth_service.authenticate(payload.username, payload.password, payload.device_id).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(Deserialize)]
struct LogoutRequest {
    token: String,
}

async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.auth_service.logout(payload.token).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "logged_out"}))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[derive(Deserialize)]
struct ValidateTokenRequest {
    token: String,
}

async fn validate_token(
    State(state): State<AppState>,
    Json(payload): Json<ValidateTokenRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.auth_service.validate_token(payload.token).await {
        Ok(user_info) => Ok(Json(serde_json::json!({
            "valid": true,
            "user": user_info
        }))),
        Err(_) => Ok(Json(serde_json::json!({
            "valid": false
        }))),
    }
}

async fn get_user_profile(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: Implementar extracción de usuario desde JWT
    Err::<Json<serde_json::Value>, StatusCode>(StatusCode::NOT_IMPLEMENTED)
}

async fn list_applications(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.db_service.get_applications().await {
        Ok(applications) => Ok(Json(serde_json::json!({
            "applications": applications,
            "total_count": applications.len()
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_categories(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.db_service.get_applications().await {
        Ok(applications) => {
            let mut categories: std::collections::HashMap<String, Vec<&models::Application>> = std::collections::HashMap::new();
            
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
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_active_sessions(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.db_service.get_active_sessions().await {
        Ok(sessions) => Ok(Json(serde_json::json!({
            "active_sessions": sessions,
            "total_count": sessions.len()
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn system_stats(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.db_service.get_system_stats().await {
        Ok(stats) => Ok(Json(stats)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}