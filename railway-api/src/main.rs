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
mod visualization;

use auth::AuthService;
use database::DatabaseService;
use visualization::VisualizationService;

#[derive(Clone)]
pub struct AppState {
    auth_service: Arc<AuthService>,
    db_service: Arc<DatabaseService>,
    viz_service: Arc<VisualizationService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("🚂 Iniciando ERP Railway API Service con Panel de Administración");

    // Inicializar servicios (con manejo de errores mejorado)
    let db_service = match DatabaseService::new().await {
        Ok(service) => {
            info!("✅ Base de datos PostgreSQL conectada");
            Arc::new(service)
        },
        Err(e) => {
            eprintln!("❌ Error conectando a la base de datos: {}", e);
            eprintln!("💡 Iniciando en modo mock (desarrollo sin base de datos)");
            
            // Crear un servicio mock para desarrollo
            Arc::new(DatabaseService::mock())
        }
    };
    
    // Inicializar auth service (puede fallar en modo mock, usar mock también)
    let auth_service = match AuthService::new(db_service.clone()).await {
        Ok(service) => Arc::new(service),
        Err(e) => {
            eprintln!("⚠️ Error inicializando AuthService: {}", e);
            eprintln!("💡 Usando AuthService mock");
            Arc::new(AuthService::mock())
        }
    };
    
    let viz_service = Arc::new(VisualizationService::new(db_service.clone()));

    let state = AppState {
        auth_service,
        db_service: db_service.clone(),
        viz_service: viz_service.clone(),
    };

    // Configurar rutas — todas bajo /api/ para consistencia
    let app = Router::new()
        // Raíz y salud
        .route("/", get(api_info))
        .route("/health", get(health_check))
        
        // API de autenticación
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/validate", post(validate_token))
        .route("/api/users/profile", get(get_user_profile))
        
        // API de aplicaciones
        .route("/api/applications", get(list_applications))
        .route("/api/applications/categories", get(list_categories))
        .route("/api/sessions/active", get(list_active_sessions))
        
        // API de configuración de visualización
        .route("/api/settings/config", get(visualization::get_visualization_config))
        .route("/api/settings/video", post(visualization::save_video_settings))
        .route("/api/settings/all", post(visualization::save_all_settings))
        .route("/api/containers/:id/apply-config", post(visualization::apply_config_to_container))
        
        // API de estadísticas y contenedores
        .route("/api/stats", get(visualization::get_stats))
        .route("/api/containers", get(visualization::get_containers))
        .route("/api/containers/:id/stop", post(visualization::stop_container))
        
        // API de sistema
        .route("/api/system/stats", get(system_stats))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Puerto desde variable de entorno o 3000 por defecto (evitar conflicto con IIS)
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(&addr).await?;
    info!("🌐 Railway API ejecutándose en {}", addr);
    info!("📡 API disponible en: http://localhost:{}/api/", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn api_info() -> impl IntoResponse {
    Json(serde_json::json!({
        "service": "ERP Virtualization API",
        "version": "0.1.0",
        "status": "running",
        "endpoints": {
            "health": "/health",
            "auth_login": "/api/auth/login",
            "auth_validate": "/api/auth/validate",
            "applications": "/api/applications",
            "system_stats": "/api/system/stats"
        }
    }))
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "erp-railway-api",
        "version": "0.1.0",
        "features": ["admin_panel", "visualization_config", "container_management"],
        "admin_panel": "/admin/",
        "endpoints": {
            "panel": "/admin/",
            "health": "/health",
            "api_config": "/api/settings/config",
            "api_stats": "/api/stats"
        },
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
        Err(e) => {
            tracing::error!("Error de autenticación: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
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