@echo off
echo.
echo ========================================
echo   DESPLIEGUE EN GITHUB PAGES
echo ========================================
echo.

echo 🌐 Desplegando panel en GitHub Pages...
echo.

echo 📋 Configurando GitHub Pages:
echo 1. Ve a tu repositorio en GitHub
echo 2. Settings > Pages
echo 3. Source: Deploy from a branch
echo 4. Branch: main / docs
echo.

mkdir docs 2>nul
copy "server-railway\static\index.html" "docs\index.html"

echo ✅ Panel copiado a /docs/
echo.
echo 🚀 Haz push y activa GitHub Pages
echo 📱 Tu panel estara en: https://tu-usuario.github.io/tu-repo/
echo.

pause