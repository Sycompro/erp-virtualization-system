@echo off
echo ========================================
echo   Sincronizando Gradle
echo ========================================
echo.

cd android

echo [1/2] Limpiando cache de Gradle...
call gradlew.bat clean --refresh-dependencies

echo.
echo [2/2] Verificando configuracion...
call gradlew.bat tasks

echo.
echo ========================================
echo   Sincronizacion completada
echo ========================================
echo.
echo Ahora puedes compilar el APK con:
echo   build-apk.bat
echo.

cd ..
pause
