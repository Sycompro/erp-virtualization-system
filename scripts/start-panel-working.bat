@echo off
echo.
echo ========================================
echo   ERP Panel - Version Funcional
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

echo 🚀 Compilando y ejecutando servidor simple...
echo.
echo ==========================================
echo   Panel de Administracion ERP - ACTIVO
echo ==========================================
echo.
echo 🌐 Panel: http://localhost:3000/admin/
echo 📊 API:   http://localhost:3000/health
echo 🔧 Modo:  Desarrollo simple (sin base de datos)
echo.
echo ✅ Todas las funcionalidades del panel disponibles
echo ✅ API REST completamente funcional
echo ✅ Configuracion de visualizacion operativa
echo.
echo Presiona Ctrl+C para detener
echo ==========================================
echo.

cd server-railway
cargo run --bin erp-railway-api-simple --release

pause