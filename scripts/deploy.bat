@echo off
SETLOCAL

:: ==============================
:: Configuración de variables
:: ==============================
SET "STELLAR_BIN=C:\Users\braya\.cargo\bin\stellar.exe"
SET "NETWORK=testnet"
SET "ALIAS=user_management"
SET "WASM_PATH=target\wasm32v1-none\release\user_management.wasm"
SET "SOURCE_ACCOUNT=GCOMVQUU4IVWUMKZYUCNZQYMCSJWME4YY3TMK4PV4JR6IUDTCWCT4TTY"

:: ==============================
:: Forzar certificados nativos
:: ==============================
SET RUSTLS_NATIVE_CACERTS=1

:: ==============================
:: Mostrar Stellar CLI y versión
:: ==============================
echo ==============================
echo Validando Stellar CLI...
echo ==============================
"%STELLAR_BIN%" --version
IF ERRORLEVEL 1 (
    echo ERROR: Stellar CLI no encontrado en %STELLAR_BIN%
    pause
    EXIT /B 1
)

:: ==============================
:: Fundar la cuenta en testnet (si no existe)
:: ==============================
echo ==============================
echo Fundando cuenta %SOURCE_ACCOUNT% en Testnet (si es necesario)...
echo ==============================
curl "https://friendbot.stellar.org?addr=%SOURCE_ACCOUNT%"
IF ERRORLEVEL 1 (
    echo ERROR: No se pudo fundar la cuenta en testnet.
    pause
    EXIT /B 1
)

:: ==============================
:: Desplegar contrato
:: ==============================
echo ==============================
echo Desplegando contrato %ALIAS%...
echo ==============================
"%STELLAR_BIN%" contract deploy ^
    --wasm %WASM_PATH% ^
    --source-account %SOURCE_ACCOUNT% ^
    --network %NETWORK% ^
    --alias %ALIAS%
IF ERRORLEVEL 1 (
    echo ERROR: No se pudo desplegar el contrato.
    pause
    EXIT /B 1
)

echo ==============================
echo Despliegue completado correctamente.
echo ==============================
pause
ENDLOCAL
