
#linux
CLIENT_PATH=$(PWD)/client
UI_PATH=$(PWD)/ui
COMMON_PATH=$(PWD)/common

LINUX_ROOT_DEBUG_PATH:=$(PWD)/target/debug
MINGW_TARGET_RELEASE=$(PWD)/target/x86_64-pc-windows-gnu/release
PROJECT_NAME="client"
RELEASE_ADDITIONAL_FILES=
WIN_FLAGS=--target=x86_64-pc-windows-gnu

# ---------------------------------
# run programs from source code 

run_con_data::
	@cargo build -p con_data 
	@cd $(LINUX_ROOT_DEBUG_PATH); MED_WELL_ID=2345415781  MED_MONGO_CONN_STR="mongodb://localhost:27017/?connectTimeoutMS=30000&serverSelectionTimeoutMS=30000&heartbeatFrequencyMS=10000"   MED_AMQ_ADDRESS=tcp://localhost:61616 MED_REDIS_URL=redis://127.0.0.1:6379 ./con_data

run_client:: 
	@cargo build -p client 
	@cd $(LINUX_ROOT_DEBUG_PATH); ./client 
	
run_export_checker:: build_export_checker 
	@cd $(LINUX_ROOT_DEBUG_PATH); ./export_checker

run_win_ui:: 
	@cargo build -p ui 
	@cd $(LINUX_ROOT_DEBUG_PATH); ./ui

# ---------------------------------
# build

build_export_checker:: 
	@cargo build -p export_checker

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
	@cargo clean $(WIN_FLAGS) --release
	@cargo build $(WIN_FLAGS) -p $(PROJECT_NAME) --release
	@cd $(MINGW_TARGET_RELEASE);rm -rf ~/Desktop/share/$(PROJECT_NAME).zip  ;zip -r ~/Desktop/share/$(PROJECT_NAME).zip  $(PROJECT_NAME).exe $(RELEASE_ADDITIONAL_FILES)

build_win_export_checker::
	@cargo clean $(WIN_FLAGS) --release
	@cargo build $(WIN_FLAGS) -p export_checker --release

build_win_import_ui:: 
	@cargo clean $(WIN_FLAGS) --release 
	@cargo build $(WIN_FLAGS) -p import_ui --release 

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



