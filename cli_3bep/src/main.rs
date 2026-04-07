// 3BEP CLI — Zero-Friction Deterministic Physics Validator
//
// Usage:
//   3bep validate experiment.json                       Run I64F64 simulation
//   3bep validate experiment.json --trajectory           Export full trajectory CSV
//   3bep validate experiment.json --compare-with-f64     Compare I64F64 vs IEEE 754
//   3bep validate experiment.json --export csv           Export final state as CSV
//   3bep validate experiment.json --export json          Export final state as JSON
//   3bep validate experiment.json --certificate          Generate SVG reproducibility seal
//
// Supports both .json and .bep (JSON with academic metadata) files.
// Hash algorithm: SHA-256 (FIPS 180-4, pure Rust implementation).

use std::env;
use std::fs;
use std::process;

mod config;
mod runner;
mod report;
mod f64_compare;
mod certificate;
mod sha256;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let filepath = &args[2];

    // CLI Flags
    let compare_f64 = args.iter().any(|a| a == "--compare-with-f64");
    let trajectory = args.iter().any(|a| a == "--trajectory");
    let gen_certificate = args.iter().any(|a| a == "--certificate");
    let export_format = args.iter().position(|a| a == "--export")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());

    match command.as_str() {
        "validate" => {
            // Read JSON/BEP file
            let json_str = match fs::read_to_string(filepath) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("[ERRO] Falha ao ler '{}': {}", filepath, e);
                    process::exit(1);
                }
            };

            // Parse configuration
            let config = match config::ExperimentConfig::from_json(&json_str) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[ERRO] JSON invalido: {}", e);
                    process::exit(1);
                }
            };

            println!("==========================================================");
            println!("  3BEP SANCTUARY: Deterministic Physics Validator (I64F64)");
            println!("==========================================================");
            println!();
            println!("  Experiment : {}", config.experiment_name);
            println!("  Bodies     : {}", config.bodies.len());
            println!("  Integrator : {}", config.integrator);
            println!("  dt         : {}", config.dt);
            println!("  Steps      : {}", config.steps);
            if trajectory {
                let interval = config.export_interval.unwrap_or(1);
                println!("  Trajectory : ON (every {} steps)", interval);
            }
            if gen_certificate {
                println!("  Certificate: ON (SVG with SHA-256)");
            }
            println!();

            // Execute I64F64 simulation
            let result = if trajectory {
                let traj_path = filepath.replace(".json", "_trajectory.csv")
                                        .replace(".bep", "_trajectory.csv");
                let r = runner::run_with_trajectory(&config, &traj_path);
                let rows = config.steps / config.export_interval.unwrap_or(1) + 1;
                println!("  [OK] Trajectory exported: {} ({} rows x {} bodies)",
                    traj_path, rows, config.bodies.len());
                r
            } else {
                runner::run_simulation(&config)
            };

            report::print_report(&config, &result);

            // Deterministic SHA-256 hash
            let hash = report::compute_state_hash(&result);
            println!("  State Hash (SHA-256): {}", hash);
            println!();

            // Compare with f64 (optional)
            if compare_f64 {
                println!("----------------------------------------------------------");
                println!("  IEEE 754 (f64) COMPARISON MODE");
                println!("----------------------------------------------------------");
                let f64_result = f64_compare::run_f64_simulation(&config);
                f64_compare::print_comparison(&result, &f64_result);
            }

            // Export final state (optional)
            if let Some(format) = export_format {
                let output_path = filepath.replace(".json", &format!("_results.{}", format))
                                          .replace(".bep", &format!("_results.{}", format));
                match format {
                    "csv" => report::export_csv(&config, &result, &output_path),
                    "json" => report::export_json(&config, &result, &output_path),
                    _ => eprintln!("[ERRO] Formato '{}' nao suportado. Use 'csv' ou 'json'.", format),
                }
            }

            // Generate SVG certificate (optional)
            if gen_certificate {
                let cert_path = filepath.replace(".json", "_certificate.svg")
                                        .replace(".bep", "_certificate.svg");
                certificate::generate_certificate(&config, &result, &json_str, &cert_path);
            }

            println!("==========================================================");
            println!("  Validation complete. Determinism guaranteed by I64F64.");
            println!("  Integrity sealed with SHA-256 (FIPS 180-4, pure Rust).");
            println!("==========================================================");
        }
        _ => {
            eprintln!("[ERRO] Comando '{}' desconhecido.", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("3BEP CLI — Deterministic Physics Validator");
    println!();
    println!("Usage:");
    println!("  3bep validate <experiment.json>                     Run I64F64 simulation");
    println!("  3bep validate <experiment.json> --trajectory         Export full trajectory CSV");
    println!("  3bep validate <experiment.json> --compare-with-f64   Compare I64F64 vs IEEE 754");
    println!("  3bep validate <experiment.json> --export csv         Export final state as CSV");
    println!("  3bep validate <experiment.json> --export json        Export final state as JSON");
    println!("  3bep validate <experiment.json> --certificate        Generate SVG reproducibility seal");
    println!();
    println!("Supports both .json and .bep (JSON with academic metadata) files.");
    println!();
    println!("JSON Fields:");
    println!("  experiment_name  string   Name for identification");
    println!("  bodies           array    [{{mass, pos:[x,y,z], vel:[x,y,z]}}]");
    println!("  integrator       string   \"rk4\" or \"leapfrog\"");
    println!("  dt               string   Time step (string for I64F64 precision)");
    println!("  steps            integer  Number of integration steps");
    println!("  export_interval  integer  (optional) Save trajectory every N steps");
    println!("  metadata         object   (optional) Academic metadata for .bep scripts");
    println!();
    println!("Hash: SHA-256 (FIPS 180-4, pure Rust — zero external crates)");
}
