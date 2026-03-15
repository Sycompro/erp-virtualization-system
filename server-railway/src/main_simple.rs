use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("🚂 ERP Railway API - Modo Producción Simple");

    let app = Router::new()
        // Panel de Administración
        .nest_service("/admin", ServeDir::new("static"))
        .route("/", get(redirect_to_admin))
        
        // API endpoints
        .route("/health", get(health_check))
        .route("/api/stats", get(get_stats))
        .route("/api/containers", get(get_containers))
        .route("/api/settings/config", get(get_config))
        .route("/applications/list", get(get_applications))
        
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(&addr).await?;
    info!("🌐 ERP Panel ejecutándose en {}", addr);
    info!("🎛️ Panel: https://[railway-url]/admin/");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn redirect_to_admin() -> impl IntoResponse {
    axum::response::Redirect::permanent("/admin/")
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "erp-railway-api",
        "version": "1.0.0",
        "mode": "production",
        "features": ["admin_panel", "visualization_config", "container_management"],
        "endpoints": {
            "panel": "/admin/",
            "health": "/health",
            "api_stats": "/api/stats",
            "applications": "/applications/list"
        },
        "timestamp": chrono::Utc::now()
    }))
}

async fn get_stats() -> impl IntoResponse {
    Json(json!({
        "active_containers": 2,
        "total_sessions": 5,
        "avg_latency": 25,
        "bandwidth": 15.5
    }))
}

async fn get_containers() -> impl IntoResponse {
    Json(json!([
        {
            "container_id": "sap-container-001",
            "erp_type": "sap",
            "user_id": "tablet1",
            "status": "running"
        },
        {
            "container_id": "office-container-002", 
            "erp_type": "office",
            "user_id": "tablet2",
            "status": "running"
        }
    ]))
}

async fn get_config() -> impl IntoResponse {
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

async fn get_applications() -> impl IntoResponse {
    Json(json!({
        "applications": [
            {
                "id": "sap-gui",
                "name": "SAP GUI",
                "app_type": "sap",
                "category": "ERP Systems",
                "description": "Sistema ERP empresarial SAP",
                "status": "available"
            },
            {
                "id": "ms-office",
                "name": "Microsoft Office",
                "app_type": "office", 
                "category": "Office Suite",
                "description": "Word, Excel, PowerPoint completos",
                "status": "available"
            },
            {
                "id": "autocad",
                "name": "AutoCAD",
                "app_type": "autocad",
                "category": "Design",
                "description": "Software de diseño CAD",
                "status": "available"
            }
        ],
        "total_count": 3
    }))
}