#!/bin/bash

echo "🏠 Configurando Servidor Local ERP Virtualization"

# Verificar Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker no está instalado. Instalando..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo usermod -aG docker $USER
    echo "✅ Docker instalado. Reinicia la sesión para aplicar cambios."
fi

# Verificar Docker Compose
if ! command -v docker-compose &> /dev/null; then
    echo "📦 Instalando Docker Compose..."
    sudo curl -L "https://github.com/docker/compose/releases/download/v2.24.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    sudo chmod +x /usr/local/bin/docker-compose
fi

# Crear directorio de configuración
mkdir -p ~/.erp-local
cp server-local/.env.example ~/.erp-local/.env

echo "⚙️ Configuración creada en ~/.erp-local/.env"
echo "📝 Edita el archivo para personalizar la configuración"

# Verificar Rust
if ! command -v cargo &> /dev/null; then
    echo "🦀 Instalando Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Compilar servidor local
echo "🔨 Compilando servidor local..."
cd server-local
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Servidor local compilado exitosamente"
    echo ""
    echo "🚀 Para iniciar el servidor local:"
    echo "   cd server-local"
    echo "   cargo run --release"
    echo ""
    echo "🔧 Para configurar variables de entorno:"
    echo "   cp .env.example .env"
    echo "   nano .env"
else
    echo "❌ Error compilando servidor local"
    exit 1
fi

echo ""
echo "📋 Próximos pasos:"
echo "1. Configurar variables de entorno en server-local/.env"
echo "2. Crear imágenes Docker de las aplicaciones"
echo "3. Iniciar el servidor local"
echo "4. Probar conexión desde la app Android"