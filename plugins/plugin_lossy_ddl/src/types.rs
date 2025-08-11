//! Data types for DDL lossy operation analysis

use serde::{Deserialize, Serialize};

/// Comprehensive result of DDL analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Whether the DDL operation may cause data loss
    pub is_lossy: bool,
    
    /// Risk level of the operation
    pub risk_level: RiskLevel,
    
    /// Warning messages and recommendations
    pub warnings: Vec<String>,
    
    /// Error message if analysis failed
    pub error: Option<String>,
    
    /// List of detected risky patterns
    pub analyzed_patterns: Vec<String>,
}

/// Legacy alias for backward compatibility
pub type PrecheckResult = AnalysisResult;

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
    /// Check if this risk level represents a lossy operation
    pub fn is_lossy(&self) -> bool {
        matches!(self, RiskLevel::High)
    }
    
    /// Get a human-readable description of the risk level
    pub fn description(&self) -> &'static str {
        match self {
            RiskLevel::Safe => "Safe - Confirmed no stats loss risk",
            RiskLevel::High => "High - Will cause stats loss or requires manual review",
        }
    }
    
    /// Get emoji representation for UI display
    pub fn emoji(&self) -> &'static str {
        match self {
            RiskLevel::Safe => "✅",
            RiskLevel::High => "🔴",
        }
    }
}

impl Default for RiskLevel {
    fn default() -> Self {
        RiskLevel::Safe
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji(), self.description())
    }
}


