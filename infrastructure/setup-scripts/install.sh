#!/bin/bash
# Script de instalación para ERP Virtualization - 5 Tablets

set -e

echo "🚀 Instalando ERP Virtualization para 5 tablets..."

# Verificar que se ejecuta como root
if [ "$EUID" -ne 0 ]; then
    echo "❌ Este script debe ejecutarse como root (sudo)"
    exit 1
fi

# Actualizar sistema
echo "📦 Actualizando sistema..."
apt update && apt upgrade -y

# Instalar dependencias básicas
echo "🔧 Instalando dependencias..."
apt install -y \
    curl \
    wget \
    git \
    htop \
    ufw \
    fail2ban \
    certbot \
    nginx \
    postgresql-client \
    redis-tools

# Instalar Docker
echo "🐳 Instalando Docker..."
if ! command -v docker &> /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    sh get-docker.sh
    usermod -aG docker $SUDO_USER
    rm get-docker.sh
fi

# Instalar Docker Compose
echo "🐙 Instalando Docker Compose..."
if ! command -v docker-compose &> /dev/null; then
    curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    chmod +x /usr/local/bin/docker-compose
fi

# Crear estructura de directorios
echo "📁 Creando estructura de directorios..."
mkdir -p /opt/erp-simple/{data/{postgres,redis,user-files,sap,office,autocad},logs/{nginx,erp},nginx/ssl,monitoring}

# Configurar firewall
echo "🔥 Configurando firewall..."
ufw --force reset
ufw default deny incoming
ufw default allow outgoing

# Permitir SSH solo desde red local
ufw allow from 192.168.0.0/16 to any port 22

# Permitir HTTP/HTTPS
ufw allow 80/tcp
ufw allow 443/tcp

# Permitir WebRTC/TURN
ufw allow 3478/tcp
ufw allow 3478/udp
ufw allow 49152:49162/udp

# Permitir VNC (solo desde red local)
ufw allow from 192.168.0.0/16 to any port 5900:5904

# Permitir RDP (solo desde red local)
ufw allow from 192.168.0.0/16 to any port 3389:3393

# Activar firewall
ufw --force enable

# Generar certificados SSL auto-firmados (para desarrollo)
echo "🔐 Generando certificados SSL..."
if [ ! -f /opt/erp-simple/nginx/ssl/server.crt ]; then
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout /opt/erp-simple/nginx/ssl/server.key \
        -out /opt/erp-simple/nginx/ssl/server.crt \
        -subj "/C=US/ST=State/L=City/O=Organization/CN=erp.tuempresa.com"
fi

# Configurar fail2ban
echo "🛡️ Configurando fail2ban..."
cat > /etc/fail2ban/jail.local << EOF
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 3

[sshd]
enabled = true
port = ssh
logpath = /var/log/auth.log

[nginx-http-auth]
enabled = true
port = http,https
logpath = /opt/erp-simple/logs/nginx/error.log

[nginx-limit-req]
enabled = true
port = http,https
logpath = /opt/erp-simple/logs/nginx/error.log
maxretry = 10
EOF

systemctl enable fail2ban
systemctl restart fail2ban

# Crear archivo de configuración de base de datos
echo "🗄️ Creando configuración de base de datos..."
cat > /opt/erp-simple/init-db.sql << EOF
-- Crear tablas básicas para ERP Virtualization
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id INTEGER REFERENCES users(id),
    container_id VARCHAR(100),
    application_type VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_activity TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

CREATE TABLE IF NOT EXISTS applications (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    type VARCHAR(50) NOT NULL,
    image_name VARCHAR(200) NOT NULL,
    display_protocol VARCHAR(20) NOT NULL,
    default_port INTEGER,
    is_active BOOLEAN DEFAULT true
);

-- Insertar aplicaciones por defecto
INSERT INTO applications (name, type, image_name, display_protocol, default_port) VALUES
('SAP GUI', 'sap', 'erp-virtualization/sap-gui:latest', 'VNC', 5900),
('Microsoft Office', 'office', 'erp-virtualization/office:latest', 'RDP', 3389),
('AutoCAD', 'autocad', 'erp-virtualization/autocad:latest', 'RDP', 3390);

-- Crear usuario administrador por defecto (password: admin123)
INSERT INTO users (username, email, password_hash) VALUES
('admin', 'admin@tuempresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e');
EOF

# Crear archivo de monitoreo
echo "📊 Configurando monitoreo..."
cat > /opt/erp-simple/monitoring/prometheus.yml << EOF
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'erp-server'
    static_configs:
      - targets: ['erp-server:8080']
  
  - job_name: 'nginx'
    static_configs:
      - targets: ['nginx:80']
  
  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres:5432']
EOF

# Crear scripts de utilidad
echo "🛠️ Creando scripts de utilidad..."

# Script de inicio
cat > /opt/erp-simple/start.sh << 'EOF'
#!/bin/bash
cd /opt/erp-simple
echo "🚀 Iniciando ERP Virtualization..."
docker-compose up -d
echo "✅ Sistema iniciado. Accede a https://$(hostname -I | awk '{print $1}')"
EOF

# Script de parada
cat > /opt/erp-simple/stop.sh << 'EOF'
#!/bin/bash
cd /opt/erp-simple
echo "🛑 Deteniendo ERP Virtualization..."
docker-compose down
echo "✅ Sistema detenido"
EOF

# Script de backup
cat > /opt/erp-simple/backup.sh << 'EOF'
#!/bin/bash
BACKUP_DIR="/backup/$(date +%Y%m%d_%H%M%S)"
mkdir -p $BACKUP_DIR

echo "💾 Creando backup en $BACKUP_DIR..."

# Backup de datos de usuario
rsync -av /opt/erp-simple/data/user-files/ $BACKUP_DIR/user-files/

# Backup de base de datos
docker exec erp-postgres pg_dump -U erp_user erp_db > $BACKUP_DIR/database.sql

# Backup de configuración
cp -r /opt/erp-simple/nginx/ $BACKUP_DIR/config/

echo "✅ Backup completado en $BACKUP_DIR"

# Limpiar backups antiguos (mantener 7 días)
find /backup/ -type d -mtime +7 -exec rm -rf {} \; 2>/dev/null || true
EOF

# Script de monitoreo
cat > /opt/erp-simple/health-check.sh << 'EOF'
#!/bin/bash
echo "🏥 Verificando salud del sistema..."

# Verificar containers
if ! docker ps | grep -q "erp-server"; then
    echo "❌ ERROR: Servidor ERP no está corriendo"
    exit 1
fi

if ! docker ps | grep -q "erp-postgres"; then
    echo "❌ ERROR: Base de datos no está corriendo"
    exit 1
fi

# Verificar espacio en disco
DISK_USAGE=$(df /opt/erp-simple | tail -1 | awk '{print $5}' | sed 's/%//')
if [ $DISK_USAGE -gt 80 ]; then
    echo "⚠️ WARNING: Disco al ${DISK_USAGE}%"
fi

# Verificar memoria
MEM_USAGE=$(free | grep Mem | awk '{printf "%.0f", $3/$2 * 100.0}')
if [ $MEM_USAGE -gt 85 ]; then
    echo "⚠️ WARNING: Memoria al ${MEM_USAGE}%"
fi

# Verificar conectividad
if ! curl -k -s https://localhost/health > /dev/null; then
    echo "❌ ERROR: Servidor web no responde"
    exit 1
fi

echo "✅ Sistema funcionando correctamente"
EOF

# Hacer scripts ejecutables
chmod +x /opt/erp-simple/*.sh

# Crear servicio systemd
echo "⚙️ Creando servicio systemd..."
cat > /etc/systemd/system/erp-virtualization.service << EOF
[Unit]
Description=ERP Virtualization Service
Requires=docker.service
After=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
WorkingDirectory=/opt/erp-simple
ExecStart=/opt/erp-simple/start.sh
ExecStop=/opt/erp-simple/stop.sh
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable erp-virtualization

# Configurar cron jobs
echo "⏰ Configurando tareas programadas..."
(crontab -l 2>/dev/null; echo "0 2 * * * /opt/erp-simple/backup.sh") | crontab -
(crontab -l 2>/dev/null; echo "*/5 * * * * /opt/erp-simple/health-check.sh") | crontab -

# Configurar permisos
chown -R $SUDO_USER:$SUDO_USER /opt/erp-simple
chmod 755 /opt/erp-simple/data

echo ""
echo "🎉 ¡Instalación completada!"
echo ""
echo "📋 Próximos pasos:"
echo "1. Copiar docker-compose.yml a /opt/erp-simple/"
echo "2. Copiar nginx.conf a /opt/erp-simple/nginx/"
echo "3. Construir las imágenes de aplicaciones"
echo "4. Ejecutar: sudo systemctl start erp-virtualization"
echo ""
echo "🌐 El sistema estará disponible en:"
echo "   https://$(hostname -I | awk '{print $1}')"
echo ""
echo "👤 Usuario por defecto:"
echo "   Username: admin"
echo "   Password: admin123"
echo ""
EOF