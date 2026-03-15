@echo off
echo ========================================
echo   Compilando APK RELEASE
echo   ERP Virtualization
echo ========================================
echo.

cd android

echo [1/4] Limpiando proyecto...
call gradlew.bat clean

echo.
echo [2/4] Compilando APK Release (optimizado)...
call gradlew.bat assembleRelease

echo.
echo [3/4] Verificando APK generado...
if exist "app\build\outputs\apk\release\app-release.apk" (
    echo.
    echo ========================================
    echo   APK RELEASE GENERADO EXITOSAMENTE
    echo ========================================
    echo.
    echo Ubicacion: android\app\build\outputs\apk\release\app-release.apk
    echo.
    
    echo [4/4] Copiando APK a la raiz del proyecto...
    copy "app\build\outputs\apk\release\app-release.apk" "..\erp-virtualization-release.apk"
    
    echo.
    echo APK copiado a: erp-virtualization-release.apk
    echo.
    echo NOTA: Este APK esta optimizado y minificado
    echo Tamaño reducido para produccion
    echo.
    echo Para instalar en tu dispositivo:
    echo   adb install erp-virtualization-release.apk
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
