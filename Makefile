# Variables
CARGO = cargo
PNPM = yarn
FRONTEND_DIR = frontend
FRONTEND_DIR_DIST = $(FRONTEND_DIR)/dist
EXTENSIONS_DIR = extensions/tihc-jira-extension
WORKSPACE_MEMBERS = cli microkernel web common plugins
BIN_DIR = bin
LOG_PATH = tihc.log

# Default target: build all
all: server

# Build Rust backend (all workspace crates) and copy binary to bin/
server: 
	@echo "\033[1;36m[BUILD] Building Rust backend ...\033[0m"
	rm -rf $(BIN_DIR)
	rm -rf $(LOG_PATH)
	$(CARGO) build --release -p tihc
	mkdir -p $(BIN_DIR)
	cp target/release/tihc $(BIN_DIR)/tihc
	@echo "\033[1;32m[SUCCESS] Rust backend built.\033[0m"

# Clean Rust project
clean-server:
	@echo "\033[1;33m[CLEAN] Cleaning Rust build ...\033[0m"
	$(CARGO) clean
	rm -rf $(BIN_DIR)