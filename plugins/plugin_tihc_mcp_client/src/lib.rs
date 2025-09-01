pub mod plugin;


use serde_json::Value;
use reqwest::Client;

pub struct McpClient {
    pub endpoint: String,
    client: Client,
}

impl McpClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            client: Client::new(),
        }
    }

    /// 调用 get_info
    pub async fn get_info(&self) -> Result<Value, reqwest::Error> {
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "get_info",
            "params": null,
            "id": 1
        });
        let resp = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .json(&req)
            .send()
            .await?;
        let val: Value = resp.json().await?;
        Ok(val)
    }

    /// 通用 callTool
    pub async fn call_tool(&self, tool: &str, args: Value) -> Result<Value, reqwest::Error> {
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "callTool",
            "params": {"tool": tool, "args": args},
            "id": 2
        });
        let resp = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .json(&req)
            .send()
            .await?;
        let val: Value = resp.json().await?;
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_info() {
        // 需启动 MCP server 并监听 127.0.0.1:8080/mcp
        let client = McpClient::new("http://127.0.0.1:8080/mcp");
        let resp = client.get_info().await;
        println!("get_info resp: {:?}", resp);
        assert!(resp.is_ok());
    }
}
