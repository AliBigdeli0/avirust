
#linux
CLIENT_PATH=$(PWD)/client
UI_PATH=$(PWD)/ui
COMMON_PATH=$(PWD)/common

LINUX_ROOT_DEBUG_PATH=$(PWD)/target/debug
MINGW_TARGET_RELEASE=$(PWD)/target/x86_64-pc-windows-gnu/release

# ---------------------------------
# run programs from source code 
run_win_ui:: 
	@cargo build -p ui 
	@cd $(LINUX_ROOT_DEBUG_PATH); ./ui

# ---------------------------------
# build
build_server_db: 
	@cargo build -p server_db 
	@cd $(LINUX_ROOT_DEBUG_PATH); ./server_db

deploy_win_server_db: 
	@cargo build --target=x86_64-pc-windows-gnu -p server_db --release
	@cd $(MINGW_TARGET_RELEASE); zip ~/Desktop/share/server_db.zip  server_db.exe

build_client:: copy_www_files 
	@cargo build -p client 

deploy_win_client: 
	@cargo build --target=x86_64-pc-windows-gnu -p client --release
	@cd $(MINGW_TARGET_RELEASE); zip ~/Desktop/share/client.zip  client.exe

build_win_ui:: copy_opengl_file
	@cargo build -p ui

build_all:: copy_www_files test_all
	@cargo build --all

test_all::
	@cargo test --all

# --------------------------------
# copy opengl
copy_opengl_file:: 
	@mkdir -p $(LINUX_ROOT_DEBUG_PATH)
	@cp -r $(PWD)/offline_files/opengl/opengl32.dll $(LINUX_ROOT_DEBUG_PATH)

copy_www_files::
	@rm -rf $(LINUX_ROOT_DEBUG_PATH)/static
	@mkdir -p $(LINUX_ROOT_DEBUG_PATH)/static
	@cp -r $(PWD)/offline_files/www/* $(LINUX_ROOT_DEBUG_PATH)/static

config_ui::
	@cd $(PWD)/server/config_ui; make



