// 3BEP CLI — Zero-Friction Deterministic Physics Validator
//
// Usage:
//   3bep validate experiment.json                       Run I64F64 simulation
//   3bep validate experiment.json --trajectory           Export full trajectory CSV
//   3bep validate experiment.json --compare-with-f64     Compare I64F64 vs IEEE 754
//   3bep validate experiment.json --export csv           Export final state as CSV
//   3bep validate experiment.json --export json          Export final state as JSON
//
// The CLI reads a JSON configuration file containing initial conditions,
// integrator choice, and simulation parameters. It then runs the simulation
// using the I64F64 Sanctuary engine and reports energy conservation,
// momentum conservation, and deterministic hash of the final state.

use std::env;
use std::fs;
use std::process;

mod config;
mod runner;
mod report;
mod f64_compare;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let filepath = &args[2];

    // Flags
    let compare_f64 = args.iter().any(|a| a == "--compare-with-f64");
    let trajectory = args.iter().any(|a| a == "--trajectory");
    let export_format = args.iter().position(|a| a == "--export")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());

    match command.as_str() {
        "validate" => {
            // Leitura do arquivo JSON
            let json_str = match fs::read_to_string(filepath) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("[ERRO] Falha ao ler '{}': {}", filepath, e);
                    process::exit(1);
                }
            };

            // Parse da config
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
            println!();

            // Executar simulacao I64F64
            let result = if trajectory {
                let traj_path = filepath.replace(".json", "_trajectory.csv");
                let r = runner::run_with_trajectory(&config, &traj_path);
                let rows = config.steps / config.export_interval.unwrap_or(1) + 1;
                println!("  [OK] Trajectory exported: {} ({} rows x {} bodies)",
                    traj_path, rows, config.bodies.len());
                r
            } else {
                runner::run_simulation(&config)
            };

            report::print_report(&config, &result);

            // Hash deterministico
            let hash = report::compute_state_hash(&result);
            println!("  State Hash : {}", hash);
            println!();

            // Comparacao com f64 (opcional)
            if compare_f64 {
                println!("----------------------------------------------------------");
                println!("  IEEE 754 (f64) COMPARISON MODE");
                println!("----------------------------------------------------------");
                let f64_result = f64_compare::run_f64_simulation(&config);
                f64_compare::print_comparison(&result, &f64_result);
            }

            // Exportacao de estado final (opcional)
            if let Some(format) = export_format {
                let output_path = filepath.replace(".json", &format!("_results.{}", format));
                match format {
                    "csv" => report::export_csv(&config, &result, &output_path),
                    "json" => report::export_json(&config, &result, &output_path),
                    _ => eprintln!("[ERRO] Formato '{}' nao suportado. Use 'csv' ou 'json'.", format),
                }
            }

            println!("==========================================================");
            println!("  Validation complete. Determinism guaranteed by I64F64.");
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
    println!();
    println!("JSON Fields:");
    println!("  experiment_name  string   Name for identification");
    println!("  bodies           array    [{{mass, pos:[x,y,z], vel:[x,y,z]}}]");
    println!("  integrator       string   \"rk4\" or \"leapfrog\"");
    println!("  dt               string   Time step (string for I64F64 precision)");
    println!("  steps            integer  Number of integration steps");
    println!("  export_interval  integer  (optional) Save trajectory every N steps");
}
