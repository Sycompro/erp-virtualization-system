# Configuración del Servidor ERP Virtualization

## Especificaciones Mínimas del Servidor

### Hardware Recomendado:
- **CPU**: Intel Xeon o AMD EPYC (mínimo 16 cores)
- **RAM**: 64GB DDR4 (128GB recomendado)
- **Storage**: 2TB NVMe SSD + 10TB HDD para datos
- **GPU**: NVIDIA Tesla/Quadro para aplicaciones CAD/Design
- **Red**: 2x 10Gbps Ethernet (redundancia)

### Software Base:
- **OS**: Ubuntu Server 22.04 LTS o RHEL 9
- **Kubernetes**: v1.28+
- **Docker**: v24.0+
- **Istio Service Mesh**: v1.19+

## Estructura de Directorios

```
/opt/erp-virtualization/
├── containers/           # Imágenes de aplicaciones
│   ├── sap/
│   ├── office/
│   ├── autocad/
│   └── custom/
├── data/                # Datos persistentes
│   ├── user-profiles/
│   ├── shared-storage/
│   └── databases/
├── config/              # Configuraciones
│   ├── kubernetes/
│   ├── certificates/
│   └── networking/
└── logs/               # Logs del sistema
    ├── access/
    ├── security/
    └── performance/
```

## Configuración de Red Segura

### VLAN Segregation:
- **VLAN 10**: Management (192.168.10.0/24)
- **VLAN 20**: ERP Applications (192.168.20.0/24)
- **VLAN 30**: User Access (192.168.30.0/24)
- **VLAN 40**: Storage Network (192.168.40.0/24)

### Firewall Rules:
```bash
# Solo permitir HTTPS y WebRTC
iptables -A INPUT -p tcp --dport 443 -j ACCEPT
iptables -A INPUT -p udp --dport 3478:3479 -j ACCEPT  # STUN/TURN
iptables -A INPUT -p udp --dport 49152:65535 -j ACCEPT # WebRTC media
iptables -A INPUT -j DROP  # Denegar todo lo demás
```