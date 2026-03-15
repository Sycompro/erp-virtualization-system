@echo off
echo 🎛️  Iniciando Panel de Administración ERP
echo ==========================================

REM Detener IIS si está ejecutándose en el puerto 8080
echo 🛑 Deteniendo servicios que puedan usar el puerto 8080...
net stop "World Wide Web Publishing Service" 2>nul
net stop "IIS Admin Service" 2>nul

REM Verificar que estamos en el directorio correcto
if not exist "server-railway" (
    echo ❌ Error: Ejecuta este script desde la raíz del proyecto
    pause
    exit /b 1
)

echo 📁 Verificando archivos...

REM Verificar que existe el panel HTML
if not exist "server-railway\static\index.html" (
    echo ❌ Error: No se encuentra el panel HTML
    echo Ejecutando desde: %CD%
    echo Buscando: server-railway\static\index.html
    pause
    exit /b 1
)

echo ✅ Panel HTML encontrado

REM Cambiar al directorio del servidor
cd server-railway

echo 🔧 Verificando dependencias de Rust...
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Error: Rust/Cargo no está instalado
    echo Instala Rust desde: https://rustup.rs/
    pause
    exit /b 1
)

echo ✅ Rust encontrado

REM Usar un puerto diferente para evitar conflictos con IIS
set PORT=3000

echo 🚀 Iniciando servidor en puerto %PORT%...
echo.
echo ==========================================
echo   Panel de Administración ERP
echo ==========================================
echo.
echo 🌐 Panel Web: http://localhost:%PORT%/admin/
echo 📊 API Health: http://localhost:%PORT%/health
echo 🔧 API Docs: http://localhost:%PORT%/api/
echo.
echo Funcionalidades:
echo   ✅ Configuración de visualización
echo   ✅ Gestión de contenedores ERP  
echo   ✅ Estadísticas en tiempo real
echo   ✅ Control de calidad de video
echo.
echo Presiona Ctrl+C para detener
echo ==========================================
echo.

REM Ejecutar el servidor con el puerto personalizado
cargo run --release

if %errorlevel% neq 0 (
    echo.
    echo ❌ Error iniciando el servidor
    echo.
    echo Posibles soluciones:
    echo 1. Verifica que PostgreSQL esté ejecutándose
    echo 2. Configura la variable DATABASE_URL
    echo 3. Revisa los logs de error arriba
    echo.
    pause
)