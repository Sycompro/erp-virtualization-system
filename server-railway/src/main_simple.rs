use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("🚂 Iniciando ERP Railway API Service - Modo Simple");

    // Configurar rutas
    let app = Router::new()
        // Panel de Administración (servir archivos estáticos)
        .nest_service("/admin", ServeDir::new("static"))
        .route("/", get(redirect_to_admin))
        
        // API de salud
        .route("/health", get(health_check))
        
        // API de aplicaciones (mock)
        .route("/applications/list", get(list_applications))
        .route("/applications/categories", get(list_categories))
        .route("/sessions/active", get(list_active_sessions))
        
        // API de configuración de visualización (mock)
        .route("/api/settings/config", get(get_visualization_config))
        .route("/api/settings/video", post(save_video_settings))
        .route("/api/settings/all", post(save_all_settings))
        .route("/api/containers/:id/apply-config", post(apply_config_to_container))
        
        // API de estadísticas y contenedores (mock)
        .route("/api/stats", get(get_stats))
        .route("/api/containers", get(get_containers))
        .route("/api/containers/:id/stop", post(stop_container))
        
        // API de sistema
        .route("/system/stats", get(system_stats))
        
        .layer(CorsLayer::permissive());

    // Puerto desde variable de entorno o 3000 por defecto (evitar conflicto con IIS)
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(&addr).await?;
    info!("🌐 Railway API ejecutándose en {}", addr);
    info!("🎛️  Panel de Administración disponible en: http://localhost:{}/admin/", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn redirect_to_admin() -> impl IntoResponse {
    axum::response::Redirect::permanent("/admin/")
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "erp-railway-api-simple",
        "version": "0.1.0",
        "mode": "development",
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

async fn list_applications() -> impl IntoResponse {
    Json(json!({
        "applications": [
            {
                "id": "sap-gui",
                "name": "SAP GUI",
                "app_type": "sap",
                "category": "ERP Systems",
                "description": "Sistema ERP empresarial SAP con interfaz completa",
                "image_name": "erp-virtualization/sap-gui:latest",
                "display_protocol": "VNC",
                "default_port": 5900,
                "is_active": true
            },
            {
                "id": "office",
                "name": "Microsoft Office",
                "app_type": "office",
                "category": "Office Suite",
                "description": "Word, Excel, PowerPoint, Outlook completos",
                "image_name": "erp-virtualization/office:latest",
                "display_protocol": "RDP",
                "default_port": 3389,
                "is_active": true
            },
            {
                "id": "autocad",
                "name": "AutoCAD",
                "app_type": "autocad",
                "category": "Design",
                "description": "Software de diseño asistido por computadora",
                "image_name": "erp-virtualization/autocad:latest",
                "display_protocol": "RDP",
                "default_port": 3390,
                "is_active": true
            },
            {
                "id": "libreoffice",
                "name": "LibreOffice",
                "app_type": "libreoffice",
                "category": "Office Suite",
                "description": "Suite de oficina libre y gratuita",
                "image_name": "erp-virtualization/libreoffice:latest",
                "display_protocol": "VNC",
                "default_port": 5901,
                "is_active": true
            }
        ],
        "total_count": 4
    }))
}

async fn list_categories() -> impl IntoResponse {
    Json(json!({
        "categories": {
            "ERP Systems": [
                {
                    "id": "sap-gui",
                    "name": "SAP GUI",
                    "app_type": "sap"
                }
            ],
            "Office Suite": [
                {
                    "id": "office",
                    "name": "Microsoft Office",
                    "app_type": "office"
                },
                {
                    "id": "libreoffice",
                    "name": "LibreOffice",
                    "app_type": "libreoffice"
                }
            ],
            "Design": [
                {
                    "id": "autocad",
                    "name": "AutoCAD",
                    "app_type": "autocad"
                }
            ]
        },
        "category_count": 3
    }))
}

async fn list_active_sessions() -> impl IntoResponse {
    Json(json!({
        "active_sessions": [
            {
                "id": "session-1",
                "user_id": "tablet1",
                "device_id": "tablet1",
                "ip_address": "192.168.1.101",
                "created_at": "2024-01-15T10:30:00Z",
                "last_activity": "2024-01-15T14:25:00Z",
                "is_active": true
            },
            {
                "id": "session-2",
                "user_id": "tablet2",
                "device_id": "tablet2",
                "ip_address": "192.168.1.102",
                "created_at": "2024-01-15T11:15:00Z",
                "last_activity": "2024-01-15T14:20:00Z",
                "is_active": true
            }
        ],
        "total_count": 2
    }))
}

async fn system_stats() -> impl IntoResponse {
    Json(json!({
        "active_users": 5,
        "active_sessions": 3,
        "running_containers": 2,
        "available_applications": 4,
        "activities_last_24h": 25
    }))
}

async fn get_visualization_config() -> impl IntoResponse {
    Json(json!({
        "video": {
            "resolution": "1920x1080",
            "fps": 60,
            "bitrate": 5000
        },
        "performance": {
            "codec": "h264",
            "quality": 75,
            "hw_accel": true,
            "low_latency": true
        },
        "network": {
            "transport": "webrtc",
            "buffer": 100,
            "auto_reconnect": true,
            "adaptive_bitrate": true
        }
    }))
}

async fn save_video_settings(Json(_payload): Json<serde_json::Value>) -> impl IntoResponse {
    Json(json!({
        "status": "success",
        "message": "Configuración de video guardada (modo demo)"
    }))
}

async fn save_all_settings(Json(_payload): Json<serde_json::Value>) -> impl IntoResponse {
    Json(json!({
        "status": "success",
        "message": "Configuración completa guardada (modo demo)"
    }))
}

async fn apply_config_to_container() -> impl IntoResponse {
    Json(json!({
        "status": "success",
        "message": "Configuración aplicada al contenedor (modo demo)"
    }))
}

async fn get_stats() -> impl IntoResponse {
    Json(json!({
        "active_containers": 2,
        "total_sessions": 3,
        "avg_latency": 25,
        "bandwidth": 15.5
    }))
}

async fn get_containers() -> impl IntoResponse {
    Json(json!([
        {
            "container_id": "sap-container-1",
            "erp_type": "sap",
            "user_id": "tablet1",
            "status": "running"
        },
        {
            "container_id": "office-container-1",
            "erp_type": "office",
            "user_id": "tablet2",
            "status": "running"
        }
    ]))
}

async fn stop_container() -> impl IntoResponse {
    Json(json!({
        "status": "success",
        "message": "Contenedor detenido (modo demo)"
    }))
}