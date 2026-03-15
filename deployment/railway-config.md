# Configuración Railway para ERP Virtualization

## 🚂 Arquitectura Híbrida Recomendada

### Opción 1: Railway para API + Servidor Local para Apps
```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Railway       │    │   Servidor Local │    │   5 Tablets     │
│   (API + Auth)  │◄──►│   (Apps + Stream)│◄──►│   (Android)     │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

**Beneficios:**
- ✅ API segura en Railway con SSL automático
- ✅ Aplicaciones ERP en servidor local (baja latencia)
- ✅ Datos sensibles nunca salen de tu red
- ✅ Fácil mantenimiento y actualizaciones

### Opción 2: Railway Solo para Desarrollo
```
Desarrollo:  Railway (rápido, fácil)
Producción:  Servidor propio (seguro, rápido)
```

## 🔧 Configuración Railway

### 1. railway.json
```json
{
  "build": {
    "builder": "DOCKERFILE",
    "dockerfilePath": "Dockerfile.railway"
  },
  "deploy": {
    "numReplicas": 1,
    "sleepApplication": false,
    "restartPolicyType": "ON_FAILURE"
  }
}
```

### 2. Dockerfile.railway
```dockerfile
# Dockerfile optimizado para Railway
FROM rust:1.75-slim as builder

WORKDIR /app
COPY server/Cargo.toml server/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY server/src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/erp-virtualization-server /app/
WORKDIR /app

# Railway automáticamente expone el puerto
EXPOSE $PORT

CMD ["./erp-virtualization-server"]
```

### 3. Variables de Entorno Railway
```bash
# En Railway Dashboard
DATABASE_URL=postgresql://user:pass@railway-postgres:5432/erp_db
REDIS_URL=redis://railway-redis:6379
JWT_SECRET=railway-super-secure-jwt-key
PORT=8080
RUST_LOG=info
RAILWAY_ENVIRONMENT=production
```

## 🔒 Configuración de Seguridad Railway

### 1. Networking Privado
```yaml
# railway.toml
[build]
builder = "dockerfile"
dockerfilePath = "Dockerfile.railway"

[deploy]
healthcheckPath = "/health"
healthcheckTimeout = 300
restartPolicyType = "on_failure"

# Red privada solo para servicios internos
[networking]
private = true
```

### 2. Secrets Management
```bash
# Usar Railway CLI para secrets seguros
railway login
railway link
railway variables set JWT_SECRET="$(openssl rand -base64 32)"
railway variables set DATABASE_PASSWORD="$(openssl rand -base64 24)"
```

## 💰 Análisis de Costos

### Railway (Mensual)
```
Starter Plan:     $5/mes  (512MB RAM, 1GB storage)
Developer Plan:   $20/mes (8GB RAM, 100GB storage)
Team Plan:        $99/mes (32GB RAM, 500GB storage)

Para 5 tablets: ~$20-99/mes
```

### Servidor Propio (Una vez + Mensual)
```
Hardware:         $3,500 (una vez)
Electricidad:     $50/mes
Internet:         $100/mes
Mantenimiento:    $200/mes

Total: $350/mes después de inversión inicial
```

## 🎯 Recomendación Final

### Para Desarrollo y Pruebas: Railway ✅
```bash
# Despliegue súper rápido
git push origin main
# Railway automáticamente despliega
```

### Para Producción: Servidor Propio ✅
```bash
# Máxima seguridad y rendimiento
sudo bash infrastructure/setup-scripts/install.sh
```

### Arquitectura Híbrida (Ideal): 🚀
```
Railway:         API, autenticación, gestión
Servidor Local:  Aplicaciones ERP, streaming
```