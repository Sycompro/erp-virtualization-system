#!/bin/bash
# Script de construcción para ERP Virtualization System

set -e

echo "🚀 Construyendo ERP Virtualization System..."

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para imprimir con colores
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias..."
    
    # Verificar Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust no está instalado. Instala desde: https://rustup.rs/"
        exit 1
    fi
    
    # Verificar Docker
    if ! command -v docker &> /dev/null; then
        print_error "Docker no está instalado. Instala desde: https://docs.docker.com/get-docker/"
        exit 1
    fi
    
    # Verificar Android SDK (opcional)
    if [ -z "$ANDROID_HOME" ]; then
        print_warning "ANDROID_HOME no está configurado. La app Android no se construirá."
    fi
    
    print_success "Dependencias verificadas"
}

# Construir servidor Rust
build_server() {
    print_status "Construyendo servidor Rust..."
    
    cd server
    
    # Verificar que Cargo.toml existe
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml no encontrado en el directorio server"
        exit 1
    fi
    
    # Construir en modo release
    cargo build --release
    
    if [ $? -eq 0 ]; then
        print_success "Servidor Rust construido exitosamente"
    else
        print_error "Error construyendo servidor Rust"
        exit 1
    fi
    
    cd ..
}

# Construir app Android
build_android() {
    if [ -z "$ANDROID_HOME" ]; then
        print_warning "Saltando construcción de Android (ANDROID_HOME no configurado)"
        return
    fi
    
    print_status "Construyendo app Android..."
    
    cd android
    
    # Verificar que build.gradle.kts existe
    if [ ! -f "build.gradle.kts" ]; then
        print_error "build.gradle.kts no encontrado en el directorio android"
        exit 1
    fi
    
    # Construir APK debug
    ./gradlew assembleDebug
    
    if [ $? -eq 0 ]; then
        print_success "App Android construida exitosamente"
        print_status "APK ubicado en: android/app/build/outputs/apk/debug/"
    else
        print_error "Error construyendo app Android"
        exit 1
    fi
    
    cd ..
}

# Construir imágenes Docker
build_docker_images() {
    print_status "Construyendo imágenes Docker..."
    
    # Construir imagen del servidor
    print_status "Construyendo imagen del servidor..."
    docker build -t erp-virtualization/server:latest -f server/Dockerfile server/
    
    if [ $? -eq 0 ]; then
        print_success "Imagen del servidor construida"
    else
        print_error "Error construyendo imagen del servidor"
        exit 1
    fi
    
    # Construir imágenes de aplicaciones (si existen)
    if [ -d "containers" ]; then
        print_status "Construyendo imágenes de aplicaciones..."
        
        for app_dir in containers/*/; do
            if [ -d "$app_dir" ]; then
                app_name=$(basename "$app_dir")
                print_status "Construyendo imagen para $app_name..."
                
                docker build -t "erp-virtualization/$app_name:latest" "$app_dir"
                
                if [ $? -eq 0 ]; then
                    print_success "Imagen $app_name construida"
                else
                    print_warning "Error construyendo imagen $app_name"
                fi
            fi
        done
    fi
}

# Ejecutar tests
run_tests() {
    print_status "Ejecutando tests..."
    
    # Tests del servidor Rust
    if [ -d "server" ]; then
        print_status "Ejecutando tests del servidor..."
        cd server
        cargo test
        cd ..
    fi
    
    # Tests de Android (si está disponible)
    if [ -n "$ANDROID_HOME" ] && [ -d "android" ]; then
        print_status "Ejecutando tests de Android..."
        cd android
        ./gradlew test
        cd ..
    fi
    
    print_success "Tests completados"
}

# Crear paquete de distribución
create_package() {
    print_status "Creando paquete de distribución..."
    
    PACKAGE_DIR="dist/erp-virtualization-$(date +%Y%m%d_%H%M%S)"
    mkdir -p "$PACKAGE_DIR"
    
    # Copiar binarios
    if [ -f "server/target/release/erp-virtualization-server" ]; then
        cp server/target/release/erp-virtualization-server "$PACKAGE_DIR/"
    fi
    
    # Copiar APK de Android
    if [ -f "android/app/build/outputs/apk/debug/app-debug.apk" ]; then
        cp android/app/build/outputs/apk/debug/app-debug.apk "$PACKAGE_DIR/erp-virtualization.apk"
    fi
    
    # Copiar archivos de configuración
    cp -r infrastructure/ "$PACKAGE_DIR/"
    
    # Crear README del paquete
    cat > "$PACKAGE_DIR/README.md" << EOF
# ERP Virtualization System

## Contenido del Paquete

- \`erp-virtualization-server\`: Servidor principal (Rust)
- \`erp-virtualization.apk\`: App Android
- \`infrastructure/\`: Archivos de configuración e instalación

## Instalación Rápida

1. Ejecutar: \`sudo bash infrastructure/setup-scripts/install.sh\`
2. Copiar \`infrastructure/docker-compose.yml\` a \`/opt/erp-simple/\`
3. Iniciar: \`sudo systemctl start erp-virtualization\`
4. Instalar APK en tablets Android

## Acceso

- Web: https://tu-servidor-ip
- Usuario: admin
- Password: admin123

EOF
    
    # Crear archivo ZIP
    cd dist
    zip -r "erp-virtualization-$(date +%Y%m%d_%H%M%S).zip" "$(basename "$PACKAGE_DIR")"
    cd ..
    
    print_success "Paquete creado en: $PACKAGE_DIR"
}

# Función principal
main() {
    echo "🏗️ ERP Virtualization Build System"
    echo "=================================="
    
    # Verificar argumentos
    BUILD_SERVER=true
    BUILD_ANDROID=true
    BUILD_DOCKER=false
    RUN_TESTS=false
    CREATE_PACKAGE=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --server-only)
                BUILD_ANDROID=false
                shift
                ;;
            --android-only)
                BUILD_SERVER=false
                shift
                ;;
            --with-docker)
                BUILD_DOCKER=true
                shift
                ;;
            --with-tests)
                RUN_TESTS=true
                shift
                ;;
            --package)
                CREATE_PACKAGE=true
                shift
                ;;
            --help)
                echo "Uso: $0 [opciones]"
                echo "Opciones:"
                echo "  --server-only    Solo construir servidor Rust"
                echo "  --android-only   Solo construir app Android"
                echo "  --with-docker    Construir imágenes Docker"
                echo "  --with-tests     Ejecutar tests"
                echo "  --package        Crear paquete de distribución"
                echo "  --help           Mostrar esta ayuda"
                exit 0
                ;;
            *)
                print_error "Opción desconocida: $1"
                exit 1
                ;;
        esac
    done
    
    # Ejecutar pasos de construcción
    check_dependencies
    
    if [ "$BUILD_SERVER" = true ]; then
        build_server
    fi
    
    if [ "$BUILD_ANDROID" = true ]; then
        build_android
    fi
    
    if [ "$BUILD_DOCKER" = true ]; then
        build_docker_images
    fi
    
    if [ "$RUN_TESTS" = true ]; then
        run_tests
    fi
    
    if [ "$CREATE_PACKAGE" = true ]; then
        create_package
    fi
    
    print_success "🎉 Construcción completada exitosamente!"
    
    echo ""
    echo "📋 Próximos pasos:"
    echo "1. Configurar servidor con: sudo bash infrastructure/setup-scripts/install.sh"
    echo "2. Instalar APK en tablets Android"
    echo "3. Acceder a: https://tu-servidor-ip"
    echo ""
}

# Ejecutar función principal con todos los argumentos
main "$@"