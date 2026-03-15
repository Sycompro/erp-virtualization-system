# Arquitectura de Almacenamiento para ERP Virtualization

## 1. Estructura de Almacenamiento

### Tipos de Storage:
```
/opt/erp-storage/
├── fast-ssd/           # NVMe SSD para aplicaciones y OS
│   ├── applications/   # Imágenes de containers (1TB)
│   ├── os-images/     # Sistemas operativos base (500GB)
│   └── cache/         # Cache de streaming (200GB)
├── bulk-hdd/          # HDD para datos de usuario
│   ├── user-profiles/ # Perfiles de usuario (5TB)
│   ├── documents/     # Documentos compartidos (10TB)
│   └── backups/       # Respaldos (20TB)
└── network-storage/   # NFS/CIFS para compartir
    ├── shared-apps/   # Aplicaciones compartidas
    ├── templates/     # Plantillas de containers
    └── licenses/      # Licencias de software
```

## 2. Configuración de NFS Server

### /etc/exports
```bash
# Aplicaciones (solo lectura)
/opt/erp-storage/fast-ssd/applications 192.168.0.0/16(ro,sync,no_subtree_check)

# Datos de usuario (lectura/escritura)
/opt/erp-storage/bulk-hdd/user-profiles 192.168.0.0/16(rw,sync,no_subtree_check,no_root_squash)

# Documentos compartidos
/opt/erp-storage/bulk-hdd/documents 192.168.0.0/16(rw,sync,no_subtree_check)

# Cache distribuido
/opt/erp-storage/fast-ssd/cache 192.168.0.0/16(rw,sync,no_subtree_check)
```

## 3. Backup Strategy

### Backup Automático:
```bash
#!/bin/bash
# /opt/scripts/backup-erp.sh

# Backup incremental diario
rsync -av --delete /opt/erp-storage/bulk-hdd/user-profiles/ \
    /backup/daily/$(date +%Y%m%d)/user-profiles/

# Backup de aplicaciones semanal
tar -czf /backup/weekly/applications-$(date +%Y%m%d).tar.gz \
    /opt/erp-storage/fast-ssd/applications/

# Backup de configuraciones
kubectl get all -n erp-system -o yaml > \
    /backup/config/k8s-config-$(date +%Y%m%d).yaml
```

## 4. Gestión de Licencias

### Estructura de Licencias:
```
/opt/erp-storage/licenses/
├── sap/
│   ├── license.key
│   └── users.conf
├── office/
│   ├── volume-license.xml
│   └── activation.key
├── autocad/
│   ├── network-license.lic
│   └── server.config
└── adobe/
    ├── creative-suite.lic
    └── user-limits.json
```

## 5. Monitoreo de Storage

### Métricas importantes:
- **IOPS**: >10,000 para SSD, >500 para HDD
- **Latencia**: <1ms para SSD, <10ms para HDD
- **Throughput**: >1GB/s para aplicaciones
- **Espacio libre**: Alerta si <20% disponible

### Script de monitoreo:
```bash
#!/bin/bash
# Verificar espacio en disco
df -h /opt/erp-storage/ | awk 'NR>1 {
    if ($5+0 > 80) {
        print "ALERT: " $1 " is " $5 " full"
    }
}'

# Verificar performance de discos
iostat -x 1 1 | grep -E "(sda|nvme)" | awk '{
    if ($10 > 10) {
        print "ALERT: High latency on " $1 ": " $10 "ms"
    }
}'
```