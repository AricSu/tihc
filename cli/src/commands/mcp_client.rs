use clap::Args;

#[derive(Args, Debug)]
pub struct McpOptions {
	/// MCP 调用模式: get_info 或 call_tool
	#[clap(long, default_value = "get_info", value_parser = ["get_info", "call_tool"])]
	pub mode: String,
	/// MCP 服务端 endpoint
	#[clap(long, default_value = "http://127.0.0.1:8080/mcp")]
	pub endpoint: String,
	/// MCP 工具名（仅 call_tool 模式需要）
	#[clap(long, default_value = "")]
	pub tool: String,
	/// MCP 工具参数（仅 call_tool 模式需要，JSON 字符串）
	#[clap(long, default_value = "{}")]
	pub args: String,
}
