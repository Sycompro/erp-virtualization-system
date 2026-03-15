use anyhow::Result;
use bollard::{Docker, API_DEFAULT_VERSION};
use bollard::container::{CreateContainerOptions, Config, StartContainerOptions, StopContainerOptions, RemoveContainerOptions};
use bollard::models::{ContainerCreateResponse, HostConfig, PortBinding};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, error, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub container_id: String,
    pub app_type: String,
    pub user_id: String,
    pub status: String,
    pub vnc_port: Option<u16>,
    pub rdp_port: Option<u16>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStatus {
    pub container_id: String,
    pub status: String,
    pub uptime: Option<String>,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<u64>,
}

pub struct ContainerService {
    docker: Docker,
    active_containers: std::sync::Arc<tokio::sync::RwLock<HashMap<String, ContainerInfo>>>,
    port_manager: std::sync::Arc<tokio::sync::Mutex<PortManager>>,
}

struct PortManager {
    vnc_ports: std::collections::HashSet<u16>,
    rdp_ports: std::collections::HashSet<u16>,
    next_vnc_port: u16,
    next_rdp_port: u16,
}

impl PortManager {
    fn new() -> Self {
        Self {
            vnc_ports: std::collections::HashSet::new(),
            rdp_ports: std::collections::HashSet::new(),
            next_vnc_port: 5900,
            next_rdp_port: 3389,
        }
    }

    fn allocate_vnc_port(&mut self) -> u16 {
        while self.vnc_ports.contains(&self.next_vnc_port) {
            self.next_vnc_port += 1;
        }
        let port = self.next_vnc_port;
        self.vnc_ports.insert(port);
        self.next_vnc_port += 1;
        port
    }

    fn allocate_rdp_port(&mut self) -> u16 {
        while self.rdp_ports.contains(&self.next_rdp_port) {
            self.next_rdp_port += 1;
        }
        let port = self.next_rdp_port;
        self.rdp_ports.insert(port);
        self.next_rdp_port += 1;
        port
    }

    fn release_vnc_port(&mut self, port: u16) {
        self.vnc_ports.remove(&port);
    }

    fn release_rdp_port(&mut self, port: u16) {
        self.rdp_ports.remove(&port);
    }
}

impl ContainerService {
    pub async fn new() -> Result<Self> {
        let docker = Docker::connect_with_socket_defaults()?;
        
        // Verificar conexión Docker
        match docker.version().await {
            Ok(version) => info!("🐳 Docker conectado: {}", version.version.unwrap_or_default()),
            Err(e) => {
                error!("❌ Error conectando a Docker: {}", e);
                return Err(e.into());
            }
        }

        Ok(Self {
            docker,
            active_containers: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            port_manager: std::sync::Arc::new(tokio::sync::Mutex::new(PortManager::new())),
        })
    }

    pub async fn start_container(&self, app_type: &str, user_id: &str) -> Result<ContainerInfo> {
        let container_name = format!("erp-{}-{}", app_type, Uuid::new_v4().to_string()[..8].to_lowercase());
        
        let (image_name, protocol, env_vars, port_bindings) = match app_type {
            "sap" => {
                let vnc_port = self.port_manager.lock().await.allocate_vnc_port();
                let port_bindings = self.create_vnc_port_bindings(vnc_port);
                (
                    "erp-virtualization/sap-gui:latest",
                    "VNC",
                    vec![
                        format!("DISPLAY=:1"),
                        format!("VNC_RESOLUTION=1920x1080"),
                        format!("VNC_PASSWORD=sap_vnc_{}", user_id),
                        format!("USER_ID={}", user_id),
                    ],
                    port_bindings
                )
            }
            "office" => {
                let rdp_port = self.port_manager.lock().await.allocate_rdp_port();
                let port_bindings = self.create_rdp_port_bindings(rdp_port);
                (
                    "erp-virtualization/office:latest",
                    "RDP",
                    vec![
                        format!("RDP_USER=office_{}", user_id),
                        format!("RDP_PASSWORD=office_pass_{}", user_id),
                        format!("USER_ID={}", user_id),
                    ],
                    port_bindings
                )
            }
            "autocad" => {
                let rdp_port = self.port_manager.lock().await.allocate_rdp_port();
                let port_bindings = self.create_rdp_port_bindings(rdp_port);
                (
                    "erp-virtualization/autocad:latest",
                    "RDP",
                    vec![
                        format!("RDP_USER=cad_{}", user_id),
                        format!("RDP_PASSWORD=cad_pass_{}", user_id),
                        format!("GPU_ACCELERATION=true"),
                        format!("USER_ID={}", user_id),
                    ],
                    port_bindings
                )
            }
            _ => return Err(anyhow::anyhow!("Tipo de aplicación no soportado: {}", app_type)),
        };

        // Configuración del container
        let config = Config {
            image: Some(image_name.to_string()),
            env: Some(env_vars),
            host_config: Some(HostConfig {
                port_bindings: Some(port_bindings),
                memory: Some(4 * 1024 * 1024 * 1024), // 4GB RAM
                cpu_quota: Some(200000), // 2 CPU cores
                restart_policy: Some(bollard::models::RestartPolicy {
                    name: Some(bollard::models::RestartPolicyNameEnum::UNLESS_STOPPED),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Crear container
        info!("🚀 Creando container {} para usuario {}", container_name, user_id);
        
        let create_response: ContainerCreateResponse = self.docker
            .create_container(
                Some(CreateContainerOptions {
                    name: container_name.clone(),
                    ..Default::default()
                }),
                config,
            )
            .await?;

        // Iniciar container
        self.docker
            .start_container(&create_response.id, None::<StartContainerOptions<String>>)
            .await?;

        // Crear info del container
        let container_info = ContainerInfo {
            container_id: create_response.id.clone(),
            app_type: app_type.to_string(),
            user_id: user_id.to_string(),
            status: "starting".to_string(),
            vnc_port: if protocol == "VNC" { Some(5900) } else { None },
            rdp_port: if protocol == "RDP" { Some(3389) } else { None },
            created_at: chrono::Utc::now(),
        };

        // Guardar en memoria
        self.active_containers.write().await.insert(
            create_response.id.clone(),
            container_info.clone(),
        );

        info!("✅ Container {} iniciado correctamente", container_name);
        Ok(container_info)
    }

    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        info!("🛑 Deteniendo container {}", container_id);

        // Detener container
        self.docker
            .stop_container(container_id, Some(StopContainerOptions { t: 10 }))
            .await?;

        // Remover container
        self.docker
            .remove_container(
                container_id,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;

        // Liberar puertos y limpiar memoria
        if let Some(container_info) = self.active_containers.write().await.remove(container_id) {
            let mut port_manager = self.port_manager.lock().await;
            if let Some(vnc_port) = container_info.vnc_port {
                port_manager.release_vnc_port(vnc_port);
            }
            if let Some(rdp_port) = container_info.rdp_port {
                port_manager.release_rdp_port(rdp_port);
            }
        }

        info!("✅ Container {} detenido y removido", container_id);
        Ok(())
    }

    pub async fn get_container_status(&self, container_id: &str) -> Result<ContainerStatus> {
        let inspect = self.docker.inspect_container(container_id, None).await?;
        
        let status = inspect.state
            .and_then(|s| s.status)
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "unknown".to_string());

        Ok(ContainerStatus {
            container_id: container_id.to_string(),
            status,
            uptime: None, // TODO: Calcular uptime
            cpu_usage: None, // TODO: Obtener stats de CPU
            memory_usage: None, // TODO: Obtener stats de memoria
        })
    }

    pub async fn list_active_containers(&self) -> Vec<ContainerInfo> {
        self.active_containers.read().await.values().cloned().collect()
    }

    fn create_vnc_port_bindings(&self, vnc_port: u16) -> HashMap<String, Option<Vec<PortBinding>>> {
        let mut port_bindings = HashMap::new();
        
        port_bindings.insert(
            "5900/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some(vnc_port.to_string()),
            }]),
        );

        // Puerto noVNC web
        port_bindings.insert(
            "6080/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some((vnc_port + 1000).to_string()),
            }]),
        );

        port_bindings
    }

    fn create_rdp_port_bindings(&self, rdp_port: u16) -> HashMap<String, Option<Vec<PortBinding>>> {
        let mut port_bindings = HashMap::new();
        
        port_bindings.insert(
            "3389/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some(rdp_port.to_string()),
            }]),
        );

        port_bindings
    }
}