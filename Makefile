# Variables
CARGO = cargo
PNPM = yarn
FRONTEND_DIR = frontend
FRONTEND_DIR_DIST = $(FRONTEND_DIR)/dist
WORKSPACE_MEMBERS = cli microkernel web common plugins
BIN_DIR = bin
LOG_PATH = tihc.log

# Default target: build all
all: plugin-go dashboard server

# Build plugin_lossy_ddl go c-archive
plugin-go:
	@echo "\033[1;36m[BUILD] Compiling plugin_lossy_ddl/go/libschematracker.a ...\033[0m"
	rm -f plugins/plugin_lossy_ddl/go/libschematracker.a plugins/plugin_lossy_ddl/go/libschematracker.h
	cd plugins/plugin_lossy_ddl/go && go build -buildmode=c-archive -o libschematracker.a libschematracker.go
	@echo "\033[1;32m[SUCCESS] plugin_lossy_ddl/go/libschematracker.a built.\033[0m"

# Build Rust backend (all workspace crates) and copy binary to bin/
server: plugin-go
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

# Clean frontend build
clean-dashboard:
	@echo "\033[1;33m[CLEAN] Cleaning frontend build ...\033[0m"
	cd $(FRONTEND_DIR) && $(PNPM) clean || rm -rf dist

# Clean plugin_lossy_ddl-go artifacts
clean-plugin-go:
	@echo "\033[1;33m[CLEAN] Cleaning plugin_lossy_ddl/go artifacts ...\033[0m"
	rm -f plugins/plugin_lossy_ddl/go/libschematracker.a plugins/plugin_lossy_ddl/go/libschematracker.h

# Clean all
clean: clean-server clean-dashboard clean-plugin-go

.PHONY: all server dashboard clean clean-server clean-dashboard plugin-go clean-plugin-go