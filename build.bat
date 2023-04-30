cargo skyline package
rename target\aarch64-skyline-switch\release\libplugin.nro plugin.nro
move target\aarch64-skyline-switch\release\plugin.nro ssbu-rewritten
echo Press X to listen

@echo off
set "KEY=" & for /F "delims=" %%K in ('
    2^> nul xcopy /L /W /I "%~f0" "%TEMP%"

') do if not defined KEY set "KEY=%%K"
setlocal EnableDelayedExpansion
if /I "!KEY:~-1!"=="x" goto :MoreCode
endlocal & exit /B

:MoreCode
echo listening...
cargo skyline listen



