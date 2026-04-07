// Deterministic Reproducibility SVG Certificate
//
// Generates an inline visual seal (pure SVG, zero external crates) that
// researchers can include in PDF papers as proof that
// the simulation was verified by the Sanctuary I64F64 engine.
//
// Hash: SHA-256 (FIPS 180-4, pure Rust)
// Contains: engine version, input hash, output hash,
// integrator, number of steps, and UTC timestamp.

use crate::config::ExperimentConfig;
use crate::report;
use crate::runner::SimulationResult;
use crate::sha256;

/// Generates SHA-256 of the JSON input content (cryptographic fingerprint of initial conditions)
fn hash_input(json_str: &str) -> String {
    sha256::sha256_hex(json_str.as_bytes())
}

/// Generates manual UTC timestamp (no external crate, just SystemTime)
/// civil_from_days algorithm: Howard Hinnant (2016), public domain
fn utc_timestamp() -> String {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => {
            let secs = d.as_secs();
            let days = (secs / 86400) as i64;
            let time_of_day = secs % 86400;
            let hours = time_of_day / 3600;
            let minutes = (time_of_day % 3600) / 60;
            let seconds = time_of_day % 60;

            let z = days + 719468;
            let era = if z >= 0 { z } else { z - 146096 } / 146097;
            let doe = (z - era * 146097) as u64;
            let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365;
            let y = yoe as i64 + era * 400;
            let doy = doe - (365*yoe + yoe/4 - yoe/100);
            let mp = (5*doy + 2) / 153;
            let d = doy - (153*mp + 2)/5 + 1;
            let m = if mp < 10 { mp + 3 } else { mp - 9 };
            let year = if m <= 2 { y + 1 } else { y };

            format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, m, d, hours, minutes, seconds)
        }
        Err(_) => "0000-00-00T00:00:00Z".to_string(),
    }
}

/// Generates the complete SVG certificate and saves it to disk
pub fn generate_certificate(
    config: &ExperimentConfig,
    result: &SimulationResult,
    input_json: &str,
    output_path: &str,
) {
    let input_hash = hash_input(input_json);
    let output_hash = report::compute_state_hash(result);
    let timestamp = utc_timestamp();
    let engine_version = "v0.1.0";
    let integrator = &config.integrator;
    let steps = result.steps_executed;
    let experiment = &config.experiment_name;

    // Extract title from .bep metadata (if available)
    let title = match &config.metadata {
        Some(meta) if !meta.title.is_empty() => meta.title.clone(),
        _ => experiment.replace('_', " "),
    };

    // Extract author from .bep metadata (if available)
    let author = match &config.metadata {
        Some(meta) if !meta.author.is_empty() => meta.author.clone(),
        _ => "Independent Researcher".to_string(),
    };

    // Full SVG Hash (64 chars) — researchers must copy to validate
    let input_display = &input_hash;
    let output_display = &output_hash;

    let svg = format!(
r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="900" height="490" viewBox="0 0 900 490">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color="#1a1a2e"/>
      <stop offset="100%" stop-color="#16213e"/>
    </linearGradient>
    <linearGradient id="gold" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="#ffd700"/>
      <stop offset="50%" stop-color="#ffaa00"/>
      <stop offset="100%" stop-color="#ffd700"/>
    </linearGradient>
    <linearGradient id="seal" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color="#ffd700" stop-opacity="0.3"/>
      <stop offset="100%" stop-color="#ffd700" stop-opacity="0.05"/>
    </linearGradient>
  </defs>

  <!-- Fundo com borda dourada -->
  <rect x="2" y="2" width="896" height="486" rx="14" ry="14" fill="url(#bg)" stroke="url(#gold)" stroke-width="2.5"/>

  <!-- Selo circular decorativo (canto superior direito) -->
  <circle cx="830" cy="70" r="42" fill="url(#seal)" stroke="#ffd700" stroke-width="1.8" stroke-dasharray="5 3"/>
  <text x="830" y="65" text-anchor="middle" fill="#ffd700" font-family="monospace" font-size="13" font-weight="bold">3BEP</text>
  <text x="830" y="82" text-anchor="middle" fill="#ffd700" font-family="monospace" font-size="11">I64F64</text>

  <!-- Titulo principal -->
  <text x="40" y="52" fill="#ffd700" font-family="'Segoe UI', Arial, sans-serif" font-size="22" font-weight="bold" letter-spacing="2">DETERMINISTIC REPRODUCIBILITY VERIFIED</text>

  <!-- Linha separadora -->
  <line x1="40" y1="68" x2="770" y2="68" stroke="#ffd700" stroke-width="0.8" stroke-opacity="0.5"/>

  <!-- Titulo do experimento -->
  <text x="40" y="100" fill="#e0e0e0" font-family="'Segoe UI', Arial, sans-serif" font-size="17">{title}</text>

  <!-- Grid de informacoes -->
  <text x="40" y="145" fill="#888" font-family="monospace" font-size="13">ENGINE</text>
  <text x="200" y="145" fill="#ffffff" font-family="monospace" font-size="13">3BEP Sanctuary {engine_version}  |  I64F64 (128-bit Fixed Point)</text>

  <text x="40" y="172" fill="#888" font-family="monospace" font-size="13">INTEGRATOR</text>
  <text x="200" y="172" fill="#ffffff" font-family="monospace" font-size="13">{integrator}  |  {steps} steps</text>

  <text x="40" y="199" fill="#888" font-family="monospace" font-size="13">AUTHOR</text>
  <text x="200" y="199" fill="#ffffff" font-family="monospace" font-size="13">{author}</text>

  <!-- Hashes SHA-256 -->
  <line x1="40" y1="220" x2="860" y2="220" stroke="#333" stroke-width="0.5"/>

  <text x="40" y="248" fill="#888" font-family="monospace" font-size="12">INPUT  SHA-256</text>
  <text x="200" y="248" fill="#00ff88" font-family="monospace" font-size="12">{input_display}</text>

  <text x="40" y="275" fill="#888" font-family="monospace" font-size="12">OUTPUT SHA-256</text>
  <text x="200" y="275" fill="#00ff88" font-family="monospace" font-size="12">{output_display}</text>

  <!-- Metricas de conservacao -->
  <line x1="40" y1="296" x2="860" y2="296" stroke="#333" stroke-width="0.5"/>

  <text x="40" y="324" fill="#888" font-family="monospace" font-size="13">ENERGY DRIFT</text>
  <text x="200" y="324" fill="#ffffff" font-family="monospace" font-size="13">{energy_drift:?}</text>

  <text x="40" y="351" fill="#888" font-family="monospace" font-size="13">MOMENTUM dP</text>
  <text x="200" y="351" fill="#ffffff" font-family="monospace" font-size="13">dPx={dpx:?}  dPy={dpy:?}</text>

  <!-- Timestamp e rodape -->
  <line x1="40" y1="378" x2="860" y2="378" stroke="#ffd700" stroke-width="0.8" stroke-opacity="0.3"/>

  <text x="40" y="406" fill="#666" font-family="monospace" font-size="12">GENERATED</text>
  <text x="200" y="406" fill="#999" font-family="monospace" font-size="12">{timestamp}</text>

  <!-- Rodape institucional -->
  <text x="40" y="460" fill="#555" font-family="'Segoe UI', Arial, sans-serif" font-size="12">3BEP Labs  |  The Infrastructure of Physical Truth  |  github.com/3beplabs/3bep-open-science-archive</text>

  <!-- Indicador de verificacao (canto inferior direito) -->
  <rect x="730" y="440" width="140" height="30" rx="6" ry="6" fill="none" stroke="#00ff88" stroke-width="1.5"/>
  <text x="800" y="460" text-anchor="middle" fill="#00ff88" font-family="monospace" font-size="13" font-weight="bold">BIT-PERFECT</text>
</svg>"##,
        title = escape_xml(&title),
        engine_version = engine_version,
        integrator = integrator.to_uppercase(),
        steps = steps,
        author = escape_xml(&author),
        input_display = input_display,
        output_display = output_display,
        energy_drift = result.energy_drift,
        dpx = (result.initial_momentum.0 - result.final_momentum.0).abs(),
        dpy = (result.initial_momentum.1 - result.final_momentum.1).abs(),
        timestamp = timestamp,
    );

    match std::fs::write(output_path, &svg) {
        Ok(_) => {
            println!("  [OK] Certificate generated: {}", output_path);
            println!("       Input  SHA-256: {}", input_hash);
            println!("       Output SHA-256: {}", output_hash);
        }
        Err(e) => eprintln!("  [ERRO] Failed to write certificate: {}", e),
    }
}

/// Escapes special characters for XML/SVG
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}
