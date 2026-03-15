#!/bin/bash
# Script para desplegar en Railway

set -e

echo "🚂 Desplegando ERP Virtualization en Railway"
echo "============================================"

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

# Verificar Railway CLI
check_railway_cli() {
    if ! command -v railway &> /dev/null; then
        print_error "Railway CLI no está instalado"
        echo ""
        echo "Instala Railway CLI:"
        echo "npm install -g @railway/cli"
        echo "o"
        echo "curl -fsSL https://railway.app/install.sh | sh"
        exit 1
    fi
    
    print_success "Railway CLI encontrado: $(railway --version)"
}

# Login a Railway
railway_login() {
    print_status "Verificando autenticación Railway..."
    
    if ! railway whoami &> /dev/null; then
        print_status "Iniciando sesión en Railway..."
        railway login
    else
        print_success "Ya autenticado en Railway: $(railway whoami)"
    fi
}

# Crear proyecto Railway
create_railway_project() {
    print_status "Configurando proyecto Railway..."
    
    # Verificar si ya existe un proyecto vinculado
    if [ -f ".railway/project.json" ]; then
        print_status "Proyecto Railway ya existe"
        return
    fi
    
    # Crear nuevo proyecto
    print_status "Creando nuevo proyecto Railway..."
    railway init
    
    print_success "Proyecto Railway creado"
}

# Configurar variables de entorno
setup_environment() {
    print_status "Configurando variables de entorno..."
    
    # Generar JWT secret seguro
    JWT_SECRET=$(openssl rand -base64 32 2>/dev/null || echo "railway-jwt-secret-$(date +%s)")
    
    # Configurar variables
    railway variables set JWT_SECRET="$JWT_SECRET"
    railway variables set RUST_LOG="info"
    railway variables set RAILWAY_ENVIRONMENT="production"
    railway variables set PORT="8080"
    
    print_success "Variables de entorno configuradas"
}

# Agregar base de datos PostgreSQL
add_database() {
    print_status "Agregando base de datos PostgreSQL..."
    
    # Agregar plugin PostgreSQL
    railway add postgresql
    
    print_success "PostgreSQL agregado al proyecto"
}

# Agregar Redis
add_redis() {
    print_status "Agregando Redis..."
    
    # Agregar plugin Redis
    railway add redis
    
    print_success "Redis agregado al proyecto"
}

# Desplegar aplicación
deploy_app() {
    print_status "Desplegando aplicación..."
    
    # Asegurar que estamos en la rama correcta
    git add .
    git commit -m "Deploy to Railway: $(date)" || true
    
    # Desplegar
    railway up --detach
    
    print_success "Aplicación desplegada"
}

# Obtener información del despliegue
get_deployment_info() {
    print_status "Obteniendo información del despliegue..."
    
    # Obtener URL del servicio
    SERVICE_URL=$(railway status --json | grep -o '"url":"[^"]*"' | cut -d'"' -f4 2>/dev/null || echo "")
    
    if [ -n "$SERVICE_URL" ]; then
        print_success "Aplicación desplegada en: $SERVICE_URL"
        
        echo ""
        echo "📋 Información del Despliegue:"
        echo "=============================="
        echo "🌐 URL: $SERVICE_URL"
        echo "🔍 Health Check: $SERVICE_URL/health"
        echo "📊 API: $SERVICE_URL/applications/list"
        echo ""
        echo "🔧 Comandos útiles:"
        echo "  Ver logs:     railway logs"
        echo "  Ver status:   railway status"
        echo "  Abrir app:    railway open"
        echo "  Variables:    railway variables"
        echo ""
    else
        print_warning "No se pudo obtener la URL del servicio"
        echo "Usa 'railway status' para ver el estado del despliegue"
    fi
}

# Configurar dominio personalizado (opcional)
setup_custom_domain() {
    read -p "¿Quieres configurar un dominio personalizado? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        read -p "Ingresa tu dominio (ej: erp.tuempresa.com): " DOMAIN
        
        if [ -n "$DOMAIN" ]; then
            print_status "Configurando dominio personalizado: $DOMAIN"
            railway domain add "$DOMAIN"
            
            print_success "Dominio configurado"
            echo "📝 Configura tu DNS para apuntar a Railway:"
            echo "   CNAME: $DOMAIN -> railway.app"
        fi
    fi
}

# Función principal
main() {
    echo "🚂 Railway Deployment Script"
    echo "============================"
    
    check_railway_cli
    railway_login
    create_railway_project
    setup_environment
    add_database
    add_redis
    deploy_app
    get_deployment_info
    setup_custom_domain
    
    echo ""
    print_success "🎉 Despliegue completado en Railway!"
    
    echo ""
    echo "📱 Para conectar las tablets Android:"
    echo "1. Actualizar URL del servidor en la app"
    echo "2. Usar la URL de Railway como servidor"
    echo "3. Probar conexión con usuario: admin / admin123"
    echo ""
    echo "⚠️  Nota: Para aplicaciones ERP (SAP, Office), necesitarás"
    echo "   un servidor local debido a las limitaciones de Railway"
    echo "   con aplicaciones GUI y Docker privilegiado."
}

# Ejecutar script
main "$@"