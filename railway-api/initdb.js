const express = require('express');
const path = require('path');
const fs = require('fs');

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(express.json());
app.use(express.static('static'));

// Rutas principales
app.get('/', (req, res) => {
    res.json({
        service: "ERP Virtualization API",
        version: "0.1.0",
        status: "running",
        message: "Panel de administración funcionando",
        endpoints: {
            health: "/health",
            admin: "/admin",
            api: "/api"
        }
    });
});

app.get('/health', (req, res) => {
    res.json({
        status: "healthy",
        service: "erp-railway-api",
        version: "0.1.0",
        timestamp: new Date().toISOString()
    });
});

// Servir panel de administración
app.get('/admin', (req, res) => {
    const adminPath = path.join(__dirname, 'static', 'index.html');
    if (fs.existsSync(adminPath)) {
        res.sendFile(adminPath);
    } else {
        res.json({
            message: "Panel de administración",
            status: "active",
            version: "0.1.0"
        });
    }
});

// API endpoints mock
app.get('/api/applications', (req, res) => {
    res.json({
        applications: [
            { id: 1, name: "SAP ERP", category: "ERP", status: "available" },
            { id: 2, name: "Microsoft Office", category: "Office", status: "available" }
        ]
    });
});

app.post('/api/auth/login', (req, res) => {
    res.json({
        success: true,
        token: "mock-jwt-token",
        user: { id: 1, username: "admin" }
    });
});

app.listen(PORT, '0.0.0.0', () => {
    console.log(`🚂 Railway API Server running on port ${PORT}`);
    console.log(`📡 Health check: http://localhost:${PORT}/health`);
    console.log(`🎛️ Admin panel: http://localhost:${PORT}/admin`);
});
