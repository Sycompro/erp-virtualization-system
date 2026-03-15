#!/bin/bash
# Script para desplegar ERP Railway API via Git Push

set -e

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_status() { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

echo "🚂 Despliegue Railway ERP API"
echo "============================="

# Verificar que estamos en el directorio correcto
if [ ! -f "server-railway/Cargo.toml" ]; then
    print_error "No se encuentra server-railway/Cargo.toml"
    print_error "Ejecuta este script desde la raíz del proyecto"
    exit 1
fi

# Verificar Railway CLI
if ! command -v railway &> /dev/null; then
    print_error "Railway CLI no está instalado"
    echo ""
    echo "Instala Railway CLI:"
    echo "npm install -g @railway/cli"
    echo "o"
    echo "curl -fsSL https://railway.app/install.sh | sh"
    exit 1
fi

# Verificar login Railway
print_status "Verificando autenticación Railway..."
if ! railway whoami &> /dev/null; then
    print_status "Iniciando sesión en Railway..."
    railway login
else
    print_success "Autenticado en Railway: $(railway whoami)"
fi

# Verificar si hay un proyecto Railway vinculado
if [ ! -f ".railway/project.json" ]; then
    print_warning "No hay proyecto Railway vinculado"
    read -p "¿Quieres crear un nuevo proyecto? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Creando nuevo proyecto Railway..."
        railway init
    else
        print_error "Necesitas un proyecto Railway para continuar"
        exit 1
    fi
fi

# Configurar variables de entorno esenciales
print_status "Configurando variables de entorno..."

# Generar JWT secret si no existe
JWT_SECRET=$(railway variables get JWT_SECRET 2>/dev/null || echo "")
if [ -z "$JWT_SECRET" ]; then
    JWT_SECRET=$(openssl rand -base64 32 2>/dev/null || echo "railway-jwt-secret-$(date +%s)")
    railway variables set JWT_SECRET="$JWT_SECRET"
    print_success "JWT_SECRET configurado"
fi

# Configurar otras variables
railway variables set RUST_LOG="info" 2>/dev/null || true
railway variables set RAILWAY_ENVIRONMENT="production" 2>/dev/null || true

# Configurar URL del servidor local (preguntarle al usuario)
LOCAL_SERVER_URL=$(railway variables get LOCAL_SERVER_URL 2>/dev/null || echo "")
if [ -z "$LOCAL_SERVER_URL" ]; then
    echo ""
    print_status "Configuración del servidor local:"
    read -p "Ingresa la IP de tu servidor local (ej: 192.168.1.100): " LOCAL_IP
    
    if [ -n "$LOCAL_IP" ]; then
        LOCAL_SERVER_URL="ws://${LOCAL_IP}:8080"
        railway variables set LOCAL_SERVER_URL="$LOCAL_SERVER_URL"
        print_success "LOCAL_SERVER_URL configurado: $LOCAL_SERVER_URL"
    else
        print_warning "LOCAL_SERVER_URL no configurado, usando valor por defecto"
        railway variables set LOCAL_SERVER_URL="ws://192.168.1.100:8080"
    fi
fi

# Agregar PostgreSQL si no existe
print_status "Verificando base de datos PostgreSQL..."
if ! railway variables get DATABASE_URL &> /dev/null; then
    print_status "Agregando PostgreSQL..."
    railway add postgresql
    print_success "PostgreSQL agregado"
else
    print_success "PostgreSQL ya configurado"
fi

# Preparar archivos para despliegue
print_status "Preparando archivos para despliegue..."

# Crear .railwayignore si no existe
if [ ! -f ".railwayignore" ]; then
    cat > .railwayignore << 'EOF'
# Ignorar archivos innecesarios para Railway
android/
server-local/
containers/
infrastructure/
docs/
*.md
.git/
.env
target/
node_modules/
EOF
    print_success ".railwayignore creado"
fi

# Verificar que el railway.json esté en la raíz para Railway
if [ ! -f "railway.json" ]; then
    cp server-railway/railway.json ./railway.json
    print_success "railway.json copiado a la raíz"
fi

# Verificar que el Dockerfile esté en la raíz para Railway
if [ ! -f "Dockerfile" ]; then
    # Crear Dockerfile en la raíz que apunte a server-railway
    cat > Dockerfile << 'EOF'
# Dockerfile para Railway - ERP API Service
FROM rust:1.75-slim as builder

# Instalar dependencias de compilación
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar manifiestos de Cargo desde server-railway
COPY server-railway/Cargo.toml server-railway/Cargo.lock ./

# Crear src dummy para cachear dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Construir dependencias
RUN cargo build --release && rm -rf src

# Copiar código fuente real desde server-railway
COPY server-railway/src ./src

# Construir aplicación final
RUN cargo build --release

# Etapa de runtime
FROM debian:bookworm-slim

# Instalar dependencias de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root
RUN useradd -r -s /bin/false -m -d /app railway-user

# Copiar binario
COPY --from=builder /app/target/release/erp-railway-api /app/
RUN chown railway-user:railway-user /app/erp-railway-api

# Cambiar a usuario no-root
USER railway-user
WORKDIR /app

# Railway automáticamente asigna PORT
EXPOSE $PORT

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:$PORT/health || exit 1

# Variables de entorno
ENV RUST_LOG=info
ENV RAILWAY_ENVIRONMENT=production

# Comando de inicio
CMD ["./erp-railway-api"]
EOF
    print_success "Dockerfile creado en la raíz"
fi

# Commit y push
print_status "Preparando commit para despliegue..."

# Agregar archivos al git
git add .
git add -f railway.json Dockerfile .railwayignore 2>/dev/null || true

# Commit
COMMIT_MSG="Deploy ERP Railway API - $(date '+%Y-%m-%d %H:%M:%S')"
git commit -m "$COMMIT_MSG" || print_warning "No hay cambios para commit"

# Push y desplegar
print_status "Desplegando en Railway..."
git push origin main || git push origin master || {
    print_error "Error en git push. Verifica tu repositorio remoto"
    exit 1
}

# Esperar un momento para que Railway procese
print_status "Esperando que Railway procese el despliegue..."
sleep 10

# Obtener información del despliegue
print_status "Obteniendo información del despliegue..."

# Intentar obtener la URL del servicio
SERVICE_URL=""
for i in {1..5}; do
    SERVICE_URL=$(railway status --json 2>/dev/null | grep -o '"url":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "")
    if [ -n "$SERVICE_URL" ]; then
        break
    fi
    print_status "Esperando URL del servicio... (intento $i/5)"
    sleep 5
done

# Mostrar información final
echo ""
print_success "🎉 Despliegue iniciado en Railway!"
echo ""

if [ -n "$SERVICE_URL" ]; then
    print_success "🌐 URL del servicio: $SERVICE_URL"
    
    echo ""
    echo "📋 Endpoints disponibles:"
    echo "  🔐 Login: POST $SERVICE_URL/auth/login"
    echo "  📱 Apps: GET $SERVICE_URL/applications/list"
    echo "  ❤️ Health: GET $SERVICE_URL/health"
    echo ""
    
    # Test de conectividad
    print_status "Probando conectividad..."
    sleep 15  # Esperar que el servicio esté listo
    
    if curl -f "$SERVICE_URL/health" &> /dev/null; then
        print_success "✅ Servicio funcionando correctamente!"
    else
        print_warning "⚠️ Servicio aún iniciando, verifica en unos minutos"
    fi
else
    print_warning "No se pudo obtener la URL automáticamente"
    echo "Usa 'railway status' para ver el estado del despliegue"
fi

echo ""
echo "🔧 Comandos útiles:"
echo "  Ver logs:     railway logs"
echo "  Ver status:   railway status"
echo "  Abrir app:    railway open"
echo "  Variables:    railway variables"
echo ""

print_success "Despliegue completado!"