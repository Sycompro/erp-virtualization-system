use std::sync::Arc;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::database::DatabaseService;
use crate::models::{User, AuthResponse};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // user_id
    username: String,
    exp: usize,   // expiration
    iat: usize,   // issued at
    device_id: Option<String>,
}

pub struct AuthService {
    db: Arc<DatabaseService>,
    jwt_secret: String,
    local_server_url: String,
}

impl AuthService {
    pub async fn new(db: Arc<DatabaseService>) -> Result<Self> {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "railway-jwt-secret-change-in-production".to_string());
        
        let local_server_url = std::env::var("LOCAL_SERVER_URL")
            .unwrap_or_else(|_| "ws://192.168.1.100:8080".to_string());

        Ok(Self {
            db,
            jwt_secret,
            local_server_url,
        })
    }

    pub async fn authenticate(&self, username: String, password: String, device_id: Option<String>) -> Result<AuthResponse> {
        // Buscar usuario
        let user = self.db.get_user_by_username(&username).await?
            .ok_or_else(|| anyhow::anyhow!("Usuario no encontrado"))?;

        // Verificar contraseña
        let password_hash = self.db.get_user_password_hash(&username).await?
            .ok_or_else(|| anyhow::anyhow!("Contraseña no encontrada"))?;

        if !verify(&password, &password_hash)? {
            return Err(anyhow::anyhow!("Contraseña incorrecta"));
        }

        // Generar JWT token
        let now = Utc::now();
        let expires_at = now + Duration::hours(8); // 8 horas de sesión
        
        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
            device_id: device_id.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        // Crear sesión en base de datos
        self.db.create_session(
            user.id,
            &token,
            device_id,
            expires_at,
        ).await?;

        // Actualizar último login
        self.db.update_last_login(user.id).await?;

        Ok(AuthResponse {
            token,
            expires_in: 28800, // 8 horas en segundos
            user,
            local_server_url: self.local_server_url.clone(),
        })
    }

    pub async fn validate_token(&self, token: String) -> Result<User> {
        // Validar JWT
        let validation = Validation::new(Algorithm::HS256);
        let _token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )?;

        // Verificar en base de datos
        let (user, _session) = self.db.validate_session(&token).await?
            .ok_or_else(|| anyhow::anyhow!("Sesión inválida o expirada"))?;

        Ok(user)
    }

    pub async fn logout(&self, token: String) -> Result<()> {
        self.db.invalidate_session(&token).await?;
        Ok(())
    }

    pub fn hash_password(password: &str) -> Result<String> {
        Ok(hash(password, DEFAULT_COST)?)
    }
}