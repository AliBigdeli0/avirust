@echo  off

cd %COMMON_PROJ%
cargo %ACTION% %FLAGS% 
if %errorlevel% neq 0 (
  exit /b 1
)


cd %ENTITY_PROJ%
cargo %ACTION% %FLAGS% 
if %errorlevel% neq 0 (
  exit /b 1
)

cd %CONFIG_PROJ%
cargo %ACTION% %FLAGS% 
if %errorlevel% neq 0 (
  exit /b 1
)

cd %LOGGER_PROJ%
cargo %ACTION% %FLAGS% 
if %errorlevel% neq 0 (
  exit /b 1
)

cd %DB_PROJ%
cargo %ACTION% %FLAGS% 
if %errorlevel% neq 0 (
  exit /b 1
)

cd %WEB_PROJ%
cargo %ACTION% %FLAGS% 
if %errorlevel% neq 0 (
  exit /b 1
)

cd %CLIENT_PROJ%
cargo %ACTION% %FLAGS% 
if %errorlevel% neq 0 (
  exit /b 1
)

cd %UI_PROJ%
cargo %ACTION% %FLAGS% 
if %errorlevel% neq 0 (
  exit /b 1
)


