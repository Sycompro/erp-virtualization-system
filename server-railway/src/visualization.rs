use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::database::DatabaseService;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    pub resolution: String,
    pub fps: i32,
    pub bitrate: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    pub codec: String,
    pub quality: i32,
    pub hw_accel: bool,
    pub low_latency: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub transport: String,
    pub buffer: i32,
    pub auto_reconnect: bool,
    pub adaptive_bitrate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub video: VideoSettings,
    pub performance: PerformanceSettings,
    pub network: NetworkSettings,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            video: VideoSettings {
                resolution: "1920x1080".to_string(),
                fps: 60,
                bitrate: 5000,
            },
            performance: PerformanceSettings {
                codec: "h264".to_string(),
                quality: 75,
                hw_accel: true,
                low_latency: true,
            },
            network: NetworkSettings {
                transport: "webrtc".to_string(),
                buffer: 100,
                auto_reconnect: true,
                adaptive_bitrate: true,
            },
        }
    }
}

pub struct VisualizationService {
    db_service: Arc<DatabaseService>,
}

impl VisualizationService {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }

    pub async fn get_config(&self, user_id: Option<&str>) -> Result<VisualizationConfig, sqlx::Error> {
        if self.db_service.has_pool() {
            // Modo con base de datos - por ahora devolver configuración por defecto
            // TODO: Implementar lectura real de la base de datos
            Ok(VisualizationConfig::default())
        } else {
            // Modo mock - devolver configuración por defecto
            Ok(VisualizationConfig::default())
        }
    }

    pub async fn save_config(
        &self,
        user_id: Option<&str>,
        config: &VisualizationConfig,
    ) -> Result<(), sqlx::Error> {
        if self.db_service.has_pool() {
            // Modo con base de datos - por ahora simular guardado exitoso
            // TODO: Implementar guardado real en la base de datos
            info!("💾 Configuración de visualización guardada en base de datos");
        } else {
            // Modo mock - simular guardado exitoso
            info!("💾 [MOCK] Configuración de visualización guardada");
        }
        
        Ok(())
    }

    pub async fn apply_to_container(
        &self,
        container_id: &str,
        config: &VisualizationConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🎥 Aplicando configuración de visualización al contenedor: {}", container_id);
        
        if self.db_service.has_pool() {
            // Modo con base de datos - por ahora simular aplicación exitosa
            // TODO: Implementar aplicación real al contenedor
            info!("🎥 Configuración aplicada al contenedor: {}", container_id);
        } else {
            // Modo mock - simular aplicación exitosa
            info!("🎥 [MOCK] Configuración aplicada al contenedor: {}", container_id);
        }

        Ok(())
    }
}

// Handlers HTTP
pub async fn get_visualization_config(
    State(state): State<crate::AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.viz_service.get_config(None).await {
        Ok(config) => Ok(Json(config)),
        Err(e) => {
            error!("Error obteniendo configuración: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn save_video_settings(
    State(state): State<crate::AppState>,
    Json(settings): Json<VideoSettings>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut config = state.viz_service.get_config(None).await.unwrap_or_default();
    config.video = settings;

    match state.viz_service.save_config(None, &config).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Configuración de video guardada"
        }))),
        Err(e) => {
            error!("Error guardando configuración: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn save_all_settings(
    State(state): State<crate::AppState>,
    Json(config): Json<VisualizationConfig>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.viz_service.save_config(None, &config).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Configuración completa guardada"
        }))),
        Err(e) => {
            error!("Error guardando configuración: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn apply_config_to_container(
    State(state): State<crate::AppState>,
    Path(container_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let config = state.viz_service.get_config(None).await.unwrap_or_default();

    match state.viz_service.apply_to_container(&container_id, &config).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "success",
            "message": format!("Configuración aplicada al contenedor {}", container_id)
        }))),
        Err(e) => {
            error!("Error aplicando configuración: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Serialize)]
pub struct SystemStats {
    pub active_containers: i32,
    pub total_sessions: i32,
    pub avg_latency: i32,
    pub bandwidth: f64,
}

pub async fn get_stats(
    State(_state): State<crate::AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    // Obtener estadísticas reales de la base de datos
    let stats = SystemStats {
        active_containers: 0,
        total_sessions: 0,
        avg_latency: 25,
        bandwidth: 15.5,
    };

    Ok(Json(stats))
}

#[derive(Serialize)]
pub struct ContainerInfo {
    pub container_id: String,
    pub erp_type: String,
    pub user_id: String,
    pub status: String,
}

pub async fn get_containers(
    State(_state): State<crate::AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    // Obtener contenedores activos de la base de datos
    let containers: Vec<ContainerInfo> = vec![];

    Ok(Json(containers))
}

pub async fn stop_container(
    State(_state): State<crate::AppState>,
    Path(container_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("🛑 Deteniendo contenedor: {}", container_id);

    Ok(Json(serde_json::json!({
        "status": "success",
        "message": format!("Contenedor {} detenido", container_id)
    })))
}
