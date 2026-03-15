@echo off
echo.
echo ========================================
echo   ERP Panel - Inicio Rapido
echo ========================================
echo.

echo 🔧 Configurando variables de entorno...
set PORT=3000
set RUST_LOG=info

echo 📁 Verificando directorio...
if not exist "server-railway" (
    echo ❌ Error: Directorio server-railway no encontrado
    echo 💡 Ejecuta este script desde la raiz del proyecto
    pause
    exit /b 1
)

echo 🚀 Iniciando servidor Railway en puerto 3000...
echo.
echo ==========================================
echo   Panel de Administracion ERP - ACTIVO
echo ==========================================
echo.
echo 🌐 Panel: http://localhost:3000/admin/
echo 📊 API:   http://localhost:3000/health
echo 🔧 Modo:  Desarrollo (sin base de datos)
echo.
echo Presiona Ctrl+C para detener
echo ==========================================
echo.

cd server-railway
cargo run --release

pause