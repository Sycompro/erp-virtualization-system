@echo off
echo.
echo ========================================
echo   DESPLIEGUE EN PRODUCCION - RAILWAY
echo ========================================
echo.

echo 🚂 Desplegando ERP Virtualization en Railway
echo.

echo 📋 PASOS A SEGUIR:
echo.
echo 1. Abre una nueva terminal
echo 2. Ejecuta: railway login
echo 3. Sigue las instrucciones del navegador
echo 4. Vuelve aqui y presiona ENTER
echo.

pause

echo.
echo 🔍 Verificando autenticacion...
railway whoami
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ❌ Error: No estas autenticado en Railway
    echo 💡 Ejecuta 'railway login' primero
    pause
    exit /b 1
)

echo.
echo ✅ Autenticado correctamente
echo.

echo 🏗️ Creando proyecto Railway...
railway init erp-virtualization

echo.
echo 🔧 Configurando variables de entorno...
railway variables set RUST_LOG=info
railway variables set RAILWAY_ENVIRONMENT=production
railway variables set PORT=8080

echo.
echo 🗄️ Agregando PostgreSQL...
railway add postgresql

echo.
echo 📦 Desplegando aplicacion...
git add .
git commit -m "Deploy to Railway production" || echo "No changes to commit"
railway up --detach

echo.
echo ⏳ Esperando despliegue...
timeout /t 30 /nobreak

echo.
echo 📊 Obteniendo informacion del despliegue...
railway status

echo.
echo ========================================
echo   DESPLIEGUE COMPLETADO
echo ========================================
echo.
echo 🌐 Tu aplicacion esta desplegada en Railway
echo 📱 Panel de admin disponible en: [URL]/admin/
echo 🔍 Health check: [URL]/health
echo 📊 API: [URL]/applications/list
echo.
echo 💡 Usa 'railway open' para abrir la aplicacion
echo 💡 Usa 'railway logs' para ver los logs
echo.

pause