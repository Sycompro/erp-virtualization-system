@echo off
title ERP Virtualization - Instalador cPanel
color 0A

echo.
echo ========================================
echo 🏠 INSTALADOR cPanel ERP VIRTUALIZATION
echo ========================================
echo.
echo Este instalador configurara tu PC para
echo ofrecer aplicaciones virtualizadas a
echo tus clientes de forma remota.
echo.
echo 💰 Genera ingresos con SAP, Office, AutoCAD
echo 🌐 Clientes se conectan desde cualquier lugar  
echo 📱 App movil + navegador web
echo ⚡ Instalacion automatica completa
echo.
pause

echo.
echo 📋 VERIFICANDO SISTEMA...

:: Verificar Windows version
for /f "tokens=4-5 delims=. " %%i in ('ver') do set VERSION=%%i.%%j
if "%version%" == "10.0" (
    echo ✅ Windows 10/11 detectado
) else (
    echo ❌ Se requiere Windows 10 o superior
    pause
    exit /b 1
)

:: Verificar RAM
for /f "skip=1" %%p in ('wmic computersystem get TotalPhysicalMemory') do (
    set /a RAM_GB=%%p/1024/1024/1024
    goto :ram_done
)
:ram_done
if %RAM_GB% GEQ 16 (
    echo ✅ RAM: %RAM_GB% GB (suficiente)
) else (
    echo ⚠️ RAM: %RAM_GB% GB (recomendado: 16+ GB)
    echo    El sistema funcionara pero con rendimiento limitado
)

:: Verificar espacio en disco
for /f "tokens=3" %%a in ('dir /-c %SystemDrive%\ ^| find "bytes free"') do set DISK_FREE=%%a
set /a DISK_GB=%DISK_FREE:~0,-3%/1024/1024
if %DISK_GB% GEQ 100 (
    echo ✅ Espacio libre: %DISK_GB% GB
) else (
    echo ❌ Espacio insuficiente: %DISK_GB% GB (se requieren 100+ GB)
    pause
    exit /b 1
)

echo.
echo 🔧 INSTALANDO DEPENDENCIAS...

:: Verificar/Instalar Docker Desktop
docker --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 📦 Descargando Docker Desktop...
    powershell -Command "Invoke-WebRequest -Uri 'https://desktop.docker.com/win/main/amd64/Docker%%20Desktop%%20Installer.exe' -OutFile 'DockerInstaller.exe'"
    
    echo 🔧 Instalando Docker Desktop...
    echo    IMPORTANTE: Se abrira el instalador de Docker
    echo    - Acepta todos los valores por defecto
    echo    - Reinicia cuando te lo pida
    echo    - Ejecuta este script nuevamente despues del reinicio
    pause
    
    start /wait DockerInstaller.exe install --quiet
    del DockerInstaller.exe
    
    echo.
    echo ⚠️ REINICIO REQUERIDO
    echo Docker Desktop requiere reiniciar el sistema.
    echo Despues del reinicio, ejecuta este script nuevamente.
    pause
    shutdown /r /t 60 /c "Reiniciando para completar instalacion de Docker..."
    exit /b 0
) else (
    echo ✅ Docker Desktop ya instalado
)

:: Verificar/Instalar Rust
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 📦 Descargando Rust...
    powershell -Command "Invoke-WebRequest -Uri 'https://win.rustup.rs/x86_64' -OutFile 'rustup-init.exe'"
    
    echo 🔧 Instalando Rust...
    rustup-init.exe -y --default-toolchain stable
    del rustup-init.exe
    
    :: Actualizar PATH
    call "%USERPROFILE%\.cargo\env.bat"
    echo ✅ Rust instalado correctamente
) else (
    echo ✅ Rust ya instalado
)

:: Verificar/Instalar Git
git --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 📦 Descargando Git...
    powershell -Command "Invoke-WebRequest -Uri 'https://github.com/git-for-windows/git/releases/latest/download/Git-2.42.0.2-64-bit.exe' -OutFile 'GitInstaller.exe'"
    
    echo 🔧 Instalando Git...
    GitInstaller.exe /SILENT /COMPONENTS="icons,ext\reg\shellhere,assoc,assoc_sh"
    del GitInstaller.exe
    echo ✅ Git instalado correctamente
) else (
    echo ✅ Git ya instalado
)

echo.
echo 📥 DESCARGANDO cPanel...

:: Crear directorio de instalacion
set INSTALL_DIR=%USERPROFILE%\ERP-Virtualization
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
cd /d "%INSTALL_DIR%"

:: Clonar o actualizar repositorio
if exist ".git" (
    echo 🔄 Actualizando codigo fuente...
    git pull origin main
) else (
    echo 📥 Descargando codigo fuente...
    git clone https://github.com/Sycompro/erp-virtualization-system.git .
)

echo.
echo 🔨 COMPILANDO cPanel...
cd cpanel

:: Compilar en modo release
echo 🚀 Compilando servidor cPanel (esto puede tomar varios minutos)...
cargo build --release
if %errorlevel% neq 0 (
    echo ❌ Error compilando cPanel
    echo 💡 Verifica que Rust este instalado correctamente
    pause
    exit /b 1
)
echo ✅ cPanel compilado correctamente

cd ..

echo.
echo 🐳 PREPARANDO CONTAINERS...

:: Construir containers de aplicaciones
echo 📦 Construyendo container SAP...
cd cpanel\containers\sap
docker build -t erp-virtualization/sap-gui:latest . --quiet
if %errorlevel% neq 0 (
    echo ⚠️ Error construyendo container SAP (continuando...)
)

echo 📦 Construyendo container Office...
cd ..\office
docker build -t erp-virtualization/office:latest . --quiet
if %errorlevel% neq 0 (
    echo ⚠️ Error construyendo container Office (continuando...)
)

cd ..\..\..

echo.
echo 🌐 CONFIGURANDO FIREWALL...

:: Abrir puertos en firewall
echo 🔥 Configurando reglas de firewall...
netsh advfirewall firewall add rule name="ERP cPanel" dir=in action=allow protocol=TCP localport=8081 >nul 2>&1
netsh advfirewall firewall add rule name="ERP VNC" dir=in action=allow protocol=TCP localport=5900-5950 >nul 2>&1
netsh advfirewall firewall add rule name="ERP RDP" dir=in action=allow protocol=TCP localport=3389-3450 >nul 2>&1
netsh advfirewall firewall add rule name="ERP WebRTC" dir=in action=allow protocol=TCP localport=8080-8090 >nul 2>&1
echo ✅ Firewall configurado

echo.
echo 📱 COMPILANDO APP ANDROID...
cd android
call gradlew assembleRelease --quiet
if %errorlevel% equ 0 (
    echo ✅ APK compilado: app\build\outputs\apk\release\app-release.apk
    copy app\build\outputs\apk\release\app-release.apk "%INSTALL_DIR%\erp-virtualization-client.apk" >nul 2>&1
) else (
    echo ⚠️ Error compilando APK (opcional)
)
cd ..

echo.
echo 🎯 CREANDO ACCESOS DIRECTOS...

:: Crear acceso directo en escritorio
set DESKTOP=%USERPROFILE%\Desktop
echo [InternetShortcut] > "%DESKTOP%\cPanel ERP.url"
echo URL=http://localhost:8081 >> "%DESKTOP%\cPanel ERP.url"
echo IconFile=%INSTALL_DIR%\cpanel\icon.ico >> "%DESKTOP%\cPanel ERP.url"

:: Crear script de inicio
echo @echo off > "%INSTALL_DIR%\iniciar-cpanel.bat"
echo title cPanel ERP Virtualization >> "%INSTALL_DIR%\iniciar-cpanel.bat"
echo cd /d "%INSTALL_DIR%\cpanel" >> "%INSTALL_DIR%\iniciar-cpanel.bat"
echo echo Iniciando cPanel ERP Virtualization... >> "%INSTALL_DIR%\iniciar-cpanel.bat"
echo echo Panel disponible en: http://localhost:8081 >> "%INSTALL_DIR%\iniciar-cpanel.bat"
echo echo. >> "%INSTALL_DIR%\iniciar-cpanel.bat"
echo cargo run --release >> "%INSTALL_DIR%\iniciar-cpanel.bat"
echo pause >> "%INSTALL_DIR%\iniciar-cpanel.bat"

:: Crear acceso directo para iniciar
echo [InternetShortcut] > "%DESKTOP%\Iniciar cPanel.url"
echo URL=file:///%INSTALL_DIR%\iniciar-cpanel.bat >> "%DESKTOP%\Iniciar cPanel.url"

echo.
echo 🧪 EJECUTANDO PRUEBAS...

:: Probar compilacion
echo 🔍 Verificando instalacion...
cd cpanel
timeout /t 2 /nobreak >nul
cargo run --release --bin erp-local-server -- --help >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ cPanel funciona correctamente
) else (
    echo ⚠️ Advertencia: No se pudo verificar cPanel
)
cd ..

echo.
echo ========================================
echo ✅ INSTALACION COMPLETADA EXITOSAMENTE
echo ========================================
echo.
echo 🎉 cPanel ERP Virtualization instalado en:
echo    %INSTALL_DIR%
echo.
echo 🚀 COMO INICIAR:
echo    1. Doble clic en "Iniciar cPanel" (escritorio)
echo    2. O ejecutar: %INSTALL_DIR%\iniciar-cpanel.bat
echo    3. Abrir navegador: http://localhost:8081
echo.
echo 📱 APP PARA CLIENTES:
echo    Archivo: %INSTALL_DIR%\erp-virtualization-client.apk
echo    Instalar en dispositivos Android de tus clientes
echo.
echo 💰 PROXIMOS PASOS:
echo    1. Iniciar cPanel
echo    2. Conectar con Railway (boton en panel)
echo    3. Configurar aplicaciones disponibles
echo    4. Compartir APK con clientes
echo    5. ¡Empezar a generar ingresos!
echo.
echo 📚 DOCUMENTACION:
echo    %INSTALL_DIR%\docs\INSTALACION_CLIENTE.md
echo.
echo 🎯 ¿INICIAR AHORA? (S/N)
set /p START_NOW=
if /i "%START_NOW%"=="S" (
    echo.
    echo 🚀 Iniciando cPanel...
    start "" "%INSTALL_DIR%\iniciar-cpanel.bat"
    timeout /t 3 /nobreak >nul
    start "" "http://localhost:8081"
    echo.
    echo ✅ cPanel iniciado en http://localhost:8081
)

echo.
echo 🎉 ¡INSTALACION COMPLETA!
echo    Tu negocio de virtualizacion esta listo.
pause