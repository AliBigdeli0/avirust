
#linux
CLIENT_PATH=$(PWD)/client
UI_PATH=$(PWD)/ui
COMMON_PATH=$(PWD)/common

LINUX_ROOT_DEBUG_PATH=$(PWD)/target/debug

build_client:: copy_www_files 
	@cargo build -p client 

build_win_ui:: copy_opengl_file
	@cargo build -p ui

build_all:: copy_linux_libs copy_www_files
	@RUSTFLAGS='-C prefer-dynamic' cargo test
	@RUSTFLAGS='-C prefer-dynamic' cargo build

copy_opengl_file:: 
	@mkdir -p $(LINUX_ROOT_DEBUG_PATH)
	@cp -r $(PWD)/offline_files/opengl/opengl32.dll $(LINUX_ROOT_DEBUG_PATH)

copy_www_files::
	@rm -rf $(LINUX_ROOT_DEBUG_PATH)/static
	@mkdir -p $(LINUX_ROOT_DEBUG_PATH)/static
	@cp -r $(PWD)/offline_files/www/* $(LINUX_ROOT_DEBUG_PATH)/static



