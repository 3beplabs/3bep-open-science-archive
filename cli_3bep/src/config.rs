// Configuracao do experimento (JSON -> struct)
// Suporta tanto .json quanto .bep (JSON com metadados academicos)

use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[allow(dead_code)] // Campos deserializados pelo serde para documentacao .bep
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
    /// Metadados academicos (opcional, usado em scripts .bep)
    #[serde(default)]
    pub metadata: Option<ScriptMetadata>,
    pub experiment_name: String,
    pub bodies: Vec<BodyConfig>,
    pub integrator: String,         // "rk4" ou "leapfrog"
    pub dt: String,                 // String para converter em I64F64
    pub steps: u64,
    /// Intervalo de exportacao da trajetoria (ex: 10 = salva a cada 10 passos).
    /// Se ausente, usa 1 (todo passo). Ignorado sem --trajectory.
    pub export_interval: Option<u64>,
}

impl ExperimentConfig {
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| format!("{}", e))
    }
}
