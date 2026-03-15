use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::models::{ContainerCreateResponse, HostConfig, PortBinding};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct StartContainerRequest {
    pub erp_type: String,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub resources: ResourceLimits,
}

#[derive(Debug, Deserialize)]
pub struct ResourceLimits {
    pub cpu_limit: Option<String>,
    pub memory_limit: Option<String>,
    pub storage_limit: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContainerInfo {
    pub container_id: String,
    pub erp_type: String,
    pub status: ContainerStatus,
    pub vnc_port: u16,
    pub rdp_port: Option<u16>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub enum ContainerStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error(String),
}

pub struct ContainerService {
    docker: Docker,
    active_containers: Arc<Mutex<HashMap<String, ContainerInfo>>>,
    erp_images: HashMap<String, ERPImageConfig>,
}

#[derive(Debug, Clone)]
struct ERPImageConfig {
    image_name: String,
    display_protocol: DisplayProtocol,
    default_ports: Vec<u16>,
    environment_vars: HashMap<String, String>,
    volume_mounts: Vec<VolumeMount>,
}

#[derive(Debug, Clone)]
enum DisplayProtocol {
    VNC,
    RDP,
    X11Forward,
    Wayland,
}

#[derive(Debug, Clone)]
struct VolumeMount {
    host_path: String,
    container_path: String,
    read_only: bool,
}

impl ContainerService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("🐳 Inicializando ContainerService con Docker");
        
        let docker = Docker::connect_with_local_defaults()?;
        
        // Verificar conexión con Docker
        let version = docker.version().await?;
        info!("✅ Conectado a Docker Engine v{}", version.version.unwrap_or_default());
        
        // Configurar imágenes ERP disponibles
        let mut erp_images = HashMap::new();
        
        // === SISTEMAS ERP ===
        
        // SAP ERP
        erp_images.insert("sap".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/sap-gui:latest".to_string(),
            display_protocol: DisplayProtocol::VNC,
            default_ports: vec![5900, 6080],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("DISPLAY".to_string(), ":1".to_string());
                env.insert("VNC_RESOLUTION".to_string(), "1920x1080".to_string());
                env.insert("VNC_PASSWORD".to_string(), "changeme".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/opt/sap-data".to_string(),
                    container_path: "/sap/data".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // Oracle ERP
        erp_images.insert("oracle".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/oracle-forms:latest".to_string(),
            display_protocol: DisplayProtocol::RDP,
            default_ports: vec![3389],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("RDP_USER".to_string(), "oracle".to_string());
                env.insert("ORACLE_HOME".to_string(), "/opt/oracle".to_string());
                env
            },
            volume_mounts: vec![],
        });
        
        // Microsoft Dynamics
        erp_images.insert("dynamics".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/dynamics-365:latest".to_string(),
            display_protocol: DisplayProtocol::RDP,
            default_ports: vec![3389],
            environment_vars: HashMap::new(),
            volume_mounts: vec![],
        });
        
        // === APLICACIONES WINDOWS ===
        
        // Microsoft Office Suite
        erp_images.insert("office".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/windows-office:latest".to_string(),
            display_protocol: DisplayProtocol::RDP,
            default_ports: vec![3389],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("OFFICE_VERSION".to_string(), "2021".to_string());
                env.insert("DISPLAY_RESOLUTION".to_string(), "1920x1080".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/documents".to_string(),
                    container_path: "C:\\Users\\Public\\Documents".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // AutoCAD
        erp_images.insert("autocad".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/autocad:latest".to_string(),
            display_protocol: DisplayProtocol::RDP,
            default_ports: vec![3389],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("AUTOCAD_VERSION".to_string(), "2024".to_string());
                env.insert("GPU_ACCELERATION".to_string(), "true".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/cad-files".to_string(),
                    container_path: "C:\\CAD\\Projects".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // Adobe Creative Suite
        erp_images.insert("adobe".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/adobe-creative:latest".to_string(),
            display_protocol: DisplayProtocol::RDP,
            default_ports: vec![3389],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("ADOBE_SUITE".to_string(), "CC2024".to_string());
                env.insert("GPU_MEMORY".to_string(), "4GB".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/creative-assets".to_string(),
                    container_path: "C:\\Creative\\Assets".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // Visual Studio
        erp_images.insert("visualstudio".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/visual-studio:latest".to_string(),
            display_protocol: DisplayProtocol::RDP,
            default_ports: vec![3389],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("VS_VERSION".to_string(), "2022".to_string());
                env.insert("DOTNET_VERSION".to_string(), "8.0".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/source-code".to_string(),
                    container_path: "C:\\Source".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // === APLICACIONES LINUX ===
        
        // LibreOffice Suite
        erp_images.insert("libreoffice".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/libreoffice:latest".to_string(),
            display_protocol: DisplayProtocol::VNC,
            default_ports: vec![5900, 6080],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("DISPLAY".to_string(), ":1".to_string());
                env.insert("VNC_RESOLUTION".to_string(), "1920x1080".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/documents".to_string(),
                    container_path: "/home/user/Documents".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // GIMP (Editor de imágenes)
        erp_images.insert("gimp".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/gimp:latest".to_string(),
            display_protocol: DisplayProtocol::VNC,
            default_ports: vec![5900, 6080],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("DISPLAY".to_string(), ":1".to_string());
                env.insert("VNC_RESOLUTION".to_string(), "1920x1080".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/images".to_string(),
                    container_path: "/home/user/Images".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // Blender (3D)
        erp_images.insert("blender".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/blender:latest".to_string(),
            display_protocol: DisplayProtocol::VNC,
            default_ports: vec![5900, 6080],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("DISPLAY".to_string(), ":1".to_string());
                env.insert("VNC_RESOLUTION".to_string(), "1920x1080".to_string());
                env.insert("GPU_ACCELERATION".to_string(), "true".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/3d-projects".to_string(),
                    container_path: "/home/user/Projects".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // === ESCRITORIO COMPLETO ===
        
        // Windows Desktop completo
        erp_images.insert("windows-desktop".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/windows-desktop:latest".to_string(),
            display_protocol: DisplayProtocol::RDP,
            default_ports: vec![3389],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("WINDOWS_VERSION".to_string(), "Server2022".to_string());
                env.insert("DESKTOP_EXPERIENCE".to_string(), "true".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/user-data".to_string(),
                    container_path: "C:\\Users\\Public".to_string(),
                    read_only: false,
                },
            ],
        });
        
        // Ubuntu Desktop completo
        erp_images.insert("ubuntu-desktop".to_string(), ERPImageConfig {
            image_name: "erp-virtualization/ubuntu-desktop:latest".to_string(),
            display_protocol: DisplayProtocol::VNC,
            default_ports: vec![5900, 6080],
            environment_vars: {
                let mut env = HashMap::new();
                env.insert("DISPLAY".to_string(), ":1".to_string());
                env.insert("VNC_RESOLUTION".to_string(), "1920x1080".to_string());
                env.insert("DESKTOP_ENVIRONMENT".to_string(), "GNOME".to_string());
                env
            },
            volume_mounts: vec![
                VolumeMount {
                    host_path: "/shared/user-data".to_string(),
                    container_path: "/home/user".to_string(),
                    read_only: false,
                },
            ],
        });
        
        Ok(Self {
            docker,
            active_containers: Arc::new(Mutex::new(HashMap::new())),
            erp_images,
        })
    }

    pub async fn start_erp_container(&self, payload: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        let request: StartContainerRequest = serde_json::from_value(payload)?;
        
        info!("🚀 Iniciando container ERP tipo: {} para usuario: {}", 
              request.erp_type, request.user_id);
        
        // Obtener configuración de la imagen ERP
        let erp_config = self.erp_images.get(&request.erp_type)
            .ok_or(format!("Tipo de ERP no soportado: {}", request.erp_type))?;
        
        // Generar puertos únicos para este container
        let vnc_port = self.get_available_port().await?;
        let rdp_port = match erp_config.display_protocol {
            DisplayProtocol::RDP => Some(self.get_available_port().await?),
            _ => None,
        };
        
        // Configurar port bindings
        let mut port_bindings = HashMap::new();
        
        match erp_config.display_protocol {
            DisplayProtocol::VNC => {
                port_bindings.insert(
                    "5900/tcp".to_string(),
                    Some(vec![PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some(vnc_port.to_string()),
                    }]),
                );
                port_bindings.insert(
                    "6080/tcp".to_string(),
                    Some(vec![PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some((vnc_port + 1000).to_string()),
                    }]),
                );
            }
            DisplayProtocol::RDP => {
                if let Some(rdp_port) = rdp_port {
                    port_bindings.insert(
                        "3389/tcp".to_string(),
                        Some(vec![PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
                            host_port: Some(rdp_port.to_string()),
                        }]),
                    );
                }
            }
            _ => {}
        }
        
        // Configurar límites de recursos
        let host_config = HostConfig {
            port_bindings: Some(port_bindings),
            memory: request.resources.memory_limit.as_ref()
                .and_then(|m| m.parse::<i64>().ok()),
            cpu_quota: request.resources.cpu_limit.as_ref()
                .and_then(|c| c.parse::<i64>().ok()),
            // Configuraciones de seguridad
            security_opt: Some(vec![
                "no-new-privileges:true".to_string(),
                "apparmor:docker-default".to_string(),
            ]),
            cap_drop: Some(vec!["ALL".to_string()]),
            cap_add: Some(vec!["CHOWN".to_string(), "SETUID".to_string(), "SETGID".to_string()]),
            read_only_root_fs: Some(false), // ERP necesita escribir archivos temporales
            ..Default::default()
        };
        
        // Configurar variables de entorno
        let mut env_vars = erp_config.environment_vars.clone();
        env_vars.insert("USER_ID".to_string(), request.user_id.to_string());
        env_vars.insert("SESSION_ID".to_string(), request.session_id.to_string());
        
        let env: Vec<String> = env_vars.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        
        // Crear configuración del container
        let config = Config {
            image: Some(erp_config.image_name.clone()),
            env: Some(env),
            host_config: Some(host_config),
            // Configuraciones adicionales de seguridad
            user: Some("1000:1000".to_string()), // Usuario no-root
            working_dir: Some("/app".to_string()),
            ..Default::default()
        };
        
        // Crear container
        let container_name = format!("erp-{}-{}-{}", 
                                   request.erp_type, 
                                   request.user_id, 
                                   Uuid::new_v4());
        
        let options = CreateContainerOptions {
            name: container_name.clone(),
            platform: None,
        };
        
        info!("📦 Creando container: {}", container_name);
        
        let response: ContainerCreateResponse = self.docker
            .create_container(Some(options), config)
            .await?;
        
        let container_id = response.id;
        
        // Iniciar container
        info!("▶️ Iniciando container: {}", container_id);
        
        self.docker
            .start_container(&container_id, None::<StartContainerOptions<String>>)
            .await?;
        
        // Registrar container activo
        let container_info = ContainerInfo {
            container_id: container_id.clone(),
            erp_type: request.erp_type,
            status: ContainerStatus::Starting,
            vnc_port,
            rdp_port,
            created_at: chrono::Utc::now(),
        };
        
        {
            let mut containers = self.active_containers.lock().await;
            containers.insert(container_id.clone(), container_info);
        }
        
        // Esperar a que el container esté listo
        tokio::spawn({
            let docker = self.docker.clone();
            let container_id = container_id.clone();
            let containers = self.active_containers.clone();
            
            async move {
                if let Err(e) = Self::wait_for_container_ready(&docker, &container_id).await {
                    error!("Error esperando container ready: {}", e);
                    
                    // Actualizar estado a error
                    let mut containers = containers.lock().await;
                    if let Some(info) = containers.get_mut(&container_id) {
                        info.status = ContainerStatus::Error(e.to_string());
                    }
                } else {
                    // Actualizar estado a running
                    let mut containers = containers.lock().await;
                    if let Some(info) = containers.get_mut(&container_id) {
                        info.status = ContainerStatus::Running;
                    }
                    info!("✅ Container {} está listo", container_id);
                }
            }
        });
        
        info!("🎉 Container ERP iniciado exitosamente: {}", container_id);
        Ok(container_id)
    }

    pub async fn stop_container(&self, payload: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let container_id = payload["container_id"]
            .as_str()
            .ok_or("container_id requerido")?;
        
        info!("🛑 Deteniendo container: {}", container_id);
        
        // Actualizar estado
        {
            let mut containers = self.active_containers.lock().await;
            if let Some(info) = containers.get_mut(container_id) {
                info.status = ContainerStatus::Stopping;
            }
        }
        
        // Detener container con timeout de 30 segundos
        self.docker.stop_container(container_id, None).await?;
        
        // Remover container
        self.docker.remove_container(container_id, None).await?;
        
        // Remover de containers activos
        {
            let mut containers = self.active_containers.lock().await;
            containers.remove(container_id);
        }
        
        info!("✅ Container detenido y removido: {}", container_id);
        Ok(())
    }
    
    async fn get_available_port(&self) -> Result<u16, Box<dyn std::error::Error>> {
        // En una implementación real, verificarías puertos disponibles
        // Por ahora, generar un puerto aleatorio en el rango 10000-20000
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(10000..20000))
    }
    
    async fn wait_for_container_ready(
        docker: &Docker,
        container_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use tokio::time::{sleep, Duration};
        
        for _ in 0..30 { // Esperar hasta 30 segundos
            let inspect = docker.inspect_container(container_id, None).await?;
            
            if let Some(state) = inspect.state {
                if state.running == Some(true) {
                    // Container está corriendo, verificar si el servicio está listo
                    sleep(Duration::from_secs(2)).await;
                    return Ok(());
                }
                
                if state.dead == Some(true) || state.exit_code.unwrap_or(0) != 0 {
                    return Err("Container falló al iniciar".into());
                }
            }
            
            sleep(Duration::from_secs(1)).await;
        }
        
        Err("Timeout esperando que el container esté listo".into())
    }
    
    pub async fn list_active_containers(&self) -> Vec<ContainerInfo> {
        let containers = self.active_containers.lock().await;
        containers.values().cloned().collect()
    }
}
    
    pub async fn list_available_applications(&self) -> Vec<ApplicationInfo> {
        let mut apps = Vec::new();
        
        for (app_id, config) in &self.erp_images {
            let app_info = ApplicationInfo {
                id: app_id.clone(),
                name: self.get_friendly_name(app_id),
                category: self.get_app_category(app_id),
                description: self.get_app_description(app_id),
                icon_url: format!("/icons/{}.png", app_id),
                display_protocol: format!("{:?}", config.display_protocol),
                supported_features: self.get_supported_features(app_id),
                system_requirements: self.get_system_requirements(app_id),
            };
            apps.push(app_info);
        }
        
        // Ordenar por categoría y nombre
        apps.sort_by(|a, b| {
            a.category.cmp(&b.category)
                .then_with(|| a.name.cmp(&b.name))
        });
        
        apps
    }
    
    fn get_friendly_name(&self, app_id: &str) -> String {
        match app_id {
            // ERP Systems
            "sap" => "SAP GUI".to_string(),
            "oracle" => "Oracle Forms".to_string(),
            "dynamics" => "Microsoft Dynamics 365".to_string(),
            
            // Office Applications
            "office" => "Microsoft Office Suite".to_string(),
            "libreoffice" => "LibreOffice Suite".to_string(),
            
            // Design & CAD
            "autocad" => "AutoCAD".to_string(),
            "adobe" => "Adobe Creative Suite".to_string(),
            "gimp" => "GIMP Image Editor".to_string(),
            "blender" => "Blender 3D".to_string(),
            
            // Development
            "visualstudio" => "Visual Studio".to_string(),
            
            // Full Desktops
            "windows-desktop" => "Windows Desktop".to_string(),
            "ubuntu-desktop" => "Ubuntu Desktop".to_string(),
            
            _ => app_id.to_uppercase(),
        }
    }
    
    fn get_app_category(&self, app_id: &str) -> String {
        match app_id {
            "sap" | "oracle" | "dynamics" => "ERP Systems".to_string(),
            "office" | "libreoffice" => "Office Suite".to_string(),
            "autocad" | "adobe" | "gimp" | "blender" => "Design & CAD".to_string(),
            "visualstudio" => "Development".to_string(),
            "windows-desktop" | "ubuntu-desktop" => "Full Desktop".to_string(),
            _ => "Other".to_string(),
        }
    }
    
    fn get_app_description(&self, app_id: &str) -> String {
        match app_id {
            "sap" => "Sistema ERP empresarial SAP con interfaz completa".to_string(),
            "oracle" => "Oracle Forms y aplicaciones empresariales".to_string(),
            "dynamics" => "Microsoft Dynamics 365 ERP y CRM".to_string(),
            "office" => "Word, Excel, PowerPoint, Outlook completos".to_string(),
            "libreoffice" => "Suite ofimática libre: Writer, Calc, Impress".to_string(),
            "autocad" => "Diseño CAD profesional 2D y 3D".to_string(),
            "adobe" => "Photoshop, Illustrator, InDesign, Premiere".to_string(),
            "gimp" => "Editor de imágenes avanzado".to_string(),
            "blender" => "Modelado, animación y renderizado 3D".to_string(),
            "visualstudio" => "IDE completo para desarrollo .NET".to_string(),
            "windows-desktop" => "Escritorio Windows completo con todas las aplicaciones".to_string(),
            "ubuntu-desktop" => "Escritorio Ubuntu Linux con entorno GNOME".to_string(),
            _ => "Aplicación empresarial".to_string(),
        }
    }
    
    fn get_supported_features(&self, app_id: &str) -> Vec<String> {
        let mut features = vec![
            "Streaming HD".to_string(),
            "Touch optimizado".to_string(),
            "Clipboard sync".to_string(),
        ];
        
        match app_id {
            "autocad" | "adobe" | "blender" => {
                features.push("Aceleración GPU".to_string());
                features.push("Precisión de color".to_string());
            }
            "office" | "libreoffice" => {
                features.push("Impresión remota".to_string());
                features.push("Compartir archivos".to_string());
            }
            "windows-desktop" | "ubuntu-desktop" => {
                features.push("Escritorio completo".to_string());
                features.push("Múltiples aplicaciones".to_string());
                features.push("Gestión de archivos".to_string());
            }
            _ => {}
        }
        
        features
    }
    
    fn get_system_requirements(&self, app_id: &str) -> SystemRequirements {
        match app_id {
            "autocad" | "adobe" | "blender" => SystemRequirements {
                min_ram_gb: 8,
                recommended_ram_gb: 16,
                gpu_required: true,
                min_bandwidth_mbps: 10,
                recommended_bandwidth_mbps: 25,
            },
            "windows-desktop" | "ubuntu-desktop" => SystemRequirements {
                min_ram_gb: 4,
                recommended_ram_gb: 8,
                gpu_required: false,
                min_bandwidth_mbps: 5,
                recommended_bandwidth_mbps: 15,
            },
            _ => SystemRequirements {
                min_ram_gb: 2,
                recommended_ram_gb: 4,
                gpu_required: false,
                min_bandwidth_mbps: 3,
                recommended_bandwidth_mbps: 10,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApplicationInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub icon_url: String,
    pub display_protocol: String,
    pub supported_features: Vec<String>,
    pub system_requirements: SystemRequirements,
}

#[derive(Debug, Serialize)]
pub struct SystemRequirements {
    pub min_ram_gb: u32,
    pub recommended_ram_gb: u32,
    pub gpu_required: bool,
    pub min_bandwidth_mbps: u32,
    pub recommended_bandwidth_mbps: u32,
}