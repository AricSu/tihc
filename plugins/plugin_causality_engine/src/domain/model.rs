// 领域对象定义
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionAnalysisRequest {
    pub task_id: String,
    pub clinic_url: String,
    pub start_ts: i64,
    pub end_ts: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub timezone: Tz,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvictLeaderResult {
    pub task_id: String,
    pub leader_distribution: LeaderDistribution,
    pub hot_stores: HotStoreAnalysis,
    pub recommendation: EvictRecommendation,
    pub confidence: f64,
    pub causal_explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderDistribution {
    pub store_counts: Vec<(u32, String)>,
    pub imbalance_percentage: f64,
    pub is_imbalanced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotStoreAnalysis {
    pub hot_stores: Vec<HotStore>,
    pub has_hot_stores: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotStore {
    pub store_id: String,
    pub cpu_usage: f64,
    pub heat_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvictRecommendation {
    pub should_evict: bool,
    pub actions: Vec<EvictAction>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvictAction {
    pub action_type: String,
    pub target_store: String,
    pub reason: String,
    pub priority: u8,
}
