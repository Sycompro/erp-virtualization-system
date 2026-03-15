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
        sqlx::migrate!("./database/migrations").run(&pool).await?;
        
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
}