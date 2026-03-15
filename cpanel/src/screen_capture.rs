use anyhow::Result;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, error, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub quality: String, // "low", "medium", "high", "ultra"
    pub codec: String,   // "h264", "vp8", "vp9"
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 30,
            quality: "medium".to_string(),
            codec: "h264".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScreenFrame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub frame_type: FrameType,
}

#[derive(Debug, Clone)]
pub enum FrameType {
    Keyframe,
    Deltaframe,
}

pub struct ScreenCaptureService {
    active_captures: Arc<RwLock<std::collections::HashMap<String, CaptureSession>>>,
}

struct CaptureSession {
    container_id: String,
    config: CaptureConfig,
    frame_sender: mpsc::UnboundedSender<ScreenFrame>,
    capture_handle: Option<tokio::task::JoinHandle<()>>,
}

impl ScreenCaptureService {
    pub fn new() -> Self {
        info!("📹 Inicializando Screen Capture Service");
        Self {
            active_captures: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn start_capture(
        &self,
        container_id: String,
        config: CaptureConfig,
    ) -> Result<mpsc::UnboundedReceiver<ScreenFrame>> {
        info!("🎬 Iniciando captura para container: {}", container_id);

        let (frame_sender, frame_receiver) = mpsc::unbounded_channel();
        
        // Detectar método de captura según el container
        let capture_method = self.detect_capture_method(&container_id).await?;
        
        // Iniciar captura en background
        let capture_handle = self.spawn_capture_task(
            container_id.clone(),
            config.clone(),
            frame_sender.clone(),
            capture_method,
        ).await?;

        let session = CaptureSession {
            container_id: container_id.clone(),
            config,
            frame_sender,
            capture_handle: Some(capture_handle),
        };

        self.active_captures.write().await.insert(container_id, session);
        
        Ok(frame_receiver)
    }

    pub async fn stop_capture(&self, container_id: &str) -> Result<()> {
        info!("🛑 Deteniendo captura para container: {}", container_id);

        if let Some(mut session) = self.active_captures.write().await.remove(container_id) {
            if let Some(handle) = session.capture_handle.take() {
                handle.abort();
            }
        }

        Ok(())
    }

    async fn detect_capture_method(&self, container_id: &str) -> Result<CaptureMethod> {
        // Detectar si el container tiene VNC, RDP, o X11
        let inspect_cmd = Command::new("docker")
            .args(&["inspect", container_id])
            .output()
            .await?;

        if inspect_cmd.status.success() {
            let output = String::from_utf8_lossy(&inspect_cmd.stdout);
            
            if output.contains("5900") || output.contains("VNC") {
                return Ok(CaptureMethod::VNC);
            } else if output.contains("3389") || output.contains("RDP") {
                return Ok(CaptureMethod::RDP);
            } else if output.contains("DISPLAY") {
                return Ok(CaptureMethod::X11);
            }
        }

        // Default a X11 si no se puede detectar
        Ok(CaptureMethod::X11)
    }

    async fn spawn_capture_task(
        &self,
        container_id: String,
        config: CaptureConfig,
        frame_sender: mpsc::UnboundedSender<ScreenFrame>,
        method: CaptureMethod,
    ) -> Result<tokio::task::JoinHandle<()>> {
        let handle = tokio::spawn(async move {
            if let Err(e) = Self::capture_loop(container_id, config, frame_sender, method).await {
                error!("❌ Error en captura de pantalla: {}", e);
            }
        });

        Ok(handle)
    }

    async fn capture_loop(
        container_id: String,
        config: CaptureConfig,
        frame_sender: mpsc::UnboundedSender<ScreenFrame>,
        method: CaptureMethod,
    ) -> Result<()> {
        let frame_interval = std::time::Duration::from_millis(1000 / config.fps as u64);
        let mut interval = tokio::time::interval(frame_interval);

        loop {
            interval.tick().await;

            match Self::capture_frame(&container_id, &config, &method).await {
                Ok(frame) => {
                    if frame_sender.send(frame).is_err() {
                        warn!("📹 Receptor de frames desconectado para container: {}", container_id);
                        break;
                    }
                }
                Err(e) => {
                    error!("❌ Error capturando frame: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }

        Ok(())
    }

    async fn capture_frame(
        container_id: &str,
        config: &CaptureConfig,
        method: &CaptureMethod,
    ) -> Result<ScreenFrame> {
        match method {
            CaptureMethod::VNC => Self::capture_vnc_frame(container_id, config).await,
            CaptureMethod::RDP => Self::capture_rdp_frame(container_id, config).await,
            CaptureMethod::X11 => Self::capture_x11_frame(container_id, config).await,
        }
    }

    async fn capture_vnc_frame(container_id: &str, config: &CaptureConfig) -> Result<ScreenFrame> {
        // Usar vncdo o similar para capturar frame VNC
        let output = Command::new("docker")
            .args(&[
                "exec",
                container_id,
                "vncdo",
                "-s", "localhost:5900",
                "capture",
                "/tmp/screen.png"
            ])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Error capturando VNC frame"));
        }

        // Leer imagen capturada
        let image_data = Command::new("docker")
            .args(&["exec", container_id, "cat", "/tmp/screen.png"])
            .output()
            .await?;

        Ok(ScreenFrame {
            data: image_data.stdout,
            width: config.width,
            height: config.height,
            timestamp: chrono::Utc::now(),
            frame_type: FrameType::Keyframe,
        })
    }

    async fn capture_rdp_frame(container_id: &str, config: &CaptureConfig) -> Result<ScreenFrame> {
        // Usar xfreerdp con captura de pantalla
        let output = Command::new("docker")
            .args(&[
                "exec",
                container_id,
                "xfreerdp",
                "/v:localhost:3389",
                "/u:rdp_user",
                "/p:rdp_pass",
                "/bmp-cache",
                "/screenshot:/tmp/screen.bmp"
            ])
            .output()
            .await?;

        // Convertir BMP a formato más eficiente
        let convert_output = Command::new("docker")
            .args(&[
                "exec",
                container_id,
                "convert",
                "/tmp/screen.bmp",
                "/tmp/screen.jpg"
            ])
            .output()
            .await?;

        if !convert_output.status.success() {
            return Err(anyhow::anyhow!("Error convirtiendo RDP frame"));
        }

        let image_data = Command::new("docker")
            .args(&["exec", container_id, "cat", "/tmp/screen.jpg"])
            .output()
            .await?;

        Ok(ScreenFrame {
            data: image_data.stdout,
            width: config.width,
            height: config.height,
            timestamp: chrono::Utc::now(),
            frame_type: FrameType::Keyframe,
        })
    }

    async fn capture_x11_frame(container_id: &str, config: &CaptureConfig) -> Result<ScreenFrame> {
        // Usar xwd o scrot para capturar X11
        let output = Command::new("docker")
            .args(&[
                "exec",
                "-e", "DISPLAY=:1",
                container_id,
                "scrot",
                "/tmp/screen.png"
            ])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Error capturando X11 frame"));
        }

        let image_data = Command::new("docker")
            .args(&["exec", container_id, "cat", "/tmp/screen.png"])
            .output()
            .await?;

        Ok(ScreenFrame {
            data: image_data.stdout,
            width: config.width,
            height: config.height,
            timestamp: chrono::Utc::now(),
            frame_type: FrameType::Keyframe,
        })
    }

    pub async fn update_capture_config(&self, container_id: &str, new_config: CaptureConfig) -> Result<()> {
        if let Some(session) = self.active_captures.read().await.get(container_id) {
            // Reiniciar captura con nueva configuración
            self.stop_capture(container_id).await?;
            self.start_capture(container_id.to_string(), new_config).await?;
        }
        Ok(())
    }

    pub async fn get_capture_stats(&self, container_id: &str) -> Option<CaptureStats> {
        // TODO: Implementar estadísticas de captura
        None
    }
}

#[derive(Debug, Clone)]
enum CaptureMethod {
    VNC,
    RDP,
    X11,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaptureStats {
    pub fps_actual: f32,
    pub frames_captured: u64,
    pub frames_dropped: u64,
    pub avg_frame_size: u64,
    pub capture_latency_ms: u32,
}

// Extensión para Command async
trait CommandExt {
    async fn output(&mut self) -> Result<std::process::Output>;
}

impl CommandExt for Command {
    async fn output(&mut self) -> Result<std::process::Output> {
        let output = tokio::process::Command::from(std::mem::take(self))
            .output()
            .await?;
        Ok(output)
    }
}