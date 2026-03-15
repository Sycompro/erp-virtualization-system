use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use uuid::Uuid;
use tracing::{error, info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub user_id: Uuid,
    pub roles: Vec<String>,
    pub session_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: Option<String>,
    pub biometric_data: Option<BiometricData>,
    pub device_id: String,
    pub challenge_response: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BiometricData {
    pub credential_id: String,
    pub authenticator_data: String,
    pub client_data_json: String,
    pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub user_info: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub roles: Vec<String>,
    pub last_login: chrono::DateTime<chrono::Utc>,
}

pub struct AuthService {
    jwt_secret: String,
    argon2: Argon2<'static>,
    active_sessions: HashMap<Uuid, SessionInfo>,
    // En producción, esto sería una base de datos
    users: HashMap<String, User>,
}

#[derive(Debug, Clone)]
struct SessionInfo {
    user_id: Uuid,
    device_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    last_activity: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
struct User {
    id: Uuid,
    username: String,
    password_hash: String,
    roles: Vec<String>,
    biometric_credentials: Vec<BiometricCredential>,
    is_active: bool,
}

#[derive(Debug, Clone)]
struct BiometricCredential {
    id: String,
    public_key: Vec<u8>,
    counter: u32,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl AuthService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("🔐 Inicializando AuthService con seguridad avanzada");
        
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "super-secret-key-change-in-production".to_string());
        
        let argon2 = Argon2::default();
        
        // Crear usuario demo (en producción vendría de la base de datos)
        let mut users = HashMap::new();
        let demo_user = User {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            password_hash: Self::hash_password(&argon2, "admin123")?,
            roles: vec!["admin".to_string(), "user".to_string()],
            biometric_credentials: vec![],
            is_active: true,
        };
        users.insert("admin".to_string(), demo_user);
        
        Ok(Self {
            jwt_secret,
            argon2,
            active_sessions: HashMap::new(),
            users,
        })
    }

    pub async fn authenticate(&self, payload: serde_json::Value) -> Result<AuthResponse, Box<dyn std::error::Error>> {
        let login_req: LoginRequest = serde_json::from_value(payload)?;
        
        info!("🔑 Intento de autenticación para usuario: {}", login_req.username);
        
        // Verificar si el usuario existe
        let user = self.users.get(&login_req.username)
            .ok_or("Usuario no encontrado")?;
        
        if !user.is_active {
            return Err("Usuario desactivado".into());
        }
        
        // Autenticación por biometría (FIDO2/WebAuthn)
        if let Some(biometric_data) = login_req.biometric_data {
            self.verify_biometric_authentication(user, &biometric_data).await?;
            info!("✅ Autenticación biométrica exitosa para: {}", user.username);
        }
        // Autenticación por contraseña (fallback)
        else if let Some(password) = login_req.password {
            self.verify_password_authentication(user, &password)?;
            info!("✅ Autenticación por contraseña exitosa para: {}", user.username);
        } else {
            return Err("Método de autenticación requerido".into());
        }
        
        // Crear sesión
        let session_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        
        // Generar tokens JWT
        let claims = Claims {
            sub: user.username.clone(),
            exp: (now + chrono::Duration::hours(1)).timestamp() as usize,
            iat: now.timestamp() as usize,
            user_id: user.id,
            roles: user.roles.clone(),
            session_id,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;
        
        // Generar refresh token (válido por 7 días)
        let refresh_claims = Claims {
            sub: user.username.clone(),
            exp: (now + chrono::Duration::days(7)).timestamp() as usize,
            iat: now.timestamp() as usize,
            user_id: user.id,
            roles: user.roles.clone(),
            session_id,
        };
        
        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;
        
        // Registrar sesión activa
        // En una implementación real, esto se guardaría en Redis o base de datos
        
        Ok(AuthResponse {
            token,
            refresh_token,
            expires_in: 3600, // 1 hora
            user_info: UserInfo {
                id: user.id,
                username: user.username.clone(),
                roles: user.roles.clone(),
                last_login: now,
            },
        })
    }
    
    async fn verify_biometric_authentication(
        &self,
        user: &User,
        biometric_data: &BiometricData,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔍 Verificando autenticación biométrica FIDO2/WebAuthn");
        
        // En una implementación real, aquí verificarías:
        // 1. La firma criptográfica usando la clave pública almacenada
        // 2. El challenge/response
        // 3. El contador para prevenir replay attacks
        // 4. Los datos del autenticador
        
        // Buscar credencial biométrica
        let credential = user.biometric_credentials
            .iter()
            .find(|c| c.id == biometric_data.credential_id)
            .ok_or("Credencial biométrica no encontrada")?;
        
        // Simular verificación (en producción usarías una librería WebAuthn)
        if biometric_data.signature.len() < 10 {
            return Err("Firma biométrica inválida".into());
        }
        
        info!("✅ Verificación biométrica exitosa");
        Ok(())
    }
    
    fn verify_password_authentication(
        &self,
        user: &User,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔍 Verificando autenticación por contraseña");
        
        let parsed_hash = PasswordHash::new(&user.password_hash)?;
        
        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => {
                info!("✅ Contraseña verificada correctamente");
                Ok(())
            }
            Err(_) => {
                warn!("❌ Contraseña incorrecta para usuario: {}", user.username);
                Err("Contraseña incorrecta".into())
            }
        }
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )?;
        
        Ok(token_data.claims)
    }
    
    fn hash_password(argon2: &Argon2, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }
    
    pub async fn register_biometric_credential(
        &mut self,
        username: &str,
        credential_data: BiometricCredential,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("📱 Registrando nueva credencial biométrica para: {}", username);
        
        if let Some(user) = self.users.get_mut(username) {
            user.biometric_credentials.push(credential_data);
            info!("✅ Credencial biométrica registrada exitosamente");
            Ok(())
        } else {
            Err("Usuario no encontrado".into())
        }
    }
    
    pub async fn revoke_session(&mut self, session_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚪 Revocando sesión: {}", session_id);
        self.active_sessions.remove(&session_id);
        Ok(())
    }
}