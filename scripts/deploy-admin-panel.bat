@echo off
echo 🎛️  Desplegando Panel de Administración ERP
echo ==========================================

REM Verificar que estamos en el directorio correcto
if not exist "server-railway" (
    echo ❌ Error: Ejecuta este script desde la raíz del proyecto
    pause
    exit /b 1
)

echo 📁 Preparando archivos...

REM Crear directorio static si no existe
if not exist "server-railway\static" mkdir "server-railway\static"

echo 🔧 Compilando servidor Railway...
cd server-railway

REM Compilar el proyecto
cargo build --release

if %errorlevel% equ 0 (
    echo ✅ Compilación exitosa
    echo.
    echo 🚀 Iniciando servidor...
    echo.
    echo Panel de Administración disponible en:
    echo   🌐 http://localhost:8080/admin/
    echo   📊 API Health: http://localhost:8080/health
    echo.
    echo Funcionalidades disponibles:
    echo   ✅ Configuración de visualización ^(como Parallels^)
    echo   ✅ Gestión de contenedores ERP
    echo   ✅ Estadísticas en tiempo real
    echo   ✅ Control de calidad de video
    echo   ✅ Configuración de red y rendimiento
    echo.
    echo Presiona Ctrl+C para detener el servidor
    echo.
    
    REM Ejecutar el servidor
    cargo run --release
) else (
    echo ❌ Error en la compilación
    echo Revisa los errores arriba
    pause
    exit /b 1
)