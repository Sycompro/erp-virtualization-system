# 🚀 Guía de Inicio Rápido - ERP Virtualization

## ⚡ Configuración en 5 Minutos

### 1. Preparar el Entorno
```bash
# Clonar el proyecto
git clone <tu-repositorio>
cd erp-virtualization

# Ejecutar configuración automática
./dev-setup.sh
```

### 2. Iniciar el Sistema
```bash
# Iniciar servicios base
./start-dev.sh

# Iniciar aplicación SAP
docker-compose -f docker-compose.dev.yml --profile sap up -d

# Iniciar aplicación Office
docker-compose -f docker-compose.dev.yml --profile office up -d
```

### 3. Verificar que Todo Funciona
```bash
# Verificar servicios
docker ps

# Ver logs
./logs-dev.sh postgres
./logs-dev.sh sap-dev
```

## 🌐 Acceder a las Aplicaciones

### SAP GUI (VNC)
- **URL Web**: http://localhost:6080
- **VNC Directo**: localhost:5900
- **Password**: dev_sap_vnc

### Microsoft Office (RDP)
- **Servidor**: localhost:3389
- **Usuario**: dev_user
- **Password**: dev_office_123

### Base de Datos
- **Host**: localhost:5432
- **Database**: erp_db
- **Usuario**: erp_user
- **Password**: dev_password

## 📱 Probar con Android

### 1. Construir APK
```bash
# Si tienes Android SDK configurado
./build.sh --android-only
```

### 2. Instalar en Tablet
```bash
# Conectar tablet por USB y habilitar depuración
adb install android/app/build/outputs/apk/debug/app-debug.apk
```

### 3. Configurar App
1. Abrir app en tablet
2. Configurar servidor: `http://tu-ip-local`
3. Usuario: `tablet1` / Password: `admin123`
4. Probar autenticación biométrica

## 🔧 Desarrollo del Servidor Rust

### 1. Instalar Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Ejecutar en Modo Desarrollo
```bash
cd server
cargo run
```

### 3. Probar API
```bash
# Health check
curl http://localhost:8080/health

# Listar aplicaciones
curl http://localhost:8080/applications/list

# Login (mock)
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123","deviceId":"test"}'
```

## 🐛 Solución de Problemas Comunes

### Error: "Docker no encontrado"
```bash
# Ubuntu/Debian
sudo apt update && sudo apt install docker.io docker-compose

# Windows: Instalar Docker Desktop
# https://docs.docker.com/desktop/windows/install/
```

### Error: "Puerto ya en uso"
```bash
# Ver qué está usando el puerto
netstat -tulpn | grep :5432

# Detener servicios conflictivos
sudo systemctl stop postgresql
```

### Error: "Imagen no encontrada"
```bash
# Reconstruir imágenes
docker-compose -f docker-compose.dev.yml build --no-cache
```

### SAP VNC no se conecta
```bash
# Verificar que el container está corriendo
docker ps | grep sap

# Ver logs del container
docker logs sap-gui-dev

# Reiniciar container
docker restart sap-gui-dev
```

### Office RDP no funciona
```bash
# Verificar puerto RDP
telnet localhost 3389

# Reiniciar container Office
docker restart office-suite-dev

# Verificar logs
docker logs office-suite-dev
```

## 📊 Monitoreo y Logs

### Ver Estado del Sistema
```bash
# Estado de todos los containers
docker ps -a

# Uso de recursos
docker stats

# Logs en tiempo real
./logs-dev.sh [servicio]
```

### Acceder a Containers
```bash
# Entrar al container SAP
docker exec -it sap-gui-dev bash

# Entrar al container Office
docker exec -it office-suite-dev bash

# Entrar a PostgreSQL
docker exec -it erp-postgres-dev psql -U erp_user -d erp_db
```

## 🧹 Limpiar Entorno

### Detener Todo
```bash
./stop-dev.sh
```

### Limpiar Completamente
```bash
./clean-dev.sh
```

### Reset Total
```bash
# Eliminar todos los datos (¡CUIDADO!)
rm -rf data/
./dev-setup.sh
```

## 🚀 Próximos Pasos

### Para Desarrollo
1. Modificar código Rust en `server/src/`
2. Actualizar app Android en `android/src/`
3. Agregar nuevas aplicaciones en `containers/`

### Para Producción
1. Ejecutar `./build.sh --package`
2. Seguir guía en `infrastructure/setup-scripts/install.sh`
3. Configurar dominio y SSL real

### Para Escalabilidad
1. Migrar a Kubernetes con `k8s/`
2. Configurar load balancer
3. Implementar auto-scaling

## 📞 Soporte

### Logs Importantes
- **Servidor**: `./logs/erp/`
- **Nginx**: `./logs/nginx/`
- **Containers**: `docker logs [container-name]`

### Archivos de Configuración
- **Docker**: `docker-compose.dev.yml`
- **Nginx**: `nginx/nginx.conf`
- **Base de datos**: `database/init.sql`
- **Entorno**: `.env.dev`

### Comandos Útiles
```bash
# Ver todas las imágenes
docker images | grep erp-virtualization

# Limpiar imágenes no usadas
docker image prune -f

# Ver uso de espacio
du -sh data/

# Backup de base de datos
docker exec erp-postgres-dev pg_dump -U erp_user erp_db > backup.sql
```

---

**¿Problemas?** Revisa los logs con `./logs-dev.sh [servicio]` o abre un issue en GitHub.