@echo off
echo 🚀 Building ERP Railway API...

REM Check if Docker is running
docker info >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Docker is not running. Please start Docker Desktop first.
    exit /b 1
)

REM Build the Docker image
echo 📦 Building Docker image...
docker build -f Dockerfile.railway -t erp-railway-api:latest .

if %errorlevel% equ 0 (
    echo ✅ Build successful!
    echo 🏷️  Image: erp-railway-api:latest
    echo.
    echo To test locally:
    echo   docker run -p 8080:8080 -e DATABASE_URL=your_db_url erp-railway-api:latest
    echo.
    echo To deploy to Railway:
    echo   railway up
) else (
    echo ❌ Build failed!
    exit /b 1
)