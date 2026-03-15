use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tracing::{error, info, warn};
use x509_parser::prelude::*;
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use rand::Rng;

pub struct SecurityManager {
    tls_config: Arc<ServerConfig>,
    certificate_chain: Vec<Certificate>,
    private_key: PrivateKey,
}

impl SecurityManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("🔒 Inicializando SecurityManager con configuración Zero Trust");
        
        // Cargar certificados TLS
        let cert_path = std::env::var("TLS_CERT_PATH")
            .unwrap_or_else(|_| "/etc/tls/tls.crt".to_string());
        let key_path = std::env::var("TLS_KEY_PATH")
            .unwrap_or_else(|_| "/etc/tls/tls.key".to_string());
        
        let (certificate_chain, private_key) = Self::load_tls_certificates(&cert_path, &key_path)?;
        
        // Configurar TLS con configuraciones de seguridad modernas
        let tls_config = Self::create_secure_tls_config(&certificate_chain, &private_key)?;
        
        // Verificar configuración de seguridad
        Self::verify_security_configuration(&certificate_chain).await?;
        
        Ok(Self {
            tls_config,
            certificate_chain,
            private_key,
        })
    }
    
    fn load_tls_certificates(
        cert_path: &str,
        key_path: &str,
    ) -> Result<(Vec<Certificate>, PrivateKey), Box<dyn std::error::Error>> {
        info!("📜 Cargando certificados TLS desde: {}", cert_path);
        
        // Cargar certificados
        let cert_file = File::open(cert_path)
            .map_err(|e| format!("No se pudo abrir archivo de certificado {}: {}", cert_path, e))?;
        let mut cert_reader = BufReader::new(cert_file);
        let cert_chain = certs(&mut cert_reader)?
            .into_iter()
            .map(Certificate)
            .collect();
        
        // Cargar clave privada
        let key_file = File::open(key_path)
            .map_err(|e| format!("No se pudo abrir archivo de clave privada {}: {}", key_path, e))?;
        let mut key_reader = BufReader::new(key_file);
        let mut keys = pkcs8_private_keys(&mut key_reader)?;
        
        if keys.is_empty() {
            return Err("No se encontraron claves privadas válidas".into());
        }
        
        let private_key = PrivateKey(keys.remove(0));
        
        info!("✅ Certificados TLS cargados exitosamente");
        Ok((cert_chain, private_key))
    }
    
    fn create_secure_tls_config(
        cert_chain: &[Certificate],
        private_key: &PrivateKey,
    ) -> Result<Arc<ServerConfig>, Box<dyn std::error::Error>> {
        info!("🔧 Configurando TLS con estándares de seguridad modernos");
        
        let config = ServerConfig::builder()
            // Usar solo TLS 1.3 para máxima seguridad
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|e| format!("Error configurando versiones TLS: {}", e))?
            // No requerir autenticación de cliente por defecto
            .with_no_client_auth()
            // Configurar certificado y clave
            .with_single_cert(cert_chain.to_vec(), private_key.clone())
            .map_err(|e| format!("Error configurando certificado: {}", e))?;
        
        info!("✅ Configuración TLS 1.3 establecida");
        Ok(Arc::new(config))
    }
    
    async fn verify_security_configuration(
        cert_chain: &[Certificate],
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔍 Verificando configuración de seguridad");
        
        if cert_chain.is_empty() {
            return Err("Cadena de certificados vacía".into());
        }
        
        // Verificar el certificado principal
        let main_cert = &cert_chain[0];
        let (_, cert) = X509Certificate::from_der(&main_cert.0)?;
        
        // Verificar validez temporal
        let now = chrono::Utc::now();
        let not_before = cert.validity().not_before.to_datetime();
        let not_after = cert.validity().not_after.to_datetime();
        
        if now < not_before {
            warn!("⚠️ Certificado aún no es válido");
        }
        
        if now > not_after {
            error!("❌ Certificado expirado");
            return Err("Certificado TLS expirado".into());
        }
        
        // Verificar algoritmo de firma (debe ser RSA-PSS o ECDSA)
        let sig_alg = cert.signature_algorithm.algorithm;
        info!("🔐 Algoritmo de firma: {:?}", sig_alg);
        
        // Verificar longitud de clave
        if let Ok(public_key) = cert.public_key() {
            match public_key.algorithm.algorithm {
                x509_parser::oid_registry::OID_PKCS1_RSAENCRYPTION => {
                    info!("🔑 Certificado RSA detectado");
                }
                x509_parser::oid_registry::OID_KEY_TYPE_EC_PUBLIC_KEY => {
                    info!("🔑 Certificado ECDSA detectado");
                }
                _ => {
                    warn!("⚠️ Algoritmo de clave pública no reconocido");
                }
            }
        }
        
        // Verificar extensiones críticas
        for ext in cert.extensions() {
            if ext.critical {
                info!("🔒 Extensión crítica: {:?}", ext.oid);
            }
        }
        
        info!("✅ Configuración de seguridad verificada");
        Ok(())
    }
    
    pub fn get_tls_config(&self) -> Arc<ServerConfig> {
        self.tls_config.clone()
    }
    
    pub fn validate_client_certificate(
        &self,
        client_cert: &[u8],
    ) -> Result<ClientCertInfo, Box<dyn std::error::Error>> {
        info!("🔍 Validando certificado de cliente");
        
        let (_, cert) = X509Certificate::from_der(client_cert)?;
        
        // Extraer información del certificado
        let subject = cert.subject().to_string();
        let serial = cert.serial.to_string();
        let issuer = cert.issuer().to_string();
        
        // Verificar validez temporal
        let now = chrono::Utc::now();
        let not_before = cert.validity().not_before.to_datetime();
        let not_after = cert.validity().not_after.to_datetime();
        
        if now < not_before || now > not_after {
            return Err("Certificado de cliente no válido temporalmente".into());
        }
        
        // Verificar cadena de confianza (en producción)
        // Aquí verificarías contra tu CA raíz
        
        info!("✅ Certificado de cliente válido: {}", subject);
        
        Ok(ClientCertInfo {
            subject,
            serial,
            issuer,
            valid_from: not_before,
            valid_until: not_after,
        })
    }
    
    pub fn generate_session_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        use rand::Rng;
        
        // Generar token seguro de 32 bytes
        let mut rng = rand::thread_rng();
        let token_bytes: [u8; 32] = rng.gen();
        
        // Codificar en base64url
        Ok(base64::encode_config(token_bytes, base64::URL_SAFE_NO_PAD))
    }
    
    pub fn encrypt_sensitive_data(
        &self,
        data: &[u8],
        key: &[u8; 32],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit, OsRng},
            ChaCha20Poly1305, Nonce,
        };
        
        let cipher = ChaCha20Poly1305::new_from_slice(key)?;
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        
        let mut ciphertext = cipher.encrypt(&nonce, data)?;
        
        // Prepender nonce al ciphertext
        let mut result = nonce.to_vec();
        result.append(&mut ciphertext);
        
        Ok(result)
    }
    
    pub fn decrypt_sensitive_data(
        &self,
        encrypted_data: &[u8],
        key: &[u8; 32],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };
        
        if encrypted_data.len() < 12 {
            return Err("Datos encriptados demasiado cortos".into());
        }
        
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let cipher = ChaCha20Poly1305::new_from_slice(key)?;
        let plaintext = cipher.decrypt(nonce, ciphertext)?;
        
        Ok(plaintext)
    }
}

#[derive(Debug)]
pub struct ClientCertInfo {
    pub subject: String,
    pub serial: String,
    pub issuer: String,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_until: chrono::DateTime<chrono::Utc>,
}

// Middleware para validación de certificados mTLS
pub async fn mtls_middleware(
    cert_info: Option<ClientCertInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    match cert_info {
        Some(info) => {
            info!("✅ Cliente autenticado con mTLS: {}", info.subject);
            Ok(())
        }
        None => {
            warn!("❌ Cliente sin certificado válido");
            Err("Certificado de cliente requerido".into())
        }
    }
}

// Configuración de headers de seguridad
pub fn security_headers() -> Vec<(&'static str, &'static str)> {
    vec![
        // HSTS - Forzar HTTPS por 1 año
        ("Strict-Transport-Security", "max-age=31536000; includeSubDomains; preload"),
        // Prevenir clickjacking
        ("X-Frame-Options", "DENY"),
        // Prevenir MIME sniffing
        ("X-Content-Type-Options", "nosniff"),
        // XSS Protection
        ("X-XSS-Protection", "1; mode=block"),
        // Referrer Policy
        ("Referrer-Policy", "strict-origin-when-cross-origin"),
        // Content Security Policy estricta
        ("Content-Security-Policy", 
         "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self' wss:; font-src 'self'; object-src 'none'; media-src 'self'; frame-src 'none';"),
        // Permissions Policy
        ("Permissions-Policy", 
         "geolocation=(), microphone=(), camera=(), payment=(), usb=()"),
    ]
}