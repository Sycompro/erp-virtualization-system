@echo off
echo.
echo ========================================
echo   ARREGLO DE DESPLIEGUE RAILWAY
echo ========================================
echo.

echo 🔧 Aplicando arreglos al despliegue...
echo.

echo 📋 PASOS:
echo 1. Abre https://railway.app en tu navegador
echo 2. Haz login con GitHub/Google
echo 3. Crea un nuevo proyecto
echo 4. Conecta este repositorio
echo 5. Railway detectara automaticamente el Dockerfile.production
echo.

echo 💡 ALTERNATIVA RAPIDA:
echo.
echo Si ya tienes Railway CLI configurado:
echo   railway login
echo   railway init
echo   railway up
echo.

echo 🌐 Una vez desplegado, tu panel estara en:
echo   https://tu-proyecto.railway.app/admin/
echo.

echo 📊 Endpoints disponibles:
echo   /health     - Estado del servidor
echo   /admin/     - Panel de administracion
echo   /api/stats  - Estadisticas
echo   /api/containers - Lista de contenedores
echo.

pause