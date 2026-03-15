#!/bin/bash

echo "🎛️  Desplegando Panel de Administración ERP"
echo "=========================================="

# Verificar que estamos en el directorio correcto
if [ ! -d "server-railway" ]; then
    echo "❌ Error: Ejecuta este script desde la raíz del proyecto"
    exit 1
fi

echo "📁 Preparando archivos..."

# Crear directorio static si no existe
mkdir -p server-railway/static

# Copiar el panel HTML si no existe
if [ ! -f "server-railway/static/index.html" ]; then
    echo "📄 Copiando panel HTML..."
    cp server-railway/static/index.html server-railway/static/index.html 2>/dev/null || echo "⚠️  Panel HTML ya existe"
fi

echo "🔧 Compilando servidor Railway..."
cd server-railway

# Compilar el proyecto
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilación exitosa"
    
    echo "🚀 Iniciando servidor..."
    echo ""
    echo "Panel de Administración disponible en:"
    echo "  🌐 http://localhost:8080/admin/"
    echo "  📊 API Health: http://localhost:8080/health"
    echo ""
    echo "Funcionalidades disponibles:"
    echo "  ✅ Configuración de visualización (como Parallels)"
    echo "  ✅ Gestión de contenedores ERP"
    echo "  ✅ Estadísticas en tiempo real"
    echo "  ✅ Control de calidad de video"
    echo "  ✅ Configuración de red y rendimiento"
    echo ""
    echo "Presiona Ctrl+C para detener el servidor"
    echo ""
    
    # Ejecutar el servidor
    cargo run --release
else
    echo "❌ Error en la compilación"
    echo "Revisa los errores arriba"
    exit 1
fi