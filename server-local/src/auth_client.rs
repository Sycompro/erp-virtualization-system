use anyhow::Result;
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

#[derive(Debug, Serialize, Deserialize)]
struct ValidateTokenRequest {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidateTokenResponse {
    valid: bool,
    user: Option<User>,
}

pub struct AuthClient {
    railway_api_url: String,
    client: reqwest::Client,
}

impl AuthClient {
    pub async fn new() -> Result<Self> {
        let railway_api_url = std::env::var("RAILWAY_API_URL")
            .unwrap_or_else(|_| "https://your-railway-app.railway.app".to_string());

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self {
            railway_api_url,
            client,
        })
    }

    pub async fn validate_token(&self, token: &str) -> Result<User> {
        let request = ValidateTokenRequest {
            token: token.to_string(),
        };

        let response = self.client
            .post(&format!("{}/auth/validate", self.railway_api_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Token validation failed"));
        }

        let validation_response: ValidateTokenResponse = response.json().await?;

        if validation_response.valid {
            validation_response.user
                .ok_or_else(|| anyhow::anyhow!("Valid token but no user data"))
        } else {
            Err(anyhow::anyhow!("Invalid token"))
        }
    }

    pub async fn check_railway_connection(&self) -> Result<bool> {
        match self.client
            .get(&format!("{}/health", self.railway_api_url))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}