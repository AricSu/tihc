//! Data types for DDL lossy operation analysis

use serde::{Deserialize, Serialize};

/// Status of lossy operation detection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LossyStatus {
    /// Operation is confirmed safe - no data loss will occur
    Safe,
    /// Operation will cause data loss - confirmed lossy
    Lossy,
    /// Cannot determine if operation is lossy - requires manual review
    Unknown,
}

impl LossyStatus {
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            LossyStatus::Safe => "Safe - No data loss",
            LossyStatus::Lossy => "Lossy - Will cause data loss",
            LossyStatus::Unknown => "Unknown - Manual review required",
        }
    }
    
    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            LossyStatus::Safe => "✅",
            LossyStatus::Lossy => "⚠️",
            LossyStatus::Unknown => "❓",
        }
    }
    
    /// Check if this is a risky status (Lossy or Unknown)
    pub fn is_risky(&self) -> bool {
        matches!(self, LossyStatus::Lossy | LossyStatus::Unknown)
    }
}

impl Default for LossyStatus {
    fn default() -> Self {
        LossyStatus::Unknown
    }
}

impl std::fmt::Display for LossyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji(), self.description())
    }
}

/// Comprehensive result of DDL analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Status of lossy operation detection
    pub lossy_status: LossyStatus,
    
    /// Risk level of the operation
    pub risk_level: RiskLevel,
    
    /// Warning messages and recommendations
    pub warnings: Vec<String>,
    
    /// Error message if analysis failed
    pub error: Option<String>,
}

/// Risk level classification for DDL operations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Safe operations - confirmed no data loss risk
    /// Only assigned when analysis confirms the operation is definitely safe
    Safe,
    
    /// High risk operations - will cause permanent data loss or analysis failed
    /// Examples: DROP TABLE, DROP COLUMN, TRUNCATE TABLE, or when analysis cannot be completed
    High,
}

impl RiskLevel {
    /// Get a human-readable description of the risk level
    pub(crate) fn description(&self) -> &'static str {
        match self {
            RiskLevel::Safe => "Safe - Confirmed no stats loss risk",
            RiskLevel::High => "High - Will cause stats loss or requires manual review",
        }
    }
    
    /// Get emoji representation for UI display
    pub(crate) fn emoji(&self) -> &'static str {
        match self {
            RiskLevel::Safe => "✅",
            RiskLevel::High => "🔴",
        }
    }
}

impl Default for RiskLevel {
    fn default() -> Self {
        RiskLevel::High
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji(), self.description())
    }
}


