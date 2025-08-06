# 变量定义
CARGO = cargo
PNPM = yarn
FRONTEND_DIR = frontend
FRONTEND_DIR_DIST = $(FRONTEND_DIR)/dist
WORKSPACE_MEMBERS = cli microkernel web common plugins
BIN_DIR = bin
LOG_PATH = tihc.log

# 默认目标：构建全部
all: dashboard server 

# 构建 Rust backend（workspace所有crate），并复制 binary 到 bin/
server:
	rm -rf $(BIN_DIR)
	rm -rf $(LOG_PATH)
	$(CARGO) build --release
	mkdir -p $(BIN_DIR)
	cp target/release/cli $(BIN_DIR)/tihc

# 构建前端
dashboard:
	rm -rf $(FRONTEND_DIR_DIST)
	cd $(FRONTEND_DIR) && $(PNPM) install
	cd $(FRONTEND_DIR) && $(PNPM) build

# 清理 Rust 项目
clean-server:
	$(CARGO) clean
	rm -rf $(BIN_DIR)

# 清理前端构建产物
clean-dashboard:
	cd $(FRONTEND_DIR) && $(PNPM) clean || rm -rf dist

# 全部清理
clean: clean-server clean-dashboard

.PHONY: all server dashboard clean clean-server clean-dashboard
