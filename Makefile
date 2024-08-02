
#linux
CLIENT_PATH=$(PWD)/client
UI_PATH=$(PWD)/ui
COMMON_PATH=$(PWD)/common

LINUX_ROOT_DEBUG_PATH:=$(PWD)/target/debug
MINGW_TARGET_RELEASE=$(PWD)/target/x86_64-pc-windows-gnu/release
PROJECT_NAME="client"
RELEASE_ADDITIONAL_FILES=

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
	@make _deploy_win_all PROJECT_NAME=server_db

build_client:: copy_www_files 
	@cargo build -p client 

deploy_win_client::
	@make copy_www_files LINUX_ROOT_DEBUG_PATH=$(MINGW_TARGET_RELEASE)
	@make _deploy_win_all PROJECT_NAME=client RELEASE_ADDITIONAL_FILES=static/*

_deploy_win_all::
	@cargo build --target=x86_64-pc-windows-gnu -p $(PROJECT_NAME) --release
	@cd $(MINGW_TARGET_RELEASE); zip -r ~/Desktop/share/$(PROJECT_NAME).zip  $(PROJECT_NAME).exe $(RELEASE_ADDITIONAL_FILES)

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
	@echo the linux root path is:  $(LINUX_ROOT_DEBUG_PATH)
	@rm -rf $(LINUX_ROOT_DEBUG_PATH)/static
	@mkdir -p $(LINUX_ROOT_DEBUG_PATH)/static
	@cp -r $(PWD)/offline_files/www/* $(LINUX_ROOT_DEBUG_PATH)/static

config_ui::
	@cd $(PWD)/server/config_ui; make



