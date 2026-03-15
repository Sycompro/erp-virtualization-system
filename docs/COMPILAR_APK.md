# 📱 Guía para Compilar el APK

## ✅ Estado del Proyecto

- ✅ Pantalla principal con autenticación biométrica
- ✅ Panel de configuración de visualización implementado
- ✅ Navegación entre pantallas
- ✅ Componentes UI personalizados (ERPCard, ERPButton, etc.)
- ✅ Integración con servidor Railway
- ✅ Build configurado correctamente

## 🚀 Compilar APK

### Opción 1: APK Debug (Desarrollo)
```cmd
build-apk.bat
```

Genera: `erp-virtualization.apk`

### Opción 2: APK Release (Producción)
```cmd
build-apk-release.bat
```

Genera: `erp-virtualization-release.apk` (optimizado y minificado)

## 📦 Instalación en Dispositivo

### Con ADB (Android Debug Bridge)
```cmd
adb install erp-virtualization.apk
```

### Manualmente
1. Copia el APK a tu dispositivo Android
2. Abre el archivo APK desde el explorador de archivos
3. Permite instalación de fuentes desconocidas si es necesario
4. Instala la aplicación

## 🎨 Pantallas Implementadas

### 1. Pantalla Principal (MainActivity)
- Autenticación biométrica
- Estados de conexión (Desconectado, Autenticando, Conectando, Conectado, Error)
- Visualización del stream ERP
- Controles de pantalla completa y desconexión

### 2. Pantalla de Configuración (SettingsScreen)
Accesible desde el ícono ⚙️ en la barra superior

#### Calidad de Video
- **Calidad**: Baja, Media, Alta, Ultra
- **Resolución**: 1280x720, 1920x1080, 2560x1440, 3840x2160
- **Tasa de Frames**: 30 FPS, 60 FPS, 120 FPS

#### Rendimiento
- **Aceleración por Hardware**: Usa GPU para decodificación
- **Modo Baja Latencia**: Reduce el retraso en transmisión

#### Conexión
- **Reconexión Automática**: Reconectar si se pierde conexión

#### Información del Sistema
- Versión: 1.0.0
- Protocolo: WebRTC
- Codec: H.264/VP9
- Cifrado: TLS 1.3 + mTLS

## 🔧 Requisitos

- Java JDK 17+
- Android SDK (API 26-35)
- Gradle 8.7+
- Dispositivo Android 8.0+ (API 26+)

## 📱 Características de la App

### Seguridad
- ✅ Autenticación biométrica (huella/facial)
- ✅ Conexión cifrada TLS 1.3
- ✅ mTLS para autenticación mutua
- ✅ Network Security Config

### UI/UX
- ✅ Material Design 3
- ✅ Tema corporativo personalizado
- ✅ Animaciones fluidas
- ✅ Componentes reutilizables
- ✅ Diseño responsive

### Conectividad
- ✅ Integración con Railway API
- ✅ Soporte para servidor local
- ✅ WebRTC para streaming
- ✅ Reconexión automática

## 🐛 Solución de Problemas

### Error: "Unresolved reference: libs"
✅ **SOLUCIONADO** - Se corrigió el build.gradle.kts

### Error de compilación
1. Limpia el proyecto:
```cmd
cd android
gradlew.bat clean
```

2. Sincroniza Gradle:
```cmd
gradlew.bat --refresh-dependencies
```

3. Recompila:
```cmd
gradlew.bat assembleDebug
```

### APK no se instala
- Verifica que el dispositivo tenga Android 8.0+
- Habilita "Instalar apps desconocidas" en Configuración
- Verifica espacio disponible (mínimo 50MB)

## 📊 Tamaño del APK

- **Debug**: ~15-20 MB
- **Release**: ~8-12 MB (minificado)

## 🔄 Próximos Pasos

Para probar la app completa:

1. Compila el APK con `build-apk.bat`
2. Instala en tu dispositivo
3. Asegúrate de que el servidor Railway esté activo
4. Abre la app y prueba la autenticación biométrica
5. Accede a Configuración (⚙️) para ajustar la visualización
6. Conecta al ERP y prueba el streaming

## 📝 Notas

- El APK está configurado para conectarse a Railway por defecto
- URL Railway: `https://erp-virtualization-system-production.up.railway.app/`
- URL Local alternativa: `http://192.168.1.100:8081/`
- La configuración se puede cambiar desde la pantalla de Settings

---

**¿Listo para compilar?** Ejecuta `build-apk.bat` y en unos minutos tendrás tu APK listo para instalar. 🚀
