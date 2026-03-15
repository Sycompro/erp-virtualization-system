@echo off
echo.
echo ========================================
echo   DESPLIEGUE INMEDIATO EN RAILWAY
echo ========================================
echo.

echo 🚂 Iniciando despliegue en produccion...
echo.

echo 📋 INSTRUCCIONES:
echo 1. Ejecuta en otra terminal: railway login
echo 2. Completa la autenticacion en el navegador
echo 3. Presiona ENTER aqui para continuar
echo.

pause

echo 🔍 Verificando Railway CLI...
railway --version
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Railway CLI no encontrado
    echo 💡 Instala con: npm install -g @railway/cli
    pause
    exit /b 1
)

echo.
echo 🔐 Verificando autenticacion...
railway whoami
if %ERRORLEVEL% NEQ 0 (
    echo ❌ No autenticado. Ejecuta: railway login
    pause
    exit /b 1
)

echo.
echo ✅ Autenticado correctamente
echo.

echo 🏗️ Inicializando proyecto...
railway init

echo.
echo 🚀 Desplegando aplicacion...
railway up

echo.
echo ========================================
echo   DESPLIEGUE COMPLETADO
echo ========================================
echo.
echo 🌐 Aplicacion desplegada en Railway
echo 💡 Ejecuta 'railway open' para ver la URL
echo 📊 Panel admin: [URL]/admin/
echo.

railway status
echo.

pause