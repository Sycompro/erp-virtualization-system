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
        
        // Ejecutar migraciones si es necesario
        sqlx::migrate!("../database/migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let row = sqlx::query!(
            "SELECT id, username, email, full_name, created_at, last_login, is_active 
             FROM users WHERE username = $1 AND is_active = true",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.id,
                username: row.username,
                email: row.email,
                full_name: row.full_name,
                created_at: row.created_at,
                last_login: row.last_login,
                is_active: row.is_active,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_password_hash(&self, username: &str) -> Result<Option<String>> {
        let row = sqlx::query!(
            "SELECT password_hash FROM users WHERE username = $1 AND is_active = true",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.password_hash))
    }

    pub async fn create_session(&self, user_id: Uuid, session_token: &str, device_id: Option<String>, expires_at: chrono::DateTime<chrono::Utc>) -> Result<Uuid> {
        let session_id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO user_sessions (id, user_id, session_token, device_id, expires_at) 
             VALUES ($1, $2, $3, $4, $5)",
            session_id,
            user_id,
            session_token,
            device_id,
            expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(session_id)
    }

    pub async fn validate_session(&self, token: &str) -> Result<Option<(User, UserSession)>> {
        let row = sqlx::query!(
            "SELECT s.id as session_id, s.user_id, s.session_token, s.device_id, s.device_info, 
                    s.created_at as session_created, s.last_activity, s.expires_at, s.is_active as session_active,
                    u.id as user_id, u.username, u.email, u.full_name, u.created_at as user_created, 
                    u.last_login, u.is_active as user_active
             FROM user_sessions s
             JOIN users u ON s.user_id = u.id
             WHERE s.session_token = $1 AND s.is_active = true AND s.expires_at > NOW() AND u.is_active = true",
            token
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let user = User {
                id: row.user_id,
                username: row.username,
                email: row.email,
                full_name: row.full_name,
                created_at: row.user_created,
                last_login: row.last_login,
                is_active: row.user_active,
            };

            let session = UserSession {
                id: row.session_id,
                user_id: row.user_id,
                session_token: row.session_token,
                device_id: row.device_id,
                device_info: row.device_info,
                ip_address: None, // No almacenado en esta query
                created_at: row.session_created,
                last_activity: row.last_activity,
                expires_at: row.expires_at,
                is_active: row.session_active,
            };

            Ok(Some((user, session)))
        } else {
            Ok(None)
        }
    }

    pub async fn invalidate_session(&self, token: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE user_sessions SET is_active = false WHERE session_token = $1",
            token
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_last_login(&self, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET last_login = NOW() WHERE id = $1",
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_applications(&self) -> Result<Vec<Application>> {
        let rows = sqlx::query!(
            "SELECT id, name, app_type, category, description, image_name, display_protocol, 
                    default_port, icon_url, system_requirements, supported_features, is_active
             FROM applications WHERE is_active = true ORDER BY category, name"
        )
        .fetch_all(&self.pool)
        .await?;

        let applications = rows.into_iter().map(|row| Application {
            id: row.id,
            name: row.name,
            app_type: row.app_type,
            category: row.category,
            description: row.description,
            image_name: row.image_name,
            display_protocol: row.display_protocol,
            default_port: row.default_port,
            icon_url: row.icon_url,
            system_requirements: row.system_requirements,
            supported_features: row.supported_features,
            is_active: row.is_active,
        }).collect();

        Ok(applications)
    }

    pub async fn get_active_sessions(&self) -> Result<Vec<UserSession>> {
        let rows = sqlx::query!(
            "SELECT s.id, s.user_id, s.session_token, s.device_id, s.device_info, s.ip_address,
                    s.created_at, s.last_activity, s.expires_at, s.is_active
             FROM user_sessions s
             WHERE s.is_active = true AND s.expires_at > NOW()
             ORDER BY s.last_activity DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let sessions = rows.into_iter().map(|row| UserSession {
            id: row.id,
            user_id: row.user_id,
            session_token: row.session_token,
            device_id: row.device_id,
            device_info: row.device_info,
            ip_address: row.ip_address.map(|ip| ip.to_string()),
            created_at: row.created_at,
            last_activity: row.last_activity,
            expires_at: row.expires_at,
            is_active: row.is_active,
        }).collect();

        Ok(sessions)
    }

    pub async fn get_system_stats(&self) -> Result<SystemStats> {
        let row = sqlx::query!(
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
            active_users: row.active_users.unwrap_or(0),
            active_sessions: row.active_sessions.unwrap_or(0),
            running_containers: row.running_containers.unwrap_or(0),
            available_applications: row.available_applications.unwrap_or(0),
            activities_last_24h: row.activities_last_24h.unwrap_or(0),
        })
    }
}