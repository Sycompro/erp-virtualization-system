@echo off
echo ========================================
echo   Compilando APK - ERP Virtualization
echo ========================================
echo.

cd android

echo [1/4] Limpiando proyecto...
call gradlew.bat clean

echo.
echo [2/4] Compilando proyecto...
call gradlew.bat assembleDebug

echo.
echo [3/4] Verificando APK generado...
if exist "app\build\outputs\apk\debug\app-debug.apk" (
    echo.
    echo ========================================
    echo   APK GENERADO EXITOSAMENTE
    echo ========================================
    echo.
    echo Ubicacion: android\app\build\outputs\apk\debug\app-debug.apk
    echo.
    
    echo [4/4] Copiando APK a la raiz del proyecto...
    copy "app\build\outputs\apk\debug\app-debug.apk" "..\erp-virtualization.apk"
    
    echo.
    echo APK copiado a: erp-virtualization.apk
    echo.
    echo Para instalar en tu dispositivo:
    echo   adb install erp-virtualization.apk
    echo.
) else (
    echo.
    echo ========================================
    echo   ERROR: No se pudo generar el APK
    echo ========================================
    echo.
    echo Revisa los errores arriba.
)

cd ..
pause
