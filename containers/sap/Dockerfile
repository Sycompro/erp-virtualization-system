# SAP GUI Container con VNC
FROM ubuntu:22.04

# Evitar prompts interactivos
ENV DEBIAN_FRONTEND=noninteractive

# Instalar dependencias base
RUN apt-get update && apt-get install -y \
    xvfb \
    x11vnc \
    fluxbox \
    wget \
    curl \
    supervisor \
    net-tools \
    novnc \
    websockify \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario para SAP
RUN useradd -m -s /bin/bash sapuser && \
    echo 'sapuser:sapuser' | chpasswd

# Configurar VNC
RUN mkdir -p /home/sapuser/.vnc && \
    x11vnc -storepasswd sapvnc /home/sapuser/.vnc/passwd && \
    chown -R sapuser:sapuser /home/sapuser/.vnc

# Crear directorio para SAP
RUN mkdir -p /opt/sap && chown sapuser:sapuser /opt/sap

# Configurar supervisor
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

# Configurar noVNC
RUN ln -s /usr/share/novnc/vnc.html /usr/share/novnc/index.html

# Script de inicio
COPY start-sap.sh /usr/local/bin/start-sap.sh
RUN chmod +x /usr/local/bin/start-sap.sh

# Exponer puertos
EXPOSE 5900 6080

# Variables de entorno
ENV DISPLAY=:1
ENV VNC_RESOLUTION=1920x1080
ENV VNC_PASSWORD=sapvnc

# Cambiar a usuario SAP
USER sapuser
WORKDIR /home/sapuser

# Comando por defecto
CMD ["/usr/local/bin/start-sap.sh"]