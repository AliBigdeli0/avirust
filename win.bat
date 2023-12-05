@echo off

set PROJ_PATH=%CD%

set COMMON_PROJ=%PROJ_PATH%\common
set ENTITY_PROJ=%PROJ_PATH%\entity
set CONFIG_PROJ=%PROJ_PATH%\config
set LOGGER_PROJ=%PROJ_PATH%\logger
set WEB_PROJ=%PROJ_PATH%\web
set DB_PROJ=%PROJ_PATH%\db
set CLIENT_PROJ=%PROJ_PATH%\client
set UI_PROJ=%PROJ_PATH%\ui
set OFFLINE_FILES=%PROJ_PATH%\offline_files

set FLAGS=--profile win

call %PROJ_PATH%\bats\per_actions.bat

set ACTION=test
call %PROJ_PATH%\bats\cargo_actions.bat 

set ACTION=build
call %PROJ_PATH%\bats\cargo_actions.bat 

echo DONE!