use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tracing::info;
use crate::AppState;

pub async fn test_system_health(State(state): State<AppState>) -> impl IntoResponse {
    info!("🧪 Ejecutando prueba de salud del sistema");

    let mut results = json!({
        "timestamp": chrono::Utc::now(),
        "system_status": "healthy",
        "components": {}
    });

    // Probar Docker
    let docker_status = match tokio::process::Command::new("docker")
        .args(&["version", "--format", "{{.Server.Version}}"])
        .output()
        .await
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            json!({
                "status": "available",
                "version": version,
                "message": "Docker daemon running"
            })
        }
        _ => json!({
            "status": "unavailable",
            "message": "Docker daemon not accessible"
        })
    };

    results["components"]["docker"] = docker_status;

    // Probar containers activos
    let active_containers = state.container_service.list_active_containers().await;
    results["components"]["containers"] = json!({
        "status": "available",
        "active_count": active_containers.len(),
        "containers": active_containers
    });

    // Probar streaming sessions
    let active_sessions = state.streaming_service.get_active_sessions().await;
    results["components"]["streaming"] = json!({
        "status": "available", 
        "active_sessions": active_sessions.len(),
        "sessions": active_sessions
    });

    // Probar conexión a Railway API
    let railway_status = match state.auth_client.test_connection().await {
        Ok(_) => json!({
            "status": "connected",
            "message": "Railway API accessible"
        }),
        Err(e) => json!({
            "status": "disconnected",
            "message": format!("Railway API error: {}", e)
        })
    };

    results["components"]["railway_api"] = railway_status;

    Json(results)
}

pub async fn test_container_lifecycle(State(state): State<AppState>) -> impl IntoResponse {
    info!("🧪 Ejecutando prueba de ciclo de vida de containers");

    let test_user_id = "test-user-123";
    let mut results = json!({
        "test": "container_lifecycle",
        "timestamp": chrono::Utc::now(),
        "steps": []
    });

    // Paso 1: Iniciar container SAP
    let step1 = match state.container_service.start_container("sap", test_user_id).await {
        Ok(container_info) => {
            json!({
                "step": "start_sap_container",
                "status": "success",
                "container_id": container_info.container_id,
                "vnc_port": container_info.vnc_port
            })
        }
        Err(e) => json!({
            "step": "start_sap_container",
            "status": "failed",
            "error": e.to_string()
        })
    };

    results["steps"].as_array_mut().unwrap().push(step1.clone());

    // Si el container se inició correctamente, probamos el streaming
    if step1["status"] == "success" {
        let container_id = step1["container_id"].as_str().unwrap();

        // Paso 2: Iniciar captura de pantalla
        let step2 = match state.screen_capture.start_capture(
            container_id.to_string(),
            crate::screen_capture::CaptureConfig::default()
        ).await {
            Ok(_) => json!({
                "step": "start_screen_capture",
                "status": "success",
                "container_id": container_id
            }),
            Err(e) => json!({
                "step": "start_screen_capture", 
                "status": "failed",
                "error": e.to_string()
            })
        };

        results["steps"].as_array_mut().unwrap().push(step2.clone());

        // Paso 3: Crear sesión de streaming
        let step3 = match state.streaming_service.create_streaming_session(
            container_id.to_string(),
            test_user_id.to_string(),
            "webrtc".to_string()
        ).await {
            Ok(session_id) => json!({
                "step": "create_streaming_session",
                "status": "success",
                "session_id": session_id
            }),
            Err(e) => json!({
                "step": "create_streaming_session",
                "status": "failed", 
                "error": e.to_string()
            })
        };

        results["steps"].as_array_mut().unwrap().push(step3.clone());

        // Esperar un poco para que el container se estabilice
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        // Paso 4: Verificar estado del container
        let step4 = match state.container_service.get_container_status(container_id).await {
            Ok(status) => json!({
                "step": "check_container_status",
                "status": "success",
                "container_status": status
            }),
            Err(e) => json!({
                "step": "check_container_status",
                "status": "failed",
                "error": e.to_string()
            })
        };

        results["steps"].as_array_mut().unwrap().push(step4);

        // Paso 5: Limpiar - detener captura
        let _ = state.screen_capture.stop_capture(container_id).await;
        
        // Paso 6: Limpiar - detener container
        let step6 = match state.container_service.stop_container(container_id).await {
            Ok(_) => json!({
                "step": "stop_container",
                "status": "success"
            }),
            Err(e) => json!({
                "step": "stop_container",
                "status": "failed",
                "error": e.to_string()
            })
        };

        results["steps"].as_array_mut().unwrap().push(step6);
    }

    Json(results)
}

pub async fn test_webrtc_signaling(State(_state): State<AppState>) -> impl IntoResponse {
    info!("🧪 Ejecutando prueba de señalización WebRTC");

    let mock_offer = json!({
        "type": "offer",
        "sdp": "v=0\r\no=- 123456789 123456789 IN IP4 0.0.0.0\r\ns=-\r\nt=0 0\r\n"
    });

    let mock_answer = json!({
        "type": "answer", 
        "sdp": "v=0\r\no=- 987654321 987654321 IN IP4 0.0.0.0\r\ns=-\r\nt=0 0\r\n"
    });

    Json(json!({
        "test": "webrtc_signaling",
        "timestamp": chrono::Utc::now(),
        "mock_offer": mock_offer,
        "mock_answer": mock_answer,
        "ice_candidates": [
            {
                "candidate": "candidate:1 1 UDP 2130706431 192.168.1.100 54400 typ host",
                "sdpMLineIndex": 0,
                "sdpMid": "0"
            }
        ],
        "status": "mock_data_generated"
    }))
}