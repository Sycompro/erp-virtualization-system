use axum::extract::ws::{Message, WebSocket};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};
use webrtc::api::APIBuilder;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::RTCPeerConnection;

pub struct StreamingService {
    peer_connections: Arc<Mutex<Vec<Arc<RTCPeerConnection>>>>,
}

impl StreamingService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("🎥 Inicializando StreamingService con WebRTC");
        
        Ok(Self {
            peer_connections: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn handle_connection(&self, mut socket: WebSocket) {
        info!("📱 Nueva conexión WebSocket establecida");

        // Configurar WebRTC con servidores STUN/TURN seguros
        let config = RTCConfiguration {
            ice_servers: vec![
                RTCIceServer {
                    urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        // Crear API WebRTC
        let api = APIBuilder::new().build();
        
        match api.new_peer_connection(config).await {
            Ok(peer_connection) => {
                let pc = Arc::new(peer_connection);
                
                // Agregar a la lista de conexiones activas
                {
                    let mut connections = self.peer_connections.lock().await;
                    connections.push(pc.clone());
                }

                // Configurar handlers de WebRTC
                self.setup_webrtc_handlers(pc.clone()).await;

                // Manejar mensajes WebSocket
                while let Some(msg) = socket.recv().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Err(e) = self.handle_signaling_message(&pc, text).await {
                                error!("Error procesando mensaje de señalización: {}", e);
                            }
                        }
                        Ok(Message::Binary(data)) => {
                            // Manejar datos binarios si es necesario
                            info!("Recibidos {} bytes de datos binarios", data.len());
                        }
                        Ok(Message::Close(_)) => {
                            info!("🔌 Conexión WebSocket cerrada");
                            break;
                        }
                        Err(e) => {
                            warn!("Error en WebSocket: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }

                // Limpiar conexión
                self.cleanup_connection(pc).await;
            }
            Err(e) => {
                error!("Error creando peer connection: {}", e);
            }
        }
    }

    async fn setup_webrtc_handlers(&self, pc: Arc<RTCPeerConnection>) {
        // Handler para cuando se establece la conexión
        pc.on_connection_state_change(Box::new(move |s| {
            info!("🔗 Estado de conexión WebRTC: {:?}", s);
            Box::pin(async {})
        }));

        // Handler para ICE candidates
        pc.on_ice_candidate(Box::new(move |c| {
            if let Some(candidate) = c {
                info!("🧊 Nuevo ICE candidate: {}", candidate.to_string());
                // Aquí enviarías el candidate al cliente
            }
            Box::pin(async {})
        }));

        // Handler para data channels
        pc.on_data_channel(Box::new(move |d| {
            info!("📊 Nuevo data channel: {}", d.label());
            Box::pin(async {})
        }));
    }

    async fn handle_signaling_message(
        &self,
        pc: &Arc<RTCPeerConnection>,
        message: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let signal: serde_json::Value = serde_json::from_str(&message)?;
        
        match signal["type"].as_str() {
            Some("offer") => {
                info!("📥 Procesando WebRTC offer");
                // Procesar offer y crear answer
                self.handle_offer(pc, signal).await?;
            }
            Some("answer") => {
                info!("📤 Procesando WebRTC answer");
                // Procesar answer
                self.handle_answer(pc, signal).await?;
            }
            Some("ice-candidate") => {
                info!("🧊 Procesando ICE candidate");
                // Procesar ICE candidate
                self.handle_ice_candidate(pc, signal).await?;
            }
            _ => {
                warn!("Tipo de mensaje desconocido: {}", message);
            }
        }

        Ok(())
    }

    async fn handle_offer(
        &self,
        pc: &Arc<RTCPeerConnection>,
        offer: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementar manejo de offer WebRTC
        info!("Procesando offer WebRTC");
        Ok(())
    }

    async fn handle_answer(
        &self,
        pc: &Arc<RTCPeerConnection>,
        answer: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementar manejo de answer WebRTC
        info!("Procesando answer WebRTC");
        Ok(())
    }

    async fn handle_ice_candidate(
        &self,
        pc: &Arc<RTCPeerConnection>,
        candidate: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementar manejo de ICE candidate
        info!("Procesando ICE candidate");
        Ok(())
    }

    async fn cleanup_connection(&self, pc: Arc<RTCPeerConnection>) {
        info!("🧹 Limpiando conexión WebRTC");
        
        // Cerrar peer connection
        if let Err(e) = pc.close().await {
            error!("Error cerrando peer connection: {}", e);
        }

        // Remover de la lista de conexiones activas
        let mut connections = self.peer_connections.lock().await;
        connections.retain(|conn| !Arc::ptr_eq(conn, &pc));
        
        info!("Conexiones activas: {}", connections.len());
    }

    pub async fn start_screen_capture(&self, container_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("🖥️ Iniciando captura de pantalla para container: {}", container_id);
        
        // Aquí implementarías la captura de pantalla usando:
        // - Wayland + Pipewire (Linux moderno)
        // - O RDP/VNC para compatibilidad
        
        Ok(())
    }
}