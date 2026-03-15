#!/bin/bash

# Build script for Railway deployment
echo "🚀 Building ERP Railway API..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker Desktop first."
    exit 1
fi

# Build the Docker image
echo "📦 Building Docker image..."
docker build -f Dockerfile.railway -t erp-railway-api:latest .

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "🏷️  Image: erp-railway-api:latest"
    echo ""
    echo "To test locally:"
    echo "  docker run -p 8080:8080 -e DATABASE_URL=your_db_url erp-railway-api:latest"
    echo ""
    echo "To deploy to Railway:"
    echo "  railway up"
else
    echo "❌ Build failed!"
    exit 1
fi