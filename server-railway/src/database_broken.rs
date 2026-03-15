use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::{User, UserSession, Application, SystemStats};

pub struct DatabaseService {
    pool: Option<PgPool>,
}

impl DatabaseService {
    pub async fn new() -> Result<Self> {
        tracing::info!("🔌 Conectando a la base de datos...");
        
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL no está configurada"))?;
        
        tracing::info!("📊 URL de base de datos configurada");
        
        let pool = PgPool::connect(&database_url).await
            .map_err(|e| anyhow::anyhow!("Error conectando a PostgreSQL: {}", e))?;
        
        tracing::info!("✅ Conexión a PostgreSQL establecida");
        
        // Ejecutar inicialización de base de datos si es necesario
        Self::run_migrations_if_needed(&pool).await
            .map_err(|e| anyhow::anyhow!("Error en inicialización de BD: {}", e))?;
        
        tracing::info!("🎯 DatabaseService inicializado correctamente");
        
        Ok(Self { pool: Some(pool) })
    }
    
    pub fn mock() -> Self {
        tracing::info!("🎭 Iniciando DatabaseService en modo mock (sin PostgreSQL)");
        Self { pool: None }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        if let Some(pool) = &self.pool {
            let row: Option<(Uuid, String, String, Option<String>, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>, bool)> = sqlx::query_as(
                "SELECT id, username, email, full_name, created_at, last_login, is_active 
                 FROM users WHERE username = $1 AND is_active = true"
            )
            .bind(username)
            .fetch_optional(pool)
            .await?;

            if let Some(row) = row {
                Ok(Some(User {
                    id: row.0,
                    username: row.1,
                    email: row.2,
                    full_name: row.3,
                    created_at: row.4,
                    last_login: row.5,
                    is_active: row.6,
                }))
            } else {
                Ok(None)
            }
        } else {
            // Modo mock - devolver usuario de prueba
            if username == "admin" || username.starts_with("tablet") {
                Ok(Some(User {
                    id: Uuid::new_v4(),
                    username: username.to_string(),
                    email: format!("{}@erpvirtualization.com", username),
                    full_name: Some(format!("Usuario {}", username)),
                    created_at: chrono::Utc::now(),
                    last_login: Some(chrono::Utc::now()),
                    is_active: true,
                }))
            } else {
                Ok(None)
            }
        }
    }

    pub async fn get_user_password_hash(&self, username: &str) -> Result<Option<String>> {
        if let Some(pool) = &self.pool {
            let row: Option<(String,)> = sqlx::query_as(
                "SELECT password_hash FROM users WHERE username = $1 AND is_active = true"
            )
            .bind(username)
            .fetch_optional(pool)
            .await?;

            Ok(row.map(|r| r.0))
        } else {
            // Modo mock - devolver hash de "admin123" para usuarios válidos
            if username == "admin" || username.starts_with("tablet") {
                // Hash de "admin123" con bcrypt
                Ok(Some("$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e".to_string()))
            } else {
                Ok(None)
            }
        }
    }

    pub async fn create_session(&self, user_id: Uuid, session_token: &str, device_id: Option<String>, expires_at: chrono::DateTime<chrono::Utc>) -> Result<Uuid> {
        if let Some(pool) = &self.pool {
            let session_id = Uuid::new_v4();
            
            sqlx::query(
                "INSERT INTO user_sessions (id, user_id, session_token, device_id, expires_at) 
                 VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(session_id)
            .bind(user_id)
            .bind(session_token)
            .bind(device_id)
            .bind(expires_at)
            .execute(pool)
            .await?;

            Ok(session_id)
        } else {
            // Modo mock - devolver ID de sesión simulado
            Ok(Uuid::new_v4())
        }
    }

    pub async fn validate_session(&self, token: &str) -> Result<Option<(User, UserSession)>> {
        if let Some(pool) = &self.pool {
            // First get the session
            let session: Option<(Uuid, Uuid, String, Option<String>, Option<serde_json::Value>, Option<String>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, bool)> = sqlx::query_as(
                "SELECT id, user_id, session_token, device_id, device_info, ip_address::text,
                        created_at, last_activity, expires_at, is_active
                 FROM user_sessions 
                 WHERE session_token = $1 AND is_active = true AND expires_at > NOW()"
            )
            .bind(token)
            .fetch_optional(pool)
            .await?;

            if let Some(session_row) = session {
                let session = UserSession {
                    id: session_row.0,
                    user_id: session_row.1,
                    session_token: session_row.2,
                    device_id: session_row.3,
                    device_info: session_row.4,
                    ip_address: session_row.5,
                    created_at: session_row.6,
                    last_activity: session_row.7,
                    expires_at: session_row.8,
                    is_active: session_row.9,
                };

                // Then get the user
                let user = self.get_user_by_id(session.user_id).await?;
                if let Some(user) = user {
                    if user.is_active {
                        return Ok(Some((user, session)));
                    }
                }
            }

            Ok(None)
        } else {
            // Modo mock - validar tokens básicos
            if token.starts_with("mock_") || token.len() > 10 {
                let user = User {
                    id: Uuid::new_v4(),
                    username: "admin".to_string(),
                    email: "admin@erpvirtualization.com".to_string(),
                    full_name: Some("Administrador Mock".to_string()),
                    created_at: chrono::Utc::now(),
                    last_login: Some(chrono::Utc::now()),
                    is_active: true,
                };
                
                let session = UserSession {
                    id: Uuid::new_v4(),
                    user_id: user.id,
                    session_token: token.to_string(),
                    device_id: Some("mock_device".to_string()),
                    device_info: None,
                    ip_address: Some("127.0.0.1".to_string()),
                    created_at: chrono::Utc::now(),
                    last_activity: chrono::Utc::now(),
                    expires_at: chrono::Utc::now() + chrono::Duration::hours(8),
                    is_active: true,
                };
                
                Ok(Some((user, session)))
            } else {
                Ok(None)
            }
        }
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        if let Some(pool) = &self.pool {
            let user: Option<(Uuid, String, String, Option<String>, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>, bool)> = sqlx::query_as(
                "SELECT id, username, email, full_name, created_at, last_login, is_active
                 FROM users WHERE id = $1"
            )
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

            if let Some(row) = user {
                Ok(Some(User {
                    id: row.0,
                    username: row.1,
                    email: row.2,
                    full_name: row.3,
                    created_at: row.4,
                    last_login: row.5,
                    is_active: row.6,
                }))
            } else {
                Ok(None)
            }
        } else {
            // Modo mock
            Ok(Some(User {
                id: user_id,
                username: "admin".to_string(),
                email: "admin@erpvirtualization.com".to_string(),
                full_name: Some("Administrador Mock".to_string()),
                created_at: chrono::Utc::now(),
                last_login: Some(chrono::Utc::now()),
                is_active: true,
            }))
        }
    }

    pub async fn invalidate_session(&self, token: &str) -> Result<()> {
        if let Some(pool) = &self.pool {
            sqlx::query("UPDATE user_sessions SET is_active = false WHERE session_token = $1")
                .bind(token)
                .execute(pool)
                .await?;
        }
        // En modo mock, no hacer nada (simular éxito)
        Ok(())
    }

    pub async fn update_last_login(&self, user_id: Uuid) -> Result<()> {
        if let Some(pool) = &self.pool {
            sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
                .bind(user_id)
                .execute(pool)
                .await?;
        }
        // En modo mock, no hacer nada (simular éxito)
        Ok(())
    }
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(session_id)
    }

    pub async fn validate_session(&self, token: &str) -> Result<Option<(User, UserSession)>> {
        // First get the session
        let session: Option<(Uuid, Uuid, String, Option<String>, Option<serde_json::Value>, Option<String>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, bool)> = sqlx::query_as(
            "SELECT id, user_id, session_token, device_id, device_info, ip_address::text,
                    created_at, last_activity, expires_at, is_active
             FROM user_sessions 
             WHERE session_token = $1 AND is_active = true AND expires_at > NOW()"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(session_row) = session {
            let session = UserSession {
                id: session_row.0,
                user_id: session_row.1,
                session_token: session_row.2,
                device_id: session_row.3,
                device_info: session_row.4,
                ip_address: session_row.5,
                created_at: session_row.6,
                last_activity: session_row.7,
                expires_at: session_row.8,
                is_active: session_row.9,
            };

            // Then get the user
            let user = self.get_user_by_id(session.user_id).await?;
            if let Some(user) = user {
                if user.is_active {
                    return Ok(Some((user, session)));
                }
            }
        }

        Ok(None)
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let user: Option<(Uuid, String, String, Option<String>, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>, bool)> = sqlx::query_as(
            "SELECT id, username, email, full_name, created_at, last_login, is_active
             FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = user {
            Ok(Some(User {
                id: row.0,
                username: row.1,
                email: row.2,
                full_name: row.3,
                created_at: row.4,
                last_login: row.5,
                is_active: row.6,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn invalidate_session(&self, token: &str) -> Result<()> {
        sqlx::query("UPDATE user_sessions SET is_active = false WHERE session_token = $1")
            .bind(token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_last_login(&self, user_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_applications(&self) -> Result<Vec<Application>> {
        if let Some(pool) = &self.pool {
            let rows: Vec<(Uuid, String, String, String, Option<String>, String, String, Option<i32>, Option<String>, Option<serde_json::Value>, Option<serde_json::Value>, bool)> = sqlx::query_as(
                "SELECT id, name, app_type, category, description, image_name, display_protocol, 
                        default_port, icon_url, system_requirements, supported_features, is_active
                 FROM applications WHERE is_active = true ORDER BY category, name"
            )
            .fetch_all(pool)
            .await?;

            let applications = rows.into_iter().map(|row| Application {
                id: row.0,
                name: row.1,
                app_type: row.2,
                category: row.3,
                description: row.4,
                image_name: row.5,
                display_protocol: row.6,
                default_port: row.7,
                icon_url: row.8,
                system_requirements: row.9,
                supported_features: row.10,
                is_active: row.11,
            }).collect();

            Ok(applications)
        } else {
            // Modo mock - devolver aplicaciones de ejemplo
            self.get_applications_mock().await
        }
    }
    
    pub async fn get_applications_mock(&self) -> Result<Vec<Application>> {
        Ok(vec![
            Application {
                id: Uuid::new_v4(),
                name: "SAP GUI".to_string(),
                app_type: "sap".to_string(),
                category: "ERP Systems".to_string(),
                description: Some("Sistema ERP empresarial SAP con interfaz completa".to_string()),
                image_name: "erp-virtualization/sap-gui:latest".to_string(),
                display_protocol: "VNC".to_string(),
                default_port: Some(5900),
                icon_url: Some("/icons/sap.png".to_string()),
                system_requirements: None,
                supported_features: None,
                is_active: true,
            },
            Application {
                id: Uuid::new_v4(),
                name: "Microsoft Office".to_string(),
                app_type: "office".to_string(),
                category: "Office Suite".to_string(),
                description: Some("Word, Excel, PowerPoint, Outlook completos".to_string()),
                image_name: "erp-virtualization/office:latest".to_string(),
                display_protocol: "RDP".to_string(),
                default_port: Some(3389),
                icon_url: Some("/icons/office.png".to_string()),
                system_requirements: None,
                supported_features: None,
                is_active: true,
            },
            Application {
                id: Uuid::new_v4(),
                name: "AutoCAD".to_string(),
                app_type: "autocad".to_string(),
                category: "Design".to_string(),
                description: Some("Software de diseño asistido por computadora".to_string()),
                image_name: "erp-virtualization/autocad:latest".to_string(),
                display_protocol: "RDP".to_string(),
                default_port: Some(3390),
                icon_url: Some("/icons/autocad.png".to_string()),
                system_requirements: None,
                supported_features: None,
                is_active: true,
            },
            Application {
                id: Uuid::new_v4(),
                name: "LibreOffice".to_string(),
                app_type: "libreoffice".to_string(),
                category: "Office Suite".to_string(),
                description: Some("Suite de oficina libre y gratuita".to_string()),
                image_name: "erp-virtualization/libreoffice:latest".to_string(),
                display_protocol: "VNC".to_string(),
                default_port: Some(5901),
                icon_url: Some("/icons/libreoffice.png".to_string()),
                system_requirements: None,
                supported_features: None,
                is_active: true,
            },
        ])
    }

    pub async fn get_active_sessions(&self) -> Result<Vec<UserSession>> {
        if let Some(pool) = &self.pool {
            let rows: Vec<(Uuid, Uuid, String, Option<String>, Option<serde_json::Value>, Option<String>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, bool)> = sqlx::query_as(
                "SELECT s.id, s.user_id, s.session_token, s.device_id, s.device_info, s.ip_address::text,
                        s.created_at, s.last_activity, s.expires_at, s.is_active
                 FROM user_sessions s
                 WHERE s.is_active = true AND s.expires_at > NOW()
                 ORDER BY s.last_activity DESC"
            )
            .fetch_all(pool)
            .await?;

            let sessions = rows.into_iter().map(|row| UserSession {
                id: row.0,
                user_id: row.1,
                session_token: row.2,
                device_id: row.3,
                device_info: row.4,
                ip_address: row.5,
                created_at: row.6,
                last_activity: row.7,
                expires_at: row.8,
                is_active: row.9,
            }).collect();

            Ok(sessions)
        } else {
            // Modo mock - devolver sesiones de ejemplo
            self.get_active_sessions_mock().await
        }
    }
    
    pub async fn get_active_sessions_mock(&self) -> Result<Vec<UserSession>> {
        let now = chrono::Utc::now();
        Ok(vec![
            UserSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                session_token: "mock_session_1".to_string(),
                device_id: Some("tablet1".to_string()),
                device_info: None,
                ip_address: Some("192.168.1.101".to_string()),
                created_at: now - chrono::Duration::hours(2),
                last_activity: now - chrono::Duration::minutes(5),
                expires_at: now + chrono::Duration::hours(6),
                is_active: true,
            },
            UserSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                session_token: "mock_session_2".to_string(),
                device_id: Some("tablet2".to_string()),
                device_info: None,
                ip_address: Some("192.168.1.102".to_string()),
                created_at: now - chrono::Duration::hours(1),
                last_activity: now - chrono::Duration::minutes(2),
                expires_at: now + chrono::Duration::hours(7),
                is_active: true,
            },
        ])
    }

    pub async fn get_system_stats(&self) -> Result<SystemStats> {
        if let Some(pool) = &self.pool {
            let row: (i64, i64, i64, i64, i64) = sqlx::query_as(
                "SELECT 
                    (SELECT COUNT(*) FROM users WHERE is_active = true) as active_users,
                    (SELECT COUNT(*) FROM user_sessions WHERE is_active = true AND expires_at > NOW()) as active_sessions,
                    (SELECT COUNT(*) FROM active_containers WHERE status = 'running') as running_containers,
                    (SELECT COUNT(*) FROM applications WHERE is_active = true) as available_applications,
                    (SELECT COUNT(*) FROM activity_logs WHERE created_at > NOW() - INTERVAL '24 hours') as activities_last_24h"
            )
            .fetch_one(pool)
            .await?;

            Ok(SystemStats {
                active_users: row.0,
                active_sessions: row.1,
                running_containers: row.2,
                available_applications: row.3,
                activities_last_24h: row.4,
            })
        } else {
            // Modo mock - devolver estadísticas de ejemplo
            self.get_system_stats_mock().await
        }
    }
    
    pub async fn get_system_stats_mock(&self) -> Result<SystemStats> {
        Ok(SystemStats {
            active_users: 5,
            active_sessions: 3,
            running_containers: 2,
            available_applications: 4,
            activities_last_24h: 25,
        })
    }

    async fn run_migrations_if_needed(pool: &PgPool) -> Result<()> {
        tracing::info!("🔍 Verificando estado de la base de datos...");
        
        // Check if users table exists
        let table_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT FROM information_schema.tables 
                WHERE table_schema = 'public' 
                AND table_name = 'users'
            )"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| anyhow::anyhow!("Error verificando tablas existentes: {}", e))?;

        if !table_exists {
            tracing::info!("🏗️  Inicializando base de datos (primera vez)...");
            
            // Create extensions
            tracing::info!("📦 Creando extensiones PostgreSQL...");
            let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"").execute(pool).await;
            let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\"").execute(pool).await;
            
            // Create users table
            tracing::info!("👥 Creando tabla users...");
            sqlx::query("
                CREATE TABLE IF NOT EXISTS users (
                    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    username VARCHAR(50) UNIQUE NOT NULL,
                    email VARCHAR(100) UNIQUE NOT NULL,
                    password_hash VARCHAR(255) NOT NULL,
                    full_name VARCHAR(100),
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    last_login TIMESTAMP,
                    is_active BOOLEAN DEFAULT true,
                    failed_login_attempts INTEGER DEFAULT 0,
                    locked_until TIMESTAMP NULL
                )
            ").execute(pool).await
            .map_err(|e| anyhow::anyhow!("Error creando tabla users: {}", e))?;
            
            // Create user_sessions table
            tracing::info!("🔐 Creando tabla user_sessions...");
            sqlx::query("
                CREATE TABLE IF NOT EXISTS user_sessions (
                    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                    session_token VARCHAR(255) UNIQUE NOT NULL,
                    device_id VARCHAR(100),
                    device_info JSONB,
                    ip_address INET,
                    user_agent TEXT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    last_activity TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    expires_at TIMESTAMP NOT NULL,
                    is_active BOOLEAN DEFAULT true
                )
            ").execute(pool).await
            .map_err(|e| anyhow::anyhow!("Error creando tabla user_sessions: {}", e))?;
            
            // Create applications table
            tracing::info!("📱 Creando tabla applications...");
            sqlx::query("
                CREATE TABLE IF NOT EXISTS applications (
                    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    name VARCHAR(100) UNIQUE NOT NULL,
                    app_type VARCHAR(50) NOT NULL,
                    category VARCHAR(50) NOT NULL,
                    description TEXT,
                    image_name VARCHAR(200) NOT NULL,
                    display_protocol VARCHAR(20) NOT NULL DEFAULT 'VNC',
                    default_port INTEGER,
                    icon_url VARCHAR(255),
                    system_requirements JSONB,
                    supported_features JSONB,
                    is_active BOOLEAN DEFAULT true,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            ").execute(pool).await
            .map_err(|e| anyhow::anyhow!("Error creando tabla applications: {}", e))?;
            
            // Create active_containers table
            tracing::info!("🐳 Creando tabla active_containers...");
            sqlx::query("
                CREATE TABLE IF NOT EXISTS active_containers (
                    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    container_id VARCHAR(100) UNIQUE NOT NULL,
                    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                    session_id UUID REFERENCES user_sessions(id) ON DELETE CASCADE,
                    application_id UUID REFERENCES applications(id),
                    app_type VARCHAR(50) NOT NULL,
                    status VARCHAR(20) DEFAULT 'starting',
                    vnc_port INTEGER,
                    rdp_port INTEGER,
                    container_ip INET,
                    resources_allocated JSONB,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    started_at TIMESTAMP,
                    stopped_at TIMESTAMP,
                    last_activity TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            ").execute(pool).await
            .map_err(|e| anyhow::anyhow!("Error creando tabla active_containers: {}", e))?;
            
            // Create activity_logs table
            tracing::info!("📋 Creando tabla activity_logs...");
            sqlx::query("
                CREATE TABLE IF NOT EXISTS activity_logs (
                    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
                    session_id UUID REFERENCES user_sessions(id) ON DELETE SET NULL,
                    container_id UUID REFERENCES active_containers(id) ON DELETE SET NULL,
                    action VARCHAR(50) NOT NULL,
                    details JSONB,
                    ip_address INET,
                    user_agent TEXT,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            ").execute(pool).await
            .map_err(|e| anyhow::anyhow!("Error creando tabla activity_logs: {}", e))?;
            
            // Create system_config table
            tracing::info!("⚙️  Creando tabla system_config...");
            sqlx::query("
                CREATE TABLE IF NOT EXISTS system_config (
                    key VARCHAR(100) PRIMARY KEY,
                    value JSONB NOT NULL,
                    description TEXT,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_by UUID REFERENCES users(id)
                )
            ").execute(pool).await
            .map_err(|e| anyhow::anyhow!("Error creando tabla system_config: {}", e))?;
            
            // Create indexes
            tracing::info!("🔍 Creando índices...");
            let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)").execute(pool).await;
            let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)").execute(pool).await;
            let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_token ON user_sessions(session_token)").execute(pool).await;
            let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_active ON user_sessions(is_active)").execute(pool).await;
            
            // Insert admin user (password: admin123)
            tracing::info!("👤 Creando usuario administrador...");
            let _ = sqlx::query("
                INSERT INTO users (username, email, password_hash, full_name) VALUES
                ('admin', 'admin@erpvirtualization.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Administrador del Sistema')
                ON CONFLICT (username) DO NOTHING
            ").execute(pool).await;
            
            // Insert tablet users
            tracing::info!("📱 Creando usuarios de tablet...");
            for i in 1..=5 {
                let _ = sqlx::query(&format!("
                    INSERT INTO users (username, email, password_hash, full_name) VALUES
                    ('tablet{}', 'tablet{}@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet {}')
                    ON CONFLICT (username) DO NOTHING
                ", i, i, i)).execute(pool).await;
            }
            
            // Insert sample applications
            tracing::info!("🚀 Creando aplicaciones de ejemplo...");
            let _ = sqlx::query("
                INSERT INTO applications (name, app_type, category, description, image_name, display_protocol, default_port, system_requirements, supported_features) 
                SELECT 'SAP GUI', 'sap', 'ERP Systems', 'Sistema ERP empresarial SAP con interfaz completa', 'erp-virtualization/sap-gui:latest', 'VNC', 5900, 
                 '{\"min_ram_gb\": 4, \"recommended_ram_gb\": 8, \"gpu_required\": false}',
                 '[\"Streaming HD\", \"Touch optimizado\", \"Clipboard sync\"]'
                WHERE NOT EXISTS (SELECT 1 FROM applications WHERE name = 'SAP GUI')
            ").execute(pool).await;
            
            let _ = sqlx::query("
                INSERT INTO applications (name, app_type, category, description, image_name, display_protocol, default_port, system_requirements, supported_features) 
                SELECT 'Microsoft Office', 'office', 'Office Suite', 'Word, Excel, PowerPoint, Outlook completos', 'erp-virtualization/office:latest', 'RDP', 3389,
                 '{\"min_ram_gb\": 2, \"recommended_ram_gb\": 4, \"gpu_required\": false}',
                 '[\"Streaming HD\", \"Touch optimizado\", \"Clipboard sync\"]'
                WHERE NOT EXISTS (SELECT 1 FROM applications WHERE name = 'Microsoft Office')
            ").execute(pool).await;
            
            tracing::info!("✅ Inicialización de base de datos completada exitosamente");
        } else {
            tracing::info!("ℹ️  Base de datos ya inicializada, continuando...");
        }

        Ok(())
    }
}