
#linux
CLIENT_PATH=$(PWD)/client
COMMON_PATH=$(PWD)/common
LINUX_LIBS=$(CLIENT_PATH)/target/linux_libs
LINUX_RUST_UP_PATH=/home/mehran/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib
LINUX_DEBUG_PATH=$(LINUX_LIBS)/../debug
STD_LIB_SO:=libstd-fcf70ff533b679b2.so

STD_LIB_DLL:=std-02223d1a8ebc115e.dll

#windows
WIN_LIBS=$(CLIENT_PATH)/target/win_libs
WIN_RELEASE_PATH=$(WIN_LIBS)/../x86_64-pc-windows-gnu/release
WIN_RUST_UP_PATH=/home/mehran/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib
WIN_TARGET:= --target x86_64-pc-windows-gnu --release

PROJ_LIST:= $(PWD)/common $(PWD)/entity  $(PWD)/client

.PHONY: build test
################################
define func_build_cargo
	@for item in $1; do \
		cargo $2 $3 --manifest-path $$item/Cargo.toml ; \
	done
endef

define copy_libs
	@rm -rf $1
	@mkdir -p $1
	@cp "$2/$3" "$1/"
	@mkdir -p $4
	@cp $1/* $4
endef

###############################
## original status
run:: build test
	@echo "======================================================"
	@cd $(LINUX_DEBUG_PATH); cargo run --manifest-path $(CLIENT_PATH)/Cargo.toml

copy_linux_libs::
	$(call copy_libs,$(LINUX_LIBS),$(LINUX_RUST_UP_PATH),$(STD_LIB_SO),$(LINUX_DEBUG_PATH));

build:: copy_linux_libs
	$(call func_build_cargo,$(PROJ_LIST), build)

test::
	$(call func_build_cargo,$(PROJ_LIST), test)

clean::
	$(call func_build_cargo,$(PROJ_LIST), clean)
################################
# windows generator

copy_win_libs::
	$(call copy_libs,$(WIN_LIBS),$(WIN_RUST_UP_PATH),$(STD_LIB_DLL),$(WIN_RELEASE_PATH));

build_win:: copy_win_libs
	$(call func_build_cargo,$(PROJ_LIST), 'build', $(WIN_TARGET))
