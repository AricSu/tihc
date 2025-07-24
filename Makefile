# 变量定义
CARGO = cargo
PNPM = yarn
FRONTEND_DIR = frontend
WORKSPACE_MEMBERS = cli core web common plugins
BIN_DIR = bin

# 默认目标：构建全部
all: server dashboard

# 构建 Rust backend（workspace所有crate），并复制 binary 到 bin/
server:
	rm -rf $(BIN_DIR)
	$(CARGO) build --release
	mkdir -p $(BIN_DIR)
	cp target/release/cli $(BIN_DIR)/tihc

# 构建前端
dashboard:
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
