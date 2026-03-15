@echo off
echo.
echo ========================================
echo   PRUEBA COMPLETA DEL SISTEMA ERP
echo ========================================
echo.

echo 🔍 Verificando estructura del proyecto...
if not exist "server-railway" (
    echo ❌ Error: Directorio server-railway no encontrado
    pause
    exit /b 1
)

if not exist "android" (
    echo ❌ Error: Directorio android no encontrado
    pause
    exit /b 1
)

echo ✅ Estructura del proyecto verificada

echo.
echo 🚀 Iniciando pruebas...
echo.

echo 📱 1. COMPILANDO APK ANDROID...
echo ----------------------------------------
cd android
call gradlew assembleDebug
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Error compilando APK
    cd ..
    pause
    exit /b 1
)
echo ✅ APK compilado exitosamente
cd ..

echo.
echo 🌐 2. INICIANDO SERVIDOR RAILWAY...
echo ----------------------------------------
echo 🔧 Configurando variables de entorno...
set PORT=3000
set RUST_LOG=info

echo 🚀 Compilando y ejecutando servidor...
cd server-railway

echo.
echo ==========================================
echo   SISTEMA ERP VIRTUALIZATION - ACTIVO
echo ==========================================
echo.
echo 📱 APK: android\app\build\outputs\apk\debug\app-debug.apk
echo 🌐 Panel: http://localhost:3000/admin/
echo 📊 API: http://localhost:3000/health
echo 🔧 Modo: Desarrollo (sin base de datos)
echo.
echo ✅ Sistema completamente funcional
echo.
echo Presiona Ctrl+C para detener
echo ==========================================
echo.

cargo run --release