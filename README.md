# 🏢 ERP Virtualization System

Sistema de virtualización de aplicaciones de escritorio, similar a Parallels Access. Permite a clientes acceder a aplicaciones como SAP, Office o AutoCAD de forma remota desde el navegador o una app Android.

## 🎯 Arquitectura

```
☁️ Railway (API + DB)  ←→  🖥️ cPanel (PC Host)  ←→  📱 App / 🌐 Web (Clientes)
```

| Componente | Ubicación | Función |
|---|---|---|
| **railway-api/** | Nube (Railway) | PostgreSQL + API REST (auth, usuarios, config) |
| **cpanel/** | PC del dueño | Configura apps, da acceso, transmite escritorios vía WebRTC |
| **web-client/** | Navegador del cliente | Interfaz web para usar las apps virtualizadas |
| **android/** | Celular/Tablet | App nativa para usar las apps virtualizadas |

## 📂 Estructura del Proyecto

```
├── railway-api/          # API + DB en Railway (Rust/Axum)
│   ├── src/              # Código fuente del backend
│   ├── database/         # Schema PostgreSQL e init.sql
│   ├── Dockerfile
│   └── railway.json
│
├── cpanel/               # Panel de Control (PC Host)
│   ├── src/              # Servidor local (Rust) - containers + streaming
│   ├── panel/            # Interfaz web del admin panel
│   ├── containers/       # Imágenes Docker (SAP, Office, etc.)
│   └── Dockerfile
│
├── web-client/           # Cliente Web para usuarios finales
│   ├── index.html        # Página principal
│   ├── css/              # Estilos
│   └── js/               # Auth, WebRTC, lógica principal
│
├── android/              # App Android (Kotlin + Compose)
│   └── app/src/
│
├── infrastructure/       # Docker Compose, Nginx, Kubernetes
├── docs/                 # Documentación
└── scripts/              # Scripts de build, deploy, setup
```

## 🚀 Inicio Rápido

### Railway API
```bash
cd railway-api
railway init
railway add postgresql
railway up
```

### cPanel (PC Host)
```bash
cd cpanel
cargo run
# Abrir panel: http://localhost:8080
```

### Web Client
```bash
# Abrir directamente en el navegador
start web-client/index.html
```

### App Android
```bash
cd android
./gradlew assembleDebug
```

## 🔑 Flujo de Uso

1. El **admin** instala el cPanel en su PC y configura qué apps compartir
2. El cPanel se registra en **Railway** con las apps disponibles
3. Un **cliente** abre la **web** o la **app** → se autentica → ve las apps
4. El cliente selecciona una app → el cPanel inicia el contenedor → transmite por **WebRTC**

## 📄 Licencia

Proyecto propietario - Todos los derechos reservados