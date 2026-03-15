@echo off
echo 🎛️  Abriendo Panel de Administración ERP
echo ========================================

REM Cambiar al directorio del proyecto
cd /d "C:\Users\Administrador\Desktop\APP VIRTUAL"

echo 📍 Directorio actual: %CD%

REM Verificar que estamos en el lugar correcto
if not exist "server-railway\static\index.html" (
    echo ❌ Error: No se encuentra el panel HTML
    echo Directorio actual: %CD%
    echo.
    echo Solución: Abre el panel directamente
    echo.
    start "" "server-railway\static\index.html"
    pause
    exit /b 1
)

echo ✅ Panel encontrado

REM Abrir el panel HTML directamente en el navegador
echo 🌐 Abriendo panel en el navegador...
start "" "server-railway\static\index.html"

echo.
echo ==========================================
echo   Panel de Administración ERP - ABIERTO
echo ==========================================
echo.
echo ✅ El panel se abrió en tu navegador
echo.
echo Funcionalidades disponibles:
echo   🎥 Configuración de video (resolución, FPS, bitrate)
echo   ⚡ Configuración de rendimiento (codec, calidad)
echo   🌐 Configuración de red (protocolo, buffer)
echo   📊 Gestión de contenedores
echo.
echo NOTA: Este panel funciona sin servidor
echo Los cambios se guardan localmente en el navegador
echo.
pause