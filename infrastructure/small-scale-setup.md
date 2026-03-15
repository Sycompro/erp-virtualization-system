# Configuración Simplificada para 5 Tablets

## 1. Hardware Mínimo Requerido

### Servidor Principal (Una sola máquina):
```
CPU: Intel i7-12700K o AMD Ryzen 7 5800X (8 cores)
RAM: 32GB DDR4 (suficiente para 5 usuarios concurrentes)
Storage: 
  - 1TB NVMe SSD (sistema + aplicaciones)
  - 2TB HDD SATA (datos de usuario)
GPU: NVIDIA GTX 1660 o RTX 3060 (para CAD/Design)
Red: 1Gbps Ethernet (suficiente para 5 streams)

Costo estimado: $3,000 - $5,000 USD
```

### Especificaciones por Usuario:
```
Por tablet/usuario:
- CPU: 1.5 cores
- RAM: 6GB
- Storage: 100GB SSD + 400GB HDD
- Bandwidth: 10-25 Mbps por stream
```

## 2. Software Stack Simplificado

### Sin Kubernetes (Demasiado complejo para 5 usuarios):
```
Ubuntu Server 22.04 LTS
├── Docker Compose (orquestación simple)
├── Nginx (load balancer + SSL)
├── PostgreSQL (base de datos)
├── Redis (cache y sesiones)
└── Aplicación Rust (servidor principal)
```

## 3. Estructura de Directorios

```
/opt/erp-simple/
├── docker-compose.yml
├── nginx/
│   ├── nginx.conf
│   └── ssl/
├── data/
│   ├── postgres/
│   ├── redis/
│   └── user-files/
├── containers/
│   ├── sap/
│   ├── office/
│   └── autocad/
└── logs/
    ├── access.log
    └── error.log
```

## 4. Configuración de Red Simple

### Red Local:
```
Router/Firewall: 192.168.1.1
Servidor ERP:    192.168.1.100
Tablets:         192.168.1.101-105
DNS Local:       192.168.1.1 (router)
```

### Puertos Necesarios:
```
443  - HTTPS (app Android)
3478 - TURN server (WebRTC)
5900-5904 - VNC (una por tablet)
3389-3393 - RDP (una por tablet)
```

## 5. Configuración de Dominio Simple

### Opción 1: Dominio Local (más simple)
```
erp.local          # Servidor principal
tablet1.erp.local  # Tablet 1
tablet2.erp.local  # Tablet 2
...
```

### Opción 2: Dominio Público (recomendado)
```
erp.tuempresa.com  # Servidor principal
```

## 6. Backup Simplificado

### Script de backup diario:
```bash
#!/bin/bash
# Backup simple para 5 usuarios

# Crear directorio de backup
BACKUP_DIR="/backup/$(date +%Y%m%d)"
mkdir -p $BACKUP_DIR

# Backup de datos de usuario
rsync -av /opt/erp-simple/data/user-files/ $BACKUP_DIR/user-files/

# Backup de base de datos
docker exec postgres pg_dump -U erp_user erp_db > $BACKUP_DIR/database.sql

# Backup de configuración
cp -r /opt/erp-simple/nginx/ $BACKUP_DIR/config/

# Limpiar backups antiguos (mantener 30 días)
find /backup/ -type d -mtime +30 -exec rm -rf {} \;
```

## 7. Monitoreo Simple

### Script de salud del sistema:
```bash
#!/bin/bash
# Verificar que todo esté funcionando

# Verificar containers
docker ps | grep -q "erp-server" || echo "ERROR: Servidor ERP no está corriendo"
docker ps | grep -q "postgres" || echo "ERROR: Base de datos no está corriendo"
docker ps | grep -q "redis" || echo "ERROR: Redis no está corriendo"

# Verificar espacio en disco
DISK_USAGE=$(df /opt/erp-simple | tail -1 | awk '{print $5}' | sed 's/%//')
if [ $DISK_USAGE -gt 80 ]; then
    echo "WARNING: Disco al ${DISK_USAGE}%"
fi

# Verificar memoria
MEM_USAGE=$(free | grep Mem | awk '{printf "%.0f", $3/$2 * 100.0}')
if [ $MEM_USAGE -gt 85 ]; then
    echo "WARNING: Memoria al ${MEM_USAGE}%"
fi
```