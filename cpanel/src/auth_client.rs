use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
struct ValidateTokenRequest {
    token: String,
}

#[derive(Debug, Deserialize)]
struct ValidateTokenResponse {
    valid: bool,
    user: Option<UserInfo>,
}

pub struct AuthClient {
    client: reqwest::Client,
    railway_api_url: String,
}

impl AuthClient {
    pub async fn new() -> Result<Self> {
        let railway_api_url = std::env::var("RAILWAY_API_URL")
            .unwrap_or_else(|_| "https://erp-api-production-6448.up.railway.app".to_string());
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        tracing::info!("🔗 AuthClient configurado para: {}", railway_api_url);
        
        Ok(Self {
            client,
            railway_api_url,
        })
    }
    
    pub async fn validate_token(&self, token: &str) -> Result<UserInfo> {
        let url = format!("{}/auth/validate", self.railway_api_url);
        
        let request = ValidateTokenRequest {
            token: token.to_string(),
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let validate_response: ValidateTokenResponse = response.json().await?;
            
            if validate_response.valid {
                if let Some(user) = validate_response.user {
                    tracing::debug!("✅ Token válido para usuario: {}", user.username);
                    Ok(user)
                } else {
                    anyhow::bail!("Token válido pero sin información de usuario")
                }
            } else {
                anyhow::bail!("Token inválido")
            }
        } else {
            anyhow::bail!("Error validando token: {}", response.status())
        }
    }
    
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.railway_api_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}