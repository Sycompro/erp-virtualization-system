#!/bin/bash

echo ""
echo "========================================"
echo "   ERP Panel - Inicio Rápido"
echo "========================================"
echo ""

echo "🔧 Configurando variables de entorno..."
export PORT=3000
export RUST_LOG=info

echo "📁 Verificando directorio..."
if [ ! -d "server-railway" ]; then
    echo "❌ Error: Directorio server-railway no encontrado"
    echo "💡 Ejecuta este script desde la raíz del proyecto"
    exit 1
fi

echo "🚀 Iniciando servidor Railway en puerto 3000..."
echo ""
echo "=========================================="
echo "   Panel de Administración ERP - ACTIVO"
echo "=========================================="
echo ""
echo "🌐 Panel: http://localhost:3000/admin/"
echo "📊 API:   http://localhost:3000/health"
echo "🔧 Modo:  Desarrollo (sin base de datos)"
echo ""
echo "Presiona Ctrl+C para detener"
echo "=========================================="
echo ""

cd server-railway
cargo run --release