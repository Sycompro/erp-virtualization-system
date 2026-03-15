@echo off
echo 🧪 Probando Panel de Administración - Modo Simple
echo ================================================

REM Detener IIS para evitar conflictos
echo 🛑 Liberando puerto...
taskkill /f /im "w3wp.exe" 2>nul
net stop "World Wide Web Publishing Service" 2>nul

REM Crear un servidor web simple con Python para probar el panel HTML
echo 🐍 Iniciando servidor web simple...

cd server-railway\static

REM Verificar si Python está disponible
python --version >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ Python encontrado, iniciando servidor...
    echo.
    echo ==========================================
    echo   Panel de Administración ERP - DEMO
    echo ==========================================
    echo.
    echo 🌐 Panel: http://localhost:8000/
    echo 📱 Móvil: http://localhost:8000/
    echo.
    echo NOTA: Este es un servidor de prueba
    echo Las funciones de API no estarán disponibles
    echo.
    echo Presiona Ctrl+C para detener
    echo ==========================================
    echo.
    
    python -m http.server 8000
) else (
    echo ❌ Python no encontrado
    echo.
    echo Alternativas:
    echo 1. Instala Python desde python.org
    echo 2. Usa Node.js: npx serve .
    echo 3. Abre index.html directamente en el navegador
    echo.
    pause
)