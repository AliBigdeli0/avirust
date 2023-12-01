@echo on

set CLIENT_TARGET=%CLIENT_PROJ%\target\win

COPY  %HOMEDRIVE%%HOMEPATH%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\std-4ee9ee8805e6ac55.dll %CLIENT_TARGET%
COPY  %HOMEDRIVE%%HOMEPATH%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\std-4ee9ee8805e6ac55.pdb  %CLIENT_TARGET%

XCOPY %CLIENT_PROJ%\static %CLIENT_TARGET%\static  /E /I /Y