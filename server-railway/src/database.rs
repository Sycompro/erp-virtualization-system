use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::{User, UserSession, Application, SystemStats};

pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL debe estar configurada");
        
        let pool = PgPool::connect(&database_url).await?;
        
        // Ejecutar migraciones si es necesario (deshabilitado para Railway)
        // Las migraciones se ejecutarán manualmente o via Railway CLI
        // sqlx::migrate!("./database/migrations").run(&pool).await?;
        
        // Ejecutar inicialización de base de datos si es necesario
        Self::run_migrations_if_needed(&pool).await?;
        
        Ok(Self { pool })
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let row: Option<(Uuid, String, String, Option<String>, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>, bool)> = sqlx::query_as(
            "SELECT id, username, email, full_name, created_at, last_login, is_active 
             FROM users WHERE username = $1 AND is_active = true"
        )
        .bind(username)
        .fetch_optional(&self.pool)
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
    }

    pub async fn get_user_password_hash(&self, username: &str) -> Result<Option<String>> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT password_hash FROM users WHERE username = $1 AND is_active = true"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.0))
    }

    pub async fn create_session(&self, user_id: Uuid, session_token: &str, device_id: Option<String>, expires_at: chrono::DateTime<chrono::Utc>) -> Result<Uuid> {
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
        let rows: Vec<(Uuid, String, String, String, Option<String>, String, String, Option<i32>, Option<String>, Option<serde_json::Value>, Option<serde_json::Value>, bool)> = sqlx::query_as(
            "SELECT id, name, app_type, category, description, image_name, display_protocol, 
                    default_port, icon_url, system_requirements, supported_features, is_active
             FROM applications WHERE is_active = true ORDER BY category, name"
        )
        .fetch_all(&self.pool)
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
    }

    pub async fn get_active_sessions(&self) -> Result<Vec<UserSession>> {
        let rows: Vec<(Uuid, Uuid, String, Option<String>, Option<serde_json::Value>, Option<String>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, bool)> = sqlx::query_as(
            "SELECT s.id, s.user_id, s.session_token, s.device_id, s.device_info, s.ip_address::text,
                    s.created_at, s.last_activity, s.expires_at, s.is_active
             FROM user_sessions s
             WHERE s.is_active = true AND s.expires_at > NOW()
             ORDER BY s.last_activity DESC"
        )
        .fetch_all(&self.pool)
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
    }

    pub async fn get_system_stats(&self) -> Result<SystemStats> {
        let row: (i64, i64, i64, i64, i64) = sqlx::query_as(
            "SELECT 
                (SELECT COUNT(*) FROM users WHERE is_active = true) as active_users,
                (SELECT COUNT(*) FROM user_sessions WHERE is_active = true AND expires_at > NOW()) as active_sessions,
                (SELECT COUNT(*) FROM active_containers WHERE status = 'running') as running_containers,
                (SELECT COUNT(*) FROM applications WHERE is_active = true) as available_applications,
                (SELECT COUNT(*) FROM activity_logs WHERE created_at > NOW() - INTERVAL '24 hours') as activities_last_24h"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(SystemStats {
            active_users: row.0,
            active_sessions: row.1,
            running_containers: row.2,
            available_applications: row.3,
            activities_last_24h: row.4,
        })
    }

    async fn run_migrations_if_needed(pool: &PgPool) -> Result<()> {
        // Check if users table exists
        let table_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT FROM information_schema.tables 
                WHERE table_schema = 'public' 
                AND table_name = 'users'
            )"
        )
        .fetch_one(pool)
        .await?;

        if !table_exists {
            tracing::info!("Running database initialization...");
            
            // Create extensions
            let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"").execute(pool).await;
            let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\"").execute(pool).await;
            
            // Create users table
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
            ").execute(pool).await?;
            
            // Create user_sessions table
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
            ").execute(pool).await?;
            
            // Create applications table
            sqlx::query("
                CREATE TABLE IF NOT EXISTS applications (
                    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    name VARCHAR(100) NOT NULL,
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
            ").execute(pool).await?;
            
            // Create active_containers table
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
            ").execute(pool).await?;
            
            // Create activity_logs table
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
            ").execute(pool).await?;
            
            // Create system_config table
            sqlx::query("
                CREATE TABLE IF NOT EXISTS system_config (
                    key VARCHAR(100) PRIMARY KEY,
                    value JSONB NOT NULL,
                    description TEXT,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_by UUID REFERENCES users(id)
                )
            ").execute(pool).await?;
            
            // Create indexes
            let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)").execute(pool).await;
            let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)").execute(pool).await;
            let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_token ON user_sessions(session_token)").execute(pool).await;
            let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_active ON user_sessions(is_active)").execute(pool).await;
            
            // Insert admin user (password: admin123)
            let _ = sqlx::query("
                INSERT INTO users (username, email, password_hash, full_name) VALUES
                ('admin', 'admin@erpvirtualization.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Administrador del Sistema')
                ON CONFLICT (username) DO NOTHING
            ").execute(pool).await;
            
            // Insert tablet users
            for i in 1..=5 {
                let _ = sqlx::query(&format!("
                    INSERT INTO users (username, email, password_hash, full_name) VALUES
                    ('tablet{}', 'tablet{}@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet {}')
                    ON CONFLICT (username) DO NOTHING
                ", i, i, i)).execute(pool).await;
            }
            
            // Insert sample applications
            let _ = sqlx::query("
                INSERT INTO applications (name, app_type, category, description, image_name, display_protocol, default_port, system_requirements, supported_features) VALUES
                ('SAP GUI', 'sap', 'ERP Systems', 'Sistema ERP empresarial SAP con interfaz completa', 'erp-virtualization/sap-gui:latest', 'VNC', 5900, 
                 '{\"min_ram_gb\": 4, \"recommended_ram_gb\": 8, \"gpu_required\": false}',
                 '[\"Streaming HD\", \"Touch optimizado\", \"Clipboard sync\"]'),
                ('Microsoft Office', 'office', 'Office Suite', 'Word, Excel, PowerPoint, Outlook completos', 'erp-virtualization/office:latest', 'RDP', 3389,
                 '{\"min_ram_gb\": 2, \"recommended_ram_gb\": 4, \"gpu_required\": false}',
                 '[\"Streaming HD\", \"Touch optimizado\", \"Clipboard sync\"]')
                ON CONFLICT (name) DO NOTHING
            ").execute(pool).await;
            
            tracing::info!("Database initialization completed");
        } else {
            tracing::info!("Database already initialized, skipping migrations");
        }

        Ok(())
    }
}