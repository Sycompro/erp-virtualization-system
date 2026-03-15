use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub device_id: Option<String>,
    pub device_info: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: Uuid,
    pub name: String,
    pub app_type: String,
    pub category: String,
    pub description: Option<String>,
    pub image_name: String,
    pub display_protocol: String,
    pub default_port: Option<i32>,
    pub icon_url: Option<String>,
    pub system_requirements: Option<serde_json::Value>,
    pub supported_features: Option<serde_json::Value>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_in: i64,
    pub user: User,
    pub local_server_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub active_users: i64,
    pub active_sessions: i64,
    pub running_containers: i64,
    pub available_applications: i64,
    pub activities_last_24h: i64,
}