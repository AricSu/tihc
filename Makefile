# Variables
CARGO = cargo
PNPM = yarn
FRONTEND_DIR = frontend/shared
FRONTEND_DIR_DIST = $(FRONTEND_DIR)/dist
FRONTEND_SRC_DIR = $(FRONTEND_DIR)/src
EXTENSIONS_DIR = frontend/tihc-extension
EXT_SRC_DIR = $(EXTENSIONS_DIR)/src
EXT_ENTRYPOINTS_DIR = $(EXTENSIONS_DIR)/entrypoints
WORKSPACE_MEMBERS = cli microkernel web common plugins
BIN_DIR = bin
LOG_PATH = tihc.log

# Default target: build all
all: dashboard server 

# Build Rust backend (all workspace crates) and copy binary to bin/
server:
	@echo "\033[1;36m[BUILD] Building Rust backend ...\033[0m"
	rm -rf $(BIN_DIR)
	rm -rf $(LOG_PATH)
	$(CARGO) build --release -p tihc
	mkdir -p $(BIN_DIR)
	cp target/release/tihc $(BIN_DIR)/tihc
	@echo "\033[1;32m[SUCCESS] Rust backend built.\033[0m"

# Build frontend
dashboard:
	@echo "\033[1;36m[BUILD] Building frontend ...\033[0m"
	rm -rf $(FRONTEND_DIR_DIST)
	cd $(FRONTEND_DIR) && $(PNPM) install
	cd $(FRONTEND_DIR) && $(PNPM) build
	@echo "\033[1;32m[SUCCESS] Frontend built.\033[0m"


# Clean Rust project
clean-server:
	@echo "\033[1;33m[CLEAN] Cleaning Rust build ...\033[0m"
	$(CARGO) clean
	rm -rf $(BIN_DIR)

clean-dashboard:
	@echo "\033[1;33m[CLEAN] Cleaning frontend build ...\033[0m"
	cd $(FRONTEND_DIR) && $(PNPM) clean || rm -rf $(FRONTEND_DIR_DIST)

ext-dev:
	@echo "\033[1;36m[SYNC] Syncing frontend to extension ...\033[0m"
	rm -rf $(EXT_SRC_DIR)
	mkdir -p $(EXT_SRC_DIR)
	cp -r $(FRONTEND_SRC_DIR)/* $(EXT_SRC_DIR)/
	# 保留 entrypoints 目录结构，满足 WXT 构建要求
	cp -r $(EXT_ENTRYPOINTS_DIR) $(EXT_SRC_DIR)/
	@echo "\033[1;32m[SUCCESS] Frontend and entrypoints synced to extension.\033[0m"
	# 修改 defaultLayout 为 extension
	sed -i '' "s/export const defaultLayout = .*/export const defaultLayout = 'extension'/" $(EXT_SRC_DIR)/settings.js
	@echo "\033[1;36m[EXT] Building Chrome extension ...\033[0m"
	cd $(EXTENSIONS_DIR) && yarn install && yarn dev