EXAMPLE_NAME ?= experimental
TARGET_DIR ?= ../target
LIB_FILE := lib$(EXAMPLE_NAME).a
TARGET_ARCH ?= aarch64-unknown-none

build:
	@mkdir -p $(TARGET_DIR)
	cd $(EXAMPLE_NAME) && cargo build --target $(TARGET_ARCH) --release
	cp $(EXAMPLE_NAME)/target/$(TARGET_ARCH)/release/$(LIB_FILE) $(TARGET_DIR)/$(LIB_FILE)
	@echo "Successfully built lib -> $(TARGET_DIR)/$(LIB_FILE)"
