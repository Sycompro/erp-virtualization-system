#!/bin/bash
# Script de configuración completa para Arquitectura Híbrida ERP

set -e

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_status() { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

echo "🏗️  Configuración Híbrida ERP Virtualization"
echo "============================================="
echo ""

# Verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias..."
    
    local missing_deps=()
    
    if ! command -v docker &> /dev/null; then
        missing_deps+=("docker")
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        missing_deps+=("docker-compose")
    fi
    
    if ! command -v curl &> /dev/null; then
        missing_deps+=("curl")
    fi
    
    if ! command -v git &> /dev/null; then
        missing_deps+=("git")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Dependencias faltantes: ${missing_deps[*]}"
        echo ""
        echo "Instala las dependencias faltantes:"
        echo "  Ubuntu/Debian: sudo apt update && sudo apt install docker.io docker-compose curl git"
        echo "  CentOS/RHEL: sudo yum install docker docker-compose curl git"
        echo "  macOS: brew install docker docker-compose curl git"
        exit 1
    fi
    
    print_success "Todas las dependencias están instaladas"
}

# Configurar Railway CLI
setup_railway() {
    print_status "Configurando Railway CLI..."
    
    if ! command -v railway &> /dev/null; then
        print_status "Instalando Railway CLI..."
        curl -fsSL https://railway.app/install.sh | sh
        export PATH="$HOME/.railway/bin:$PATH"
    fi
    
    print_success "Railway CLI configurado"
}

# Crear estructura de directorios
create_directories() {
    print_status "Creando estructura de directorios..."
    
    mkdir -p data/{containers,sap,office,autocad,prometheus}
    mkdir -p logs/{local,nginx}
    mkdir -p nginx
    mkdir -p monitoring
    
    print_success "Directorios creados"
}

# Generar configuración de Nginx local
create_nginx_config() {
    print_status "Creando configuración de Nginx local..."
    
    cat > nginx/local.conf << 'EOF'
events {
    worker_connections 1024;
}

http {
    upstream erp_local {
        server erp-local-server:8080;
    }
    
    server {
        listen 80;
        server_name localhost;
        
        # Proxy para API local
        location / {
            proxy_pass http://erp_local;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
        
        # WebSocket para streaming
        location /stream/ {
            proxy_pass http://erp_local;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
        }
    }
}
EOF
    
    print_success "Configuración de Nginx creada"
}

# Generar configuración de Prometheus
create_prometheus_config() {
    print_status "Creando configuración de Prometheus..."
    
    cat > monitoring/prometheus-local.yml << 'EOF'
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'erp-local-server'
    static_configs:
      - targets: ['erp-local-server:8080']
    metrics_path: '/metrics'
    scrape_interval: 10s
    
  - job_name: 'docker'
    static_configs:
      - targets: ['localhost:9323']
    scrape_interval: 30s
EOF
    
    print_success "Configuración de Prometheus creada"
}

# Configurar variables de entorno
setup_environment() {
    print_status "Configurando variables de entorno..."
    
    # Crear archivo .env si no existe
    if [ ! -f .env ]; then
        cat > .env << EOF
# Configuración Híbrida ERP Virtualization

# Railway API URL (actualizar después del despliegue)
RAILWAY_API_URL=https://your-railway-app.railway.app

# Configuración TURN Server
TURN_PASSWORD=$(openssl rand -base64 32)
DOMAIN=localhost

# Configuración de seguridad
JWT_SECRET=$(openssl rand -base64 32)

# Configuración de red
MAX_CONCURRENT_CONTAINERS=15
VNC_PORT_RANGE_START=5900
RDP_PORT_RANGE_START=3389

# Zero Tier (opcional)
ZEROTIER_NETWORK_ID=your_zerotier_network_id
EOF
        print_success "Archivo .env creado con configuración por defecto"
    else
        print_warning "Archivo .env ya existe, no se sobrescribió"
    fi
}

# Construir imágenes Docker
build_images() {
    print_status "Construyendo imágenes Docker..."
    
    # Construir servidor local
    print_status "Construyendo servidor local..."
    docker build -t erp-local-server ./server-local
    
    print_success "Imágenes Docker construidas"
}

# Desplegar en Railway
deploy_railway() {
    read -p "¿Quieres desplegar el API en Railway ahora? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Desplegando API en Railway..."
        
        cd server-railway
        
        # Inicializar proyecto Railway
        railway init
        
        # Agregar PostgreSQL
        railway add postgresql
        
        # Configurar variables de entorno
        railway variables set JWT_SECRET="$(openssl rand -base64 32)"
        railway variables set RUST_LOG="info"
        railway variables set RAILWAY_ENVIRONMENT="production"
        
        # Desplegar
        railway up --detach
        
        # Obtener URL
        sleep 10
        RAILWAY_URL=$(railway status --json | grep -o '"url":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "")
        
        if [ -n "$RAILWAY_URL" ]; then
            print_success "API desplegado en Railway: $RAILWAY_URL"
            
            # Actualizar .env con la URL real
            cd ..
            sed -i "s|RAILWAY_API_URL=.*|RAILWAY_API_URL=$RAILWAY_URL|" .env
            
            print_success "Archivo .env actualizado con URL de Railway"
        else
            print_warning "No se pudo obtener la URL de Railway automáticamente"
            print_warning "Actualiza manualmente RAILWAY_API_URL en .env"
        fi
        
        cd ..
    else
        print_warning "Despliegue en Railway omitido"
        print_warning "Recuerda actualizar RAILWAY_API_URL en .env cuando despliegues"
    fi
}

# Iniciar servidor local
start_local_server() {
    read -p "¿Quieres iniciar el servidor local ahora? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Iniciando servidor local..."
        
        # Cargar variables de entorno
        source .env
        
        # Iniciar servicios básicos
        docker-compose -f docker-compose.hybrid.yml up -d erp-local-server coturn
        
        print_success "Servidor local iniciado"
        
        # Mostrar estado
        echo ""
        print_status "Estado de los servicios:"
        docker-compose -f docker-compose.hybrid.yml ps
        
        echo ""
        print_status "URLs disponibles:"
        echo "  🏠 Servidor Local: http://localhost:8080"
        echo "  📊 Health Check: http://localhost:8080/health"
        echo "  🎮 TURN Server: localhost:3478"
        
        # Test de conectividad
        sleep 5
        if curl -f http://localhost:8080/health &> /dev/null; then
            print_success "✅ Servidor local funcionando correctamente"
        else
            print_warning "⚠️ Servidor local puede estar iniciando, verifica en unos minutos"
        fi
    else
        print_warning "Inicio del servidor local omitido"
    fi
}

# Mostrar información final
show_final_info() {
    echo ""
    print_success "🎉 Configuración híbrida completada!"
    echo ""
    echo "📋 Próximos pasos:"
    echo "=================="
    echo ""
    echo "1. 🚂 Railway API:"
    echo "   - URL: $(grep RAILWAY_API_URL .env | cut -d'=' -f2)"
    echo "   - Endpoints: /auth/login, /applications/list, /system/stats"
    echo ""
    echo "2. 🏠 Servidor Local:"
    echo "   - URL: http://localhost:8080"
    echo "   - Endpoints: /container/start, /webrtc/offer, /stream/connect"
    echo ""
    echo "3. 📱 App Android:"
    echo "   - Actualizar URLs en la configuración"
    echo "   - Compilar y probar en tablets"
    echo ""
    echo "4. 🔧 Comandos útiles:"
    echo "   - Ver logs: docker-compose -f docker-compose.hybrid.yml logs -f"
    echo "   - Parar servicios: docker-compose -f docker-compose.hybrid.yml down"
    echo "   - Reiniciar: docker-compose -f docker-compose.hybrid.yml restart"
    echo ""
    echo "5. 🔍 Monitoreo:"
    echo "   - Prometheus: docker-compose -f docker-compose.hybrid.yml --profile monitoring up -d"
    echo "   - Nginx: docker-compose -f docker-compose.hybrid.yml --profile nginx up -d"
    echo ""
    echo "📖 Documentación completa en: deployment/hybrid-architecture.md"
}

# Función principal
main() {
    check_dependencies
    setup_railway
    create_directories
    create_nginx_config
    create_prometheus_config
    setup_environment
    build_images
    deploy_railway
    start_local_server
    show_final_info
}

# Ejecutar script
main "$@"