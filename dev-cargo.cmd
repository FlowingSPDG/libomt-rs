@echo off
setlocal enableextensions

rem Initialize Visual Studio Developer Command Prompt (x64)
if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" (
    call "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64
) else if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat" (
    call "%ProgramFiles%\Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64
) else if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Enterprise\Common7\Tools\VsDevCmd.bat" (
    call "%ProgramFiles%\Microsoft Visual Studio\2022\Enterprise\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64
) else (
    echo [dev-cargo] ERROR: Could not locate VsDevCmd.bat for VS 2022.& echo Install Visual Studio Build Tools or update this script.& exit /b 1
)

rem Configure OpenMediaTransport paths
set "LIBOMT_LIB_DIR=C:\Users\Flowing\Downloads\OpenMediaTransport.Binaries.Release.v1.0.0.12\Libraries\Winx64"
set "PATH=%PATH%;C:\Users\Flowing\Downloads\OpenMediaTransport.Binaries.Release.v1.0.0.12\Libraries\Winx64"
rem If your .lib name differs (e.g. OpenMediaTransport.lib), uncomment and adjust:
rem set "LIBOMT_LIB_NAME=OpenMediaTransport"

rem Delegate to cargo with all original arguments
cargo %*

endlocal

