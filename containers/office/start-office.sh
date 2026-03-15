#!/bin/bash

echo "🚀 Iniciando Office Container..."

# Configurar usuario RDP si se especifica
if [ ! -z "$RDP_USER" ] && [ ! -z "$RDP_PASSWORD" ]; then
    echo "👤 Configurando usuario RDP: $RDP_USER"
    echo "$RDP_USER:$RDP_PASSWORD" | chpasswd
fi

# Generar certificados SSL para XRDP
if [ ! -f /etc/xrdp/rsakeys.ini ]; then
    echo "🔐 Generando certificados XRDP..."
    xrdp-keygen xrdp auto
fi

# Configurar permisos
chown -R officeuser:officeuser /home/officeuser

# Crear directorio de documentos
mkdir -p /home/officeuser/Documents
mkdir -p /home/officeuser/Desktop
chown -R officeuser:officeuser /home/officeuser/Documents
chown -R officeuser:officeuser /home/officeuser/Desktop

# Crear acceso directo a LibreOffice en el escritorio
cat > /home/officeuser/Desktop/LibreOffice.desktop << 'EOF'
[Desktop Entry]
Version=1.0
Type=Application
Name=LibreOffice
Comment=Office Suite
Exec=libreoffice
Icon=libreoffice-startcenter
Terminal=false
Categories=Office;
EOF

chmod +x /home/officeuser/Desktop/LibreOffice.desktop
chown officeuser:officeuser /home/officeuser/Desktop/LibreOffice.desktop

echo "✅ Office Container configurado"
echo "👤 Usuario RDP: $RDP_USER"
echo "🔌 Puerto RDP: 3389"
echo "📁 Documentos: /home/officeuser/Documents"

# Iniciar supervisor
exec /usr/bin/supervisord -c /etc/supervisor/conf.d/supervisord.conf