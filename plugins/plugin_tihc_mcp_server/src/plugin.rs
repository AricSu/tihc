/// 自动注册统一的 MCP handler 到消息总线
/// 这个函数只是标记插件已加载，实际注册会通过消息总线的"register"消息触发
#[ctor::ctor]
fn init_mcp_plugin() {
    // 只标记插件已加载，真正的注册会在backend发送"register"消息时进行
    tracing::info!("MCP plugin loaded - waiting for registration trigger from backend");
}
