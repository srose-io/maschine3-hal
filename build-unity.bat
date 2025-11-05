@echo off
REM Build script for Unity integration on Windows

echo Building Maschine MK3 HAL for Unity...

REM Build release version
cargo build --release
if %errorlevel% neq 0 exit /b %errorlevel%

set LIB_NAME=maschine3_hal.dll
set LIB_PATH=target\release\%LIB_NAME%

echo Built: %LIB_PATH%

REM Check if Unity project path is provided
if "%~1"=="" (
    echo.
    echo To copy to Unity project automatically, run:
    echo   build-unity.bat C:\path\to\UnityProject
    echo.
    echo Or manually copy:
    echo   %LIB_PATH% -^> UnityProject\Assets\Plugins\x86_64\
    echo   unity\MaschineMK3Native.cs -^> UnityProject\Assets\Plugins\
    echo   unity\MaschineExampleController.cs -^> UnityProject\Assets\Scripts\
    exit /b 0
)

set UNITY_PROJECT=%~1
set PLUGINS_DIR=%UNITY_PROJECT%\Assets\Plugins\x86_64

echo Copying to Unity project: %UNITY_PROJECT%

REM Create plugins directory if it doesn't exist
if not exist "%PLUGINS_DIR%" mkdir "%PLUGINS_DIR%"

REM Copy library
copy /Y "%LIB_PATH%" "%PLUGINS_DIR%\"

REM Copy C# scripts if they don't exist
if not exist "%UNITY_PROJECT%\Assets\Plugins\MaschineMK3Native.cs" (
    if not exist "%UNITY_PROJECT%\Assets\Plugins" mkdir "%UNITY_PROJECT%\Assets\Plugins"
    copy unity\MaschineMK3Native.cs "%UNITY_PROJECT%\Assets\Plugins\"
    echo Copied MaschineMK3Native.cs
)

if not exist "%UNITY_PROJECT%\Assets\Scripts\MaschineExampleController.cs" (
    if not exist "%UNITY_PROJECT%\Assets\Scripts" mkdir "%UNITY_PROJECT%\Assets\Scripts"
    copy unity\MaschineExampleController.cs "%UNITY_PROJECT%\Assets\Scripts\"
    echo Copied MaschineExampleController.cs
)

echo Unity integration complete!
echo Library: %PLUGINS_DIR%\%LIB_NAME%
