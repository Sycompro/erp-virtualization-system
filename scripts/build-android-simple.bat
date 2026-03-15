@echo off
echo 🚀 Construyendo APK Simple de ERP Virtualization
echo.

REM Configurar JAVA_HOME
set JAVA_HOME=C:\Program Files\Eclipse Adoptium\jdk-17.0.17.10-hotspot

echo ☕ Java configurado: %JAVA_HOME%
echo.

REM Ir al directorio android
cd android

echo 📱 Construyendo APK...
echo.

REM Limpiar proyecto
call gradlew.bat clean --no-configuration-cache

REM Construir APK debug
call gradlew.bat assembleDebug --no-configuration-cache --stacktrace

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ APK construido exitosamente!
    echo 📍 Ubicación: android\app\build\outputs\apk\debug\app-debug.apk
    echo.
    echo 📱 Para instalar en tablet:
    echo    adb install app\build\outputs\apk\debug\app-debug.apk
    echo.
) else (
    echo.
    echo ❌ Error construyendo APK
    echo 💡 Revisa los errores arriba para más detalles
    echo.
)

pause