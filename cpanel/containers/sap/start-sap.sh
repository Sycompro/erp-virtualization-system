#!/bin/bash

echo "🚀 Iniciando SAP GUI Container..."

# Configurar resolución VNC
export VNC_RESOLUTION=${VNC_RESOLUTION:-1920x1080}

# Configurar password VNC
if [ ! -z "$VNC_PASSWORD" ]; then
    echo "🔐 Configurando password VNC..."
    x11vnc -storepasswd "$VNC_PASSWORD" /home/sapuser/.vnc/passwd
fi

# Crear directorio de trabajo SAP
mkdir -p /home/sapuser/sap-workspace
cd /home/sapuser/sap-workspace

# Configurar fluxbox
mkdir -p /home/sapuser/.fluxbox
cat > /home/sapuser/.fluxbox/startup << 'EOF'
#!/bin/bash
# Configuración de Fluxbox para SAP

# Configurar fondo
xsetroot -solid "#2E3440"

# Configurar teclado
setxkbmap us

# Iniciar terminal por defecto
xterm -geometry 80x24+10+10 -title "SAP Terminal" &

# Mensaje de bienvenida
xmessage -center -timeout 10 "SAP GUI Container Iniciado
Resolución: $VNC_RESOLUTION
Acceso VNC: Puerto 5900
Acceso Web: Puerto 6080" &

# Iniciar Fluxbox
exec fluxbox
EOF

chmod +x /home/sapuser/.fluxbox/startup

echo "✅ SAP GUI Container configurado"
echo "📊 Resolución: $VNC_RESOLUTION"
echo "🔌 VNC Puerto: 5900"
echo "🌐 Web Puerto: 6080"

# Iniciar supervisor
exec /usr/bin/supervisord -c /etc/supervisor/conf.d/supervisord.conf