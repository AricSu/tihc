use anyhow::Result;
use chrono;
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

use microkernel::platform::message_bus::{BusClient, BusMessage, Topic};
use microkernel::platform::plugin_manager::Plugin;

use crate::application::engine::EvictLeaderAnalysisEngine;
use crate::domain::model::{EvictLeaderResult, EvictRecommendation, HotStoreAnalysis, InspectionAnalysisRequest, LeaderDistribution};

/// 因果分析引擎插件实现
pub struct CausalityEnginePlugin {
    bus_client: Arc<BusClient>,
}

impl CausalityEnginePlugin {
    pub fn new() -> Self {
        Self {
            bus_client: Arc::new(BusClient::new()),
        }
    }

    pub fn start(&self) -> Result<()> {
        info!("🚀 [CAUSALITY_ENGINE] Evict Leader Analysis Engine started");
        Ok(())
    }
}

impl Clone for CausalityEnginePlugin {
    fn clone(&self) -> Self {
        Self {
            bus_client: Arc::clone(&self.bus_client),
        }
    }
}

/// Plugin trait 实现
#[derive(Debug, Default)]
pub struct CausalityEnginePluginDef;

impl CausalityEnginePluginDef {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Plugin for CausalityEnginePluginDef {
    fn name(&self) -> &str {
        "causality_engine"
    }

    fn description(&self) -> &str {
        "TiDB Evict Leader Analysis Engine"
    }

    fn on_shutdown(&self, _msg: &BusMessage) -> Result<()> {
        info!("🛑 [CAUSALITY_ENGINE] Shutting down...");
        Ok(())
    }

    fn topics(&self) -> Vec<String> {
        vec!["causality_engine".to_string()]
    }
}

/// 初始化因果分析引擎的消息处理器
pub fn init_causality_engine_message_handler() {
    info!("🧠 [CAUSALITY_ENGINE] Initializing Evict Leader Analysis Engine message handler...");

    let bus_client = BusClient::new();
    let topic = Topic::new("causality_engine", Some("inspection"));

    bus_client.register_request(topic, |msg| {
        info!("📥 [CAUSALITY_ENGINE] Received analysis request");

        // 示例字符串时间和时区
        let start_str = "2025-09-09 16:56:06";
        let end_str = "2025-09-09 17:10:22";
        let tz: chrono_tz::Tz = "Asia/Shanghai".parse().unwrap();

        // 字符串转时间戳辅助函数（直接用 Tz 类型）
        fn parse_datetime_with_tz(dt_str: &str, tz: chrono_tz::Tz) -> i64 {
            use chrono::NaiveDateTime;
            use chrono::TimeZone;
            let naive = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S").unwrap();
            tz.from_local_datetime(&naive).unwrap().timestamp()
        }

        let start_ts = parse_datetime_with_tz(start_str, tz);
        let end_ts = parse_datetime_with_tz(end_str, tz);

        // 从 msg.data 提取 clinic_url 字段
        let clinic_url = msg.data.get("clinic_url")
            .and_then(|v| v.as_str())
            .unwrap_or("").to_string();

        let request = InspectionAnalysisRequest {
            task_id: "mock_test".to_string(),
            clinic_url,
            start_ts,
            end_ts,
            timezone: tz,
        };

        info!(
            "🔍 [CAUSALITY_ENGINE] Processing evict leader analysis for task: {}",
            request.task_id
        );

        let mut engine = EvictLeaderAnalysisEngine::new();
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(engine.analyze(&request))
        }).unwrap_or_else(|_| create_default_evict_result(&request.task_id));

        info!(
            "✅ [CAUSALITY_ENGINE] Analysis completed - Evict: {}, Actions: {}",
            result.recommendation.should_evict,
            result.recommendation.actions.len()
        );

        // 发送状态更新
        send_status_update(&request.task_id);

        Ok(BusMessage::ok(
            Topic::new("causality_engine", Some("result")),
            serde_json::to_value(result).unwrap_or_default(),
        ))
    });

    info!("✅ [CAUSALITY_ENGINE] Evict Leader Analysis Engine message handler registered!");
}

/// 发送状态更新消息
fn send_status_update(task_id: &str) {
    let status_topic = Topic::new("inspection", Some("status_update"));
    let status_data = json!({
        "task_id": task_id,
        "status": "completed",
        "updated_at": chrono::Utc::now().timestamp()
    });

    let bus_client = BusClient::new();
    tokio::spawn(async move {
        if let Err(e) = bus_client.send_broadcast(status_topic, status_data).await {
            error!(
                "❌ [CAUSALITY_ENGINE] Failed to send status update: {:?}",
                e
            );
        } else {
            info!("📤 [CAUSALITY_ENGINE] Status update sent successfully");
        }
    });
}

/// 创建默认的分析结果
fn create_default_evict_result(task_id: &str) -> EvictLeaderResult {
    EvictLeaderResult {
        task_id: task_id.to_string(),
        leader_distribution: LeaderDistribution {
            store_counts: vec![],
            imbalance_percentage: 0.0,
            is_imbalanced: false,
        },
        hot_stores: HotStoreAnalysis {
            hot_stores: vec![],
            has_hot_stores: false,
        },
        recommendation: EvictRecommendation {
            should_evict: false,
            actions: vec![],
            confidence: 0.0,
        },
        confidence: 0.0,
        causal_explanation: None,
    }
}

/// 自动注册插件和消息处理器
#[ctor::ctor]
fn init_causality_engine_plugin() {
    println!("🧠 [CAUSALITY_ENGINE] Initializing Evict Leader Analysis Engine...");

    // 初始化消息处理器
    init_causality_engine_message_handler();

    println!("✅ [CAUSALITY_ENGINE] Evict Leader Analysis Engine registered!");
}
