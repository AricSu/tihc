// 应用服务：编排领域服务

use crate::domain::service::CausalityService;
use crate::infrastructure::metrics::fetch_metrics;
use anyhow::Result;
use tracing::info;

pub struct EvictLeaderAnalysisEngine {
    service: CausalityService,
}

impl EvictLeaderAnalysisEngine {
    pub fn new() -> Self {
        Self {
            service: CausalityService::new(),
        }
    }
    /// orchestrate: 调用领域服务和基础设施，完成分析流程
    pub async fn analyze(&mut self, request: &crate::domain::model::InspectionAnalysisRequest) -> Result<crate::domain::model::EvictLeaderResult> {
        info!("🔍 [CAUSALITY] Starting causal analysis for task: {}", request.task_id);
        let metrics = fetch_metrics(request).await?;
        // 只负责 orchestrate，具体推理和业务规则由 CausalityService 完成
        let result = self.service.analyze(metrics, request).await?;
        Ok(result)
    }
}
