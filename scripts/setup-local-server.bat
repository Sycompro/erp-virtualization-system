@echo off
echo 🏠 Configurando Servidor Local ERP Virtualization

REM Verificar Docker
docker --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Docker no está instalado. Por favor instala Docker Desktop desde:
    echo https://www.docker.com/products/docker-desktop
    pause
    exit /b 1
)

REM Verificar Rust
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 🦀 Rust no está instalado. Por favor instala Rust desde:
    echo https://rustup.rs/
    pause
    exit /b 1
)

REM Crear directorio de configuración
if not exist "%USERPROFILE%\.erp-local" mkdir "%USERPROFILE%\.erp-local"
copy server-local\.env.example "%USERPROFILE%\.erp-local\.env"

echo ⚙️ Configuración creada en %USERPROFILE%\.erp-local\.env
echo 📝 Edita el archivo para personalizar la configuración

REM Compilar servidor local
echo 🔨 Compilando servidor local...
cd server-local
cargo build --release

if %errorlevel% equ 0 (
    echo ✅ Servidor local compilado exitosamente
    echo.
    echo 🚀 Para iniciar el servidor local:
    echo    cd server-local
    echo    cargo run --release
    echo.
    echo 🔧 Para configurar variables de entorno:
    echo    copy .env.example .env
    echo    notepad .env
) else (
    echo ❌ Error compilando servidor local
    pause
    exit /b 1
)

echo.
echo 📋 Próximos pasos:
echo 1. Configurar variables de entorno en server-local\.env
echo 2. Crear imágenes Docker de las aplicaciones
echo 3. Iniciar el servidor local
echo 4. Probar conexión desde la app Android

pause