use anyhow::Result;
use axum::extract::ws::{WebSocket, Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tracing::{info, error, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingSession {
    pub session_id: String,
    pub container_id: String,
    pub user_id: String,
    pub protocol: String, // "webrtc", "vnc", "rdp"
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamingMessage {
    #[serde(rename = "webrtc_offer")]
    WebRTCOffer {
        container_id: String,
        offer: serde_json::Value,
    },
    #[serde(rename = "webrtc_answer")]
    WebRTCAnswer {
        container_id: String,
        answer: serde_json::Value,
    },
    #[serde(rename = "ice_candidate")]
    IceCandidate {
        container_id: String,
        candidate: serde_json::Value,
    },
    #[serde(rename = "vnc_frame")]
    VncFrame {
        container_id: String,
        frame_data: String, // Base64 encoded
    },
    #[serde(rename = "input_event")]
    InputEvent {
        container_id: String,
        event_type: String, // "mouse", "keyboard", "touch"
        data: serde_json::Value,
    },
    #[serde(rename = "ping")]
    Ping {
        timestamp: i64,
    },
    #[serde(rename = "pong")]
    Pong {
        timestamp: i64,
    },
}

pub struct StreamingService {
    active_sessions: Arc<RwLock<HashMap<String, StreamingSession>>>,
    websocket_connections: Arc<RwLock<HashMap<String, tokio::sync::mpsc::UnboundedSender<Message>>>>,
    webrtc_peers: Arc<Mutex<HashMap<String, WebRTCPeer>>>,
}

struct WebRTCPeer {
    peer_connection: Option<webrtc::peer_connection::RTCPeerConnection>,
    data_channel: Option<webrtc::data_channel::RTCDataChannel>,
    video_track: Option<webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP>,
}

impl StreamingService {
    pub async fn new() -> Result<Self> {
        info!("🎥 Inicializando Streaming Service");

        Ok(Self {
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            websocket_connections: Arc::new(RwLock::new(HashMap::new())),
            webrtc_peers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn handle_connection(&self, mut socket: WebSocket) {
        let session_id = Uuid::new_v4().to_string();
        info!("🔌 Nueva conexión WebSocket: {}", session_id);

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        
        // Guardar conexión
        self.websocket_connections.write().await.insert(session_id.clone(), tx);

        // Manejar mensajes entrantes
        let sessions = self.active_sessions.clone();
        let webrtc_peers = self.webrtc_peers.clone();
        let connections = self.websocket_connections.clone();
        let session_id_clone = session_id.clone();

        tokio::spawn(async move {
            while let Some(msg) = socket.recv().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Err(e) = Self::handle_text_message(&text, &sessions, &webrtc_peers).await {
                            error!("❌ Error procesando mensaje: {}", e);
                        }
                    }
                    Ok(Message::Binary(data)) => {
                        if let Err(e) = Self::handle_binary_message(&data, &sessions).await {
                            error!("❌ Error procesando datos binarios: {}", e);
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("🔌 Conexión WebSocket cerrada: {}", session_id_clone);
                        break;
                    }
                    Err(e) => {
                        error!("❌ Error en WebSocket: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            // Limpiar conexión
            connections.write().await.remove(&session_id_clone);
        });

        // Manejar mensajes salientes
        while let Some(message) = rx.recv().await {
            if socket.send(message).await.is_err() {
                break;
            }
        }

        info!("🔌 Conexión WebSocket terminada: {}", session_id);
    }

    async fn handle_text_message(
        text: &str,
        sessions: &Arc<RwLock<HashMap<String, StreamingSession>>>,
        webrtc_peers: &Arc<Mutex<HashMap<String, WebRTCPeer>>>,
    ) -> Result<()> {
        let message: StreamingMessage = serde_json::from_str(text)?;

        match message {
            StreamingMessage::WebRTCOffer { container_id, offer } => {
                info!("📞 WebRTC Offer recibido para container: {}", container_id);
                // TODO: Procesar offer y crear answer
            }
            StreamingMessage::WebRTCAnswer { container_id, answer } => {
                info!("📞 WebRTC Answer recibido para container: {}", container_id);
                // TODO: Procesar answer
            }
            StreamingMessage::IceCandidate { container_id, candidate } => {
                info!("🧊 ICE Candidate recibido para container: {}", container_id);
                // TODO: Agregar ICE candidate
            }
            StreamingMessage::InputEvent { container_id, event_type, data } => {
                info!("⌨️ Input event ({}) para container: {}", event_type, container_id);
                // TODO: Enviar input al container
            }
            StreamingMessage::Ping { timestamp } => {
                info!("🏓 Ping recibido: {}", timestamp);
                // TODO: Responder con Pong
            }
            _ => {
                warn!("⚠️ Mensaje no manejado: {:?}", message);
            }
        }

        Ok(())
    }

    async fn handle_binary_message(
        _data: &[u8],
        _sessions: &Arc<RwLock<HashMap<String, StreamingSession>>>,
    ) -> Result<()> {
        // TODO: Manejar datos binarios (frames de video, audio, etc.)
        Ok(())
    }

    pub async fn handle_webrtc_offer(&self, container_id: &str, offer: serde_json::Value) -> Result<serde_json::Value> {
        info!("🎥 Procesando WebRTC offer para container: {}", container_id);

        // TODO: Implementar lógica WebRTC completa
        // Por ahora, devolver un answer mock
        Ok(serde_json::json!({
            "type": "answer",
            "sdp": "v=0\r\no=- 0 0 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\n"
        }))
    }

    pub async fn handle_webrtc_answer(&self, container_id: &str, answer: serde_json::Value) -> Result<()> {
        info!("🎥 Procesando WebRTC answer para container: {}", container_id);
        // TODO: Implementar procesamiento de answer
        Ok(())
    }

    pub async fn handle_ice_candidate(&self, container_id: &str, candidate: serde_json::Value) -> Result<()> {
        info!("🧊 Procesando ICE candidate para container: {}", container_id);
        // TODO: Implementar procesamiento de ICE candidate
        Ok(())
    }

    pub async fn create_streaming_session(&self, container_id: String, user_id: String, protocol: String) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        
        let session = StreamingSession {
            session_id: session_id.clone(),
            container_id,
            user_id,
            protocol,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        };

        self.active_sessions.write().await.insert(session_id.clone(), session);
        
        info!("🎥 Sesión de streaming creada: {}", session_id);
        Ok(session_id)
    }

    pub async fn end_streaming_session(&self, session_id: &str) -> Result<()> {
        if let Some(_session) = self.active_sessions.write().await.remove(session_id) {
            // Limpiar WebRTC peer si existe
            self.webrtc_peers.lock().await.remove(session_id);
            
            info!("🎥 Sesión de streaming terminada: {}", session_id);
        }
        
        Ok(())
    }

    pub async fn get_active_sessions(&self) -> Vec<StreamingSession> {
        self.active_sessions.read().await.values().cloned().collect()
    }

    pub async fn broadcast_to_session(&self, session_id: &str, message: StreamingMessage) -> Result<()> {
        if let Some(tx) = self.websocket_connections.read().await.get(session_id) {
            let json_message = serde_json::to_string(&message)?;
            let _ = tx.send(Message::Text(json_message));
        }
        Ok(())
    }
}