// Experiment configuration (JSON -> struct)
// Supports both .json and .bep (JSON with academic metadata)

use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[allow(dead_code)] // Fields deserialized by serde for .bep documentation
pub struct ScriptMetadata {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub references: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub expected_claims: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct BodyConfig {
    pub mass: f64,
    pub pos: [f64; 3],
    pub vel: [f64; 3],
}

#[derive(Deserialize, Debug)]
pub struct ExperimentConfig {
    /// Academic metadata (optional, used in .bep scripts)
    #[serde(default)]
    pub metadata: Option<ScriptMetadata>,
    pub experiment_name: String,
    pub bodies: Vec<BodyConfig>,
    pub integrator: String,         // "rk4" or "leapfrog"
    pub dt: String,                 // String to convert into I64F64
    pub steps: u64,
    /// Trajectory export interval (e.g. 10 = save every 10 steps).
    /// If absent, defaults to 1 (every step). Ignored without --trajectory.
    pub export_interval: Option<u64>,
}

impl ExperimentConfig {
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| format!("{}", e))
    }
}
