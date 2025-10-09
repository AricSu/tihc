
// 接口层：插件入口与消息分发
use crate::application::engine::EvictLeaderAnalysisEngine;
use crate::domain::model::InspectionAnalysisRequest;
use anyhow::Result;

pub struct PluginApi {
	engine: EvictLeaderAnalysisEngine,
}

impl PluginApi {
	pub fn new() -> Self {
		Self {
			engine: EvictLeaderAnalysisEngine::new(),
		}
	}

	/// 消息分发与 API 入口，调用 application 层 orchestrate
	pub async fn handle_analysis(&mut self, req: InspectionAnalysisRequest) -> Result<crate::domain::model::EvictLeaderResult> {
		self.engine.analyze(&req).await
	}
}
