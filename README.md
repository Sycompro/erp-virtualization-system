# 🏢 ERP Virtualization System - Arquitectura Híbrida

Sistema profesional de virtualización ERP para 5 tablets con arquitectura híbrida Railway + Servidor Local.

## 🎯 Arquitectura Híbrida

### Railway (Nube) - API + Autenticación
- ✅ **PostgreSQL**: Usuarios, sesiones, configuraciones
- ✅ **API REST**: Autenticación JWT, gestión de aplicaciones
- ✅ **Escalabilidad**: SSL automático, CDN global
- ✅ **Seguridad**: Zero Trust, tokens seguros

### Servidor Local - Streaming + Containers
- ✅ **Docker Containers**: SAP, Office, AutoCAD
- ✅ **WebRTC Streaming**: Ultra-baja latencia (<10ms)
- ✅ **Zero Tier VPN**: Conexión segura al ERP existente
- ✅ **TURN Server**: NAT traversal para WebRTC

## 🚀 Inicio Rápido

### 1. Configuración Automática
```bash
# Ejecutar script de configuración completa
./setup-hybrid.sh
```

### 2. Configuración Manual

#### Railway API (Nube)
```bash
cd server-railway
railway init
railway add postgresql
railway up
```

#### Servidor Local
```bash
# Iniciar servicios locales
docker-compose -f docker-compose.hybrid.yml up -d
```

### 3. App Android
```bash
cd android
./gradlew assembleDebug
# Instalar en tablets
```

## 📊 Flujo de Datos

```
Tablet → Railway API (auth) → Local Server (containers) → ERP (Zero Tier)
  ↓         ↓                      ↓                        ↓
JWT Token   User Data          WebRTC Stream           Business Data
```

## 🔧 Configuración

### Variables de Entorno (.env)
```bash
# Railway API
RAILWAY_API_URL=https://your-app.railway.app
JWT_SECRET=your-jwt-secret

# Servidor Local  
TURN_PASSWORD=your-turn-password
MAX_CONCURRENT_CONTAINERS=15

# Zero Tier (opcional)
ZEROTIER_NETWORK_ID=your-network-id
```

### URLs de Servicios
- **Railway API**: `https://your-app.railway.app`
- **Servidor Local**: `http://192.168.1.100:8080`
- **WebRTC Streaming**: `ws://192.168.1.100:8080/stream/connect`

## 📱 Aplicaciones Soportadas

| Aplicación | Protocolo | Puerto | GPU | Descripción |
|------------|-----------|--------|-----|-------------|
| SAP GUI | VNC | 5900+ | No | Sistema ERP empresarial |
| Office 365 | RDP | 3389+ | No | Word, Excel, PowerPoint |
| AutoCAD | RDP | 3390+ | Sí | Diseño CAD 2D/3D |
| LibreOffice | VNC | 5901+ | No | Suite ofimática libre |

## 🔐 Seguridad

### Autenticación
- ✅ **JWT Tokens**: 8 horas de duración
- ✅ **Biométrica**: FIDO2/WebAuthn (opcional)
- ✅ **Device ID**: Vinculación por dispositivo
- ✅ **Rate Limiting**: Protección contra ataques

### Comunicación
- ✅ **TLS 1.3**: Cifrado end-to-end
- ✅ **Certificate Pinning**: Prevención MITM
- ✅ **Zero Tier VPN**: Túnel seguro al ERP
- ✅ **WebRTC DTLS**: Streaming cifrado

## 🏗️ Estructura del Proyecto

```
├── server-railway/          # API Railway (Rust + Axum)
│   ├── src/
│   ├── Dockerfile
│   └── railway.json
├── server-local/           # Servidor Local (Rust + Docker)
│   ├── src/
│   └── Dockerfile
├── android/               # App Android (Kotlin + Compose)
│   ├── src/main/java/
│   └── build.gradle.kts
├── containers/           # Imágenes Docker ERP
│   ├── sap/
│   ├── office/
│   └── autocad/
├── database/            # Esquema PostgreSQL
├── infrastructure/     # Configuración K8s/Docker
└── deployment/         # Documentación despliegue
```

## 🛠️ Comandos Útiles

### Desarrollo
```bash
# Servidor Railway (desarrollo)
cd server-railway && cargo run

# Servidor Local (desarrollo)  
cd server-local && cargo run

# App Android (desarrollo)
cd android && ./gradlew installDebug
```

### Producción
```bash
# Ver logs del servidor local
docker-compose -f docker-compose.hybrid.yml logs -f

# Reiniciar servicios
docker-compose -f docker-compose.hybrid.yml restart

# Parar todo
docker-compose -f docker-compose.hybrid.yml down

# Monitoreo con Prometheus
docker-compose -f docker-compose.hybrid.yml --profile monitoring up -d
```

### Railway
```bash
# Ver logs de Railway
railway logs

# Ver estado
railway status

# Abrir en navegador
railway open

# Variables de entorno
railway variables
```

## 📊 Monitoreo

### Health Checks
- **Railway API**: `GET /health`
- **Servidor Local**: `GET /health`
- **Containers**: Estado automático via Docker

### Métricas
- **Prometheus**: `http://localhost:9090`
- **Grafana**: `http://localhost:3000` (opcional)
- **Docker Stats**: `docker stats`

## 🔍 Troubleshooting

### Problemas Comunes

#### 1. Error de conexión Railway
```bash
# Verificar URL en .env
curl https://your-app.railway.app/health

# Verificar variables Railway
railway variables
```

#### 2. Container no inicia
```bash
# Ver logs del container
docker logs container-name

# Verificar recursos
docker system df
```

#### 3. WebRTC no conecta
```bash
# Verificar TURN server
docker logs erp-coturn

# Test de conectividad
curl http://localhost:8080/health
```

## 📖 Documentación Adicional

- [Arquitectura Híbrida](deployment/hybrid-architecture.md)
- [Configuración Railway](deployment/railway-config.md)
- [Sistema de Diseño](docs/DESIGN_SYSTEM.md)
- [Guía de Inicio Rápido](QUICK_START.md)

## 🤝 Soporte

Para soporte técnico:
1. Revisar logs: `docker-compose logs`
2. Verificar health checks
3. Consultar documentación
4. Crear issue en el repositorio

## 📄 Licencia

Proyecto propietario - Todos los derechos reservados

---

**🎯 Objetivo**: Sistema ERP virtualizado profesional para 5 tablets con máxima seguridad y rendimiento.