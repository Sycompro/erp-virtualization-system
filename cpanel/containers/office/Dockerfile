# Microsoft Office Container con RDP
FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# Instalar dependencias
RUN apt-get update && apt-get install -y \
    xrdp \
    xfce4 \
    xfce4-goodies \
    firefox \
    libreoffice \
    supervisor \
    wget \
    curl \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario office
RUN useradd -m -s /bin/bash officeuser && \
    echo 'officeuser:office123' | chpasswd && \
    adduser officeuser sudo

# Configurar XRDP
RUN sed -i 's/3389/3389/g' /etc/xrdp/xrdp.ini && \
    sed -i 's/max_bpp=32/#max_bpp=32\nmax_bpp=128/g' /etc/xrdp/xrdp.ini && \
    sed -i 's/xserverbpp=24/#xserverbpp=24\nxserverbpp=128/g' /etc/xrdp/xrdp.ini && \
    echo xfce4-session > /home/officeuser/.xsession

# Configurar supervisor
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf

# Script de inicio
COPY start-office.sh /usr/local/bin/start-office.sh
RUN chmod +x /usr/local/bin/start-office.sh

# Configurar XFCE para usuario
RUN mkdir -p /home/officeuser/.config/xfce4/xfconf/xfce-perchannel-xml
COPY xfce4-desktop.xml /home/officeuser/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-desktop.xml
RUN chown -R officeuser:officeuser /home/officeuser/.config

# Exponer puerto RDP
EXPOSE 3389

# Variables de entorno
ENV RDP_USER=officeuser
ENV RDP_PASSWORD=office123

CMD ["/usr/local/bin/start-office.sh"]