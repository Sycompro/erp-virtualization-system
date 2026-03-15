# Configuración de Dominios y DNS para ERP Virtualization

## 1. Estructura de Dominios

### Dominio Principal: `erp.company.com`
```
erp.company.com                 # Portal principal
├── api.erp.company.com        # API REST
├── stream.erp.company.com     # WebRTC streaming
├── admin.erp.company.com      # Panel administrativo
├── turn.erp.company.com       # TURN server para WebRTC
└── monitor.erp.company.com    # Monitoreo y métricas
```

### Subdominios por Tenant:
```
company-a.erp.com              # Empresa A
company-b.erp.com              # Empresa B
demo.erp.com                   # Ambiente de demostración
dev.erp.com                    # Desarrollo
```

## 2. Configuración DNS (BIND9)

### /etc/bind/named.conf.local
```bind
zone "erp.company.com" {
    type master;
    file "/etc/bind/zones/erp.company.com.db";
    allow-transfer { 192.168.1.10; };  # DNS secundario
};

zone "1.168.192.in-addr.arpa" {
    type master;
    file "/etc/bind/zones/192.168.1.rev";
};
```

### /etc/bind/zones/erp.company.com.db
```bind
$TTL    604800
@       IN      SOA     ns1.erp.company.com. admin.erp.company.com. (
                        2024031401      ; Serial
                        604800          ; Refresh
                        86400           ; Retry
                        2419200         ; Expire
                        604800 )        ; Negative Cache TTL

; Name servers
@       IN      NS      ns1.erp.company.com.
@       IN      NS      ns2.erp.company.com.

; A records
@               IN      A       192.168.1.100
ns1             IN      A       192.168.1.10
ns2             IN      A       192.168.1.11
api             IN      A       192.168.1.100
stream          IN      A       192.168.1.100
admin           IN      A       192.168.1.100
turn            IN      A       192.168.1.101
monitor         IN      A       192.168.1.102

; Load balancer records
erp             IN      A       192.168.1.100
erp             IN      A       192.168.1.101
erp             IN      A       192.168.1.102

; Tenant subdomains
company-a       IN      A       192.168.1.100
company-b       IN      A       192.168.1.100
demo            IN      A       192.168.1.100
dev             IN      A       192.168.1.103

; SRV records for WebRTC
_turn._tcp      IN      SRV     10 5 3478 turn.erp.company.com.
_turn._udp      IN      SRV     10 5 3478 turn.erp.company.com.
_stun._udp      IN      SRV     10 5 3478 turn.erp.company.com.

; TXT records for verification
@               IN      TXT     "v=spf1 include:_spf.google.com ~all"
_dmarc          IN      TXT     "v=DMARC1; p=quarantine; rua=mailto:dmarc@erp.company.com"
```

## 3. Certificados SSL/TLS

### Configuración Let's Encrypt
```bash
# Instalar certbot
apt install certbot python3-certbot-nginx

# Generar certificados wildcard
certbot certonly --manual --preferred-challenges=dns \
    -d "*.erp.company.com" -d "erp.company.com"

# Renovación automática
echo "0 12 * * * /usr/bin/certbot renew --quiet" | crontab -
```

### Configuración de Nginx (Load Balancer)
```nginx
# /etc/nginx/sites-available/erp.company.com
upstream erp_backend {
    least_conn;
    server 192.168.1.100:30080 max_fails=3 fail_timeout=30s;
    server 192.168.1.101:30080 max_fails=3 fail_timeout=30s;
    server 192.168.1.102:30080 max_fails=3 fail_timeout=30s;
}

upstream erp_streaming {
    ip_hash;  # Sticky sessions para WebRTC
    server 192.168.1.100:30443;
    server 192.168.1.101:30443;
    server 192.168.1.102:30443;
}

server {
    listen 80;
    server_name *.erp.company.com erp.company.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name erp.company.com;
    
    ssl_certificate /etc/letsencrypt/live/erp.company.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/erp.company.com/privkey.pem;
    
    # SSL Security
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512;
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    
    # Security headers
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload";
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    
    # API routes
    location /api/ {
        proxy_pass http://erp_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
    
    # WebSocket streaming
    location /stream/ {
        proxy_pass http://erp_streaming;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_read_timeout 86400;
    }
    
    # Static files
    location / {
        proxy_pass http://erp_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}

# Tenant-specific configurations
server {
    listen 443 ssl http2;
    server_name company-a.erp.com;
    
    ssl_certificate /etc/letsencrypt/live/erp.company.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/erp.company.com/privkey.pem;
    
    location / {
        proxy_pass http://erp_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Tenant-ID "company-a";
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 4. Firewall y Seguridad

### UFW Configuration
```bash
# Permitir SSH solo desde red interna
ufw allow from 192.168.1.0/24 to any port 22

# Permitir HTTP/HTTPS
ufw allow 80/tcp
ufw allow 443/tcp

# Permitir WebRTC/TURN
ufw allow 3478/tcp
ufw allow 3478/udp
ufw allow 49152:65535/udp

# Permitir Kubernetes
ufw allow from 192.168.0.0/16 to any port 6443
ufw allow from 192.168.0.0/16 to any port 2379:2380

# Denegar todo lo demás
ufw default deny incoming
ufw default allow outgoing
ufw enable
```

## 5. Monitoreo DNS

### Script de verificación DNS
```bash
#!/bin/bash
# /opt/scripts/check-dns.sh

DOMAINS=(
    "erp.company.com"
    "api.erp.company.com"
    "stream.erp.company.com"
    "company-a.erp.com"
)

for domain in "${DOMAINS[@]}"; do
    if ! nslookup $domain > /dev/null 2>&1; then
        echo "ERROR: DNS resolution failed for $domain"
        # Enviar alerta
        curl -X POST "https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK" \
            -H 'Content-type: application/json' \
            --data "{\"text\":\"DNS Alert: $domain resolution failed\"}"
    fi
done
```