RUST_LIB_DIR := fbsd-kdev
TARGET_DIR := target
LIB_FILE := libfbsd_kdev.a
TARGET_ARCH := aarch64-unknown-none


build:
	@mkdir -p $(TARGET_DIR)
	cd $(RUST_LIB_DIR) && cargo build --target $(TARGET_ARCH) --release
	cp $(RUST_LIB_DIR)/target/$(TARGET_ARCH)/release/$(LIB_FILE) $(TARGET_DIR)/$(LIB_FILE)
	@echo "Successfully built lib -> $(TARGET_DIR)/$(LIB_FILE)"
