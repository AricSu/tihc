# Variables
CARGO = cargo
PNPM = yarn
FRONTEND_DIR = frontend/shared
FRONTEND_DIR_DIST = $(FRONTEND_DIR)/dist
EXTENSIONS_DIR = extensions/tihc-jira-extension
WORKSPACE_MEMBERS = cli microkernel web common plugins
BIN_DIR = bin
LOG_PATH = tihc.log

# Default target: build all
all: frontend server 

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
frontend:
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

clean-frontend:
	@echo "\033[1;33m[CLEAN] Cleaning frontend build ...\033[0m"
	cd $(FRONTEND_DIR) && $(PNPM) clean || rm -rf $(FRONTEND_DIR_DIST)
