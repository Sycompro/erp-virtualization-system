#!/bin/bash
# Script de configuración rápida para desarrollo

set -e

echo "🚀 Configuración de Desarrollo ERP Virtualization"
echo "================================================"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Verificar sistema operativo
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    print_warning "Detectado Windows. Usando comandos compatibles..."
    IS_WINDOWS=true
else
    IS_WINDOWS=false
fi

# Función para verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias..."
    
    # Verificar Docker
    if ! command -v docker &> /dev/null; then
        print_error "Docker no está instalado"
        echo "Instala Docker desde: https://docs.docker.com/get-docker/"
        exit 1
    fi
    
    # Verificar Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose no está instalado"
        echo "Instala Docker Compose desde: https://docs.docker.com/compose/install/"
        exit 1
    fi
    
    # Verificar Rust (opcional para desarrollo)
    if command -v cargo &> /dev/null; then
        print_success "Rust encontrado: $(cargo --version)"
    else
        print_warning "Rust no encontrado. Instala desde: https://rustup.rs/"
    fi
    
    print_success "Dependencias verificadas"
}

# Crear estructura de directorios
create_directories() {
    print_status "Creando estructura de directorios..."
    
    mkdir -p {data/{postgres,redis,user-files,sap,office,autocad},logs/{nginx,erp},nginx/ssl,monitoring}
    
    # Crear directorios de datos específicos
    mkdir -p data/user-files/{user1,user2,user3,user4,user5}
    mkdir -p data/sap/{profiles,data,logs}
    mkdir -p data/office/{documents,templates}
    mkdir -p data/autocad/{projects,libraries}
    
    print_success "Estructura de directorios creada"
}

# Generar certificados SSL de desarrollo
generate_ssl_certs() {
    print_status "Generando certificados SSL para desarrollo..."
    
    if [ ! -f "nginx/ssl/server.crt" ]; then
        # Crear certificado auto-firmado
        openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
            -keyout nginx/ssl/server.key \
            -out nginx/ssl/server.crt \
            -subj "/C=US/ST=Dev/L=Dev/O=ERP-Dev/CN=localhost" \
            2>/dev/null || {
                print_warning "OpenSSL no disponible. Creando certificados dummy..."
                echo "dummy-cert" > nginx/ssl/server.crt
                echo "dummy-key" > nginx/ssl/server.key
            }
        
        print_success "Certificados SSL generados"
    else
        print_status "Certificados SSL ya existen"
    fi
}

# Construir imágenes Docker
build_images() {
    print_status "Construyendo imágenes Docker..."
    
    # Construir imagen SAP
    if [ -d "containers/sap" ]; then
        print_status "Construyendo imagen SAP..."
        docker build -t erp-virtualization/sap-gui:latest containers/sap/
        if [ $? -eq 0 ]; then
            print_success "Imagen SAP construida"
        else
            print_error "Error construyendo imagen SAP"
        fi
    fi
    
    # Construir imagen Office
    if [ -d "containers/office" ]; then
        print_status "Construyendo imagen Office..."
        docker build -t erp-virtualization/office:latest containers/office/
        if [ $? -eq 0 ]; then
            print_success "Imagen Office construida"
        else
            print_error "Error construyendo imagen Office"
        fi
    fi
    
    # Construir servidor Rust (si existe)
    if [ -f "server/Dockerfile" ]; then
        print_status "Construyendo servidor Rust..."
        docker build -t erp-virtualization/server:latest server/
        if [ $? -eq 0 ]; then
            print_success "Servidor Rust construido"
        else
            print_warning "Error construyendo servidor Rust (continuando...)"
        fi
    fi
}

# Crear archivo docker-compose para desarrollo
create_dev_compose() {
    print_status "Creando docker-compose para desarrollo..."
    
    cat > docker-compose.dev.yml << 'EOF'
version: '3.8'

services:
  # Base de datos PostgreSQL
  postgres:
    image: postgres:15-alpine
    container_name: erp-postgres-dev
    restart: unless-stopped
    environment:
      - POSTGRES_DB=erp_db
      - POSTGRES_USER=erp_user
      - POSTGRES_PASSWORD=dev_password
    ports:
      - "5432:5432"
    volumes:
      - ./data/postgres:/var/lib/postgresql/data
    networks:
      - erp-dev-network

  # Redis para cache
  redis:
    image: redis:7-alpine
    container_name: erp-redis-dev
    restart: unless-stopped
    command: redis-server --appendonly yes --requirepass dev_redis_pass
    ports:
      - "6379:6379"
    volumes:
      - ./data/redis:/data
    networks:
      - erp-dev-network

  # Contenedor SAP (desarrollo)
  sap-dev:
    image: erp-virtualization/sap-gui:latest
    container_name: sap-gui-dev
    restart: "no"
    ports:
      - "5900:5900"  # VNC
      - "6080:6080"  # noVNC web
    environment:
      - DISPLAY=:1
      - VNC_RESOLUTION=1920x1080
      - VNC_PASSWORD=dev_sap_vnc
    volumes:
      - ./data/sap:/home/sapuser/sap-data
    networks:
      - erp-dev-network
    profiles:
      - sap

  # Contenedor Office (desarrollo)
  office-dev:
    image: erp-virtualization/office:latest
    container_name: office-suite-dev
    restart: "no"
    ports:
      - "3389:3389"  # RDP
    environment:
      - RDP_USER=dev_user
      - RDP_PASSWORD=dev_office_123
    volumes:
      - ./data/office:/home/officeuser/Documents
    networks:
      - erp-dev-network
    profiles:
      - office

  # Nginx para desarrollo
  nginx-dev:
    image: nginx:alpine
    container_name: erp-nginx-dev
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/ssl:/etc/nginx/ssl:ro
      - ./logs/nginx:/var/log/nginx
    networks:
      - erp-dev-network

networks:
  erp-dev-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.30.0.0/16

volumes:
  postgres_dev_data:
  redis_dev_data:
EOF

    print_success "docker-compose.dev.yml creado"
}

# Crear scripts de utilidad para desarrollo
create_dev_scripts() {
    print_status "Creando scripts de desarrollo..."
    
    # Script para iniciar desarrollo
    cat > start-dev.sh << 'EOF'
#!/bin/bash
echo "🚀 Iniciando entorno de desarrollo..."

# Iniciar servicios base
docker-compose -f docker-compose.dev.yml up -d postgres redis nginx-dev

echo "✅ Servicios base iniciados"
echo "📊 PostgreSQL: localhost:5432 (erp_user/dev_password)"
echo "🔴 Redis: localhost:6379 (password: dev_redis_pass)"
echo "🌐 Nginx: http://localhost"

echo ""
echo "Para iniciar aplicaciones específicas:"
echo "  SAP:    docker-compose -f docker-compose.dev.yml --profile sap up -d"
echo "  Office: docker-compose -f docker-compose.dev.yml --profile office up -d"
EOF

    # Script para detener desarrollo
    cat > stop-dev.sh << 'EOF'
#!/bin/bash
echo "🛑 Deteniendo entorno de desarrollo..."
docker-compose -f docker-compose.dev.yml down
echo "✅ Entorno detenido"
EOF

    # Script para logs
    cat > logs-dev.sh << 'EOF'
#!/bin/bash
if [ -z "$1" ]; then
    echo "Uso: $0 [servicio]"
    echo "Servicios disponibles: postgres, redis, sap-dev, office-dev, nginx-dev"
    exit 1
fi

docker-compose -f docker-compose.dev.yml logs -f $1
EOF

    # Script para limpiar desarrollo
    cat > clean-dev.sh << 'EOF'
#!/bin/bash
echo "🧹 Limpiando entorno de desarrollo..."
docker-compose -f docker-compose.dev.yml down -v
docker system prune -f
echo "✅ Entorno limpiado"
EOF

    # Hacer scripts ejecutables (si no es Windows)
    if [ "$IS_WINDOWS" = false ]; then
        chmod +x start-dev.sh stop-dev.sh logs-dev.sh clean-dev.sh
    fi
    
    print_success "Scripts de desarrollo creados"
}

# Crear archivo de configuración de desarrollo
create_dev_config() {
    print_status "Creando configuración de desarrollo..."
    
    cat > .env.dev << 'EOF'
# Configuración de desarrollo ERP Virtualization

# Base de datos
DATABASE_URL=postgresql://erp_user:dev_password@localhost:5432/erp_db
REDIS_URL=redis://:dev_redis_pass@localhost:6379

# Servidor
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=debug

# JWT
JWT_SECRET=dev-jwt-secret-change-in-production

# Aplicaciones
SAP_VNC_PASSWORD=dev_sap_vnc
OFFICE_RDP_PASSWORD=dev_office_123

# Desarrollo
DEV_MODE=true
SKIP_AUTH=false
MOCK_BIOMETRIC=true
EOF

    print_success "Configuración de desarrollo creada"
}

# Mostrar información de desarrollo
show_dev_info() {
    print_success "🎉 Entorno de desarrollo configurado!"
    
    echo ""
    echo "📋 Información del entorno:"
    echo "=========================="
    echo "📁 Datos:           ./data/"
    echo "📝 Logs:            ./logs/"
    echo "🔐 SSL:             ./nginx/ssl/"
    echo "⚙️  Config:          .env.dev"
    echo ""
    echo "🚀 Para iniciar desarrollo:"
    echo "  ./start-dev.sh"
    echo ""
    echo "🛑 Para detener:"
    echo "  ./stop-dev.sh"
    echo ""
    echo "📊 Para ver logs:"
    echo "  ./logs-dev.sh [servicio]"
    echo ""
    echo "🧹 Para limpiar:"
    echo "  ./clean-dev.sh"
    echo ""
    echo "🌐 URLs de desarrollo:"
    echo "  Web:     http://localhost"
    echo "  SAP VNC: http://localhost:6080"
    echo "  Office:  RDP localhost:3389"
    echo ""
    echo "👤 Credenciales de desarrollo:"
    echo "  PostgreSQL: erp_user / dev_password"
    echo "  Redis:      dev_redis_pass"
    echo "  SAP VNC:    dev_sap_vnc"
    echo "  Office RDP: dev_user / dev_office_123"
}

# Función principal
main() {
    echo "🏗️ Configurando entorno de desarrollo..."
    
    check_dependencies
    create_directories
    generate_ssl_certs
    build_images
    create_dev_compose
    create_dev_scripts
    create_dev_config
    show_dev_info
}

# Ejecutar configuración
main "$@"