// Reporting and JSON/CSV final state export

use crate::config::ExperimentConfig;
use crate::runner::SimulationResult;
use crate::sha256;
use std::fs;

pub fn print_report(config: &ExperimentConfig, result: &SimulationResult) {
    println!("----------------------------------------------------------");
    println!("  SIMULATION REPORT (I64F64 Sanctuary)");
    println!("----------------------------------------------------------");
    println!();
    println!("  Initial Energy : {:?}", result.initial_energy);
    println!("  Final Energy   : {:?}", result.final_energy);
    println!("  Energy Drift   : {:?}", result.energy_drift);
    println!();
    println!("  Initial Momentum: Px={:?}, Py={:?}", result.initial_momentum.0, result.initial_momentum.1);
    println!("  Final Momentum  : Px={:?}, Py={:?}", result.final_momentum.0, result.final_momentum.1);
    println!("  dPx = {:?}", (result.initial_momentum.0 - result.final_momentum.0).abs());
    println!("  dPy = {:?}", (result.initial_momentum.1 - result.final_momentum.1).abs());
    println!();

    for (i, (pos, vel)) in result.final_positions.iter()
        .zip(result.final_velocities.iter())
        .enumerate()
    {
        if i < config.bodies.len() {
            println!("  Body {} | pos=({:?}, {:?}, {:?}) vel=({:?}, {:?}, {:?})",
                i, pos.0, pos.1, pos.2, vel.0, vel.1, vel.2);
        }
    }
    println!();
}

/// Generates SHA-256 of the final simulation state (Pure Rust, FIPS 180-4)
/// Total sovereignty: zero external cryptography crates
pub fn compute_state_hash(result: &SimulationResult) -> String {
    // Concatenate all final values into bytes to create the digest
    let mut data = Vec::new();

    // Energy
    data.extend_from_slice(format!("{:?}", result.final_energy).as_bytes());

    // Positions and velocities
    for p in &result.final_positions {
        data.extend_from_slice(format!("{:?}{:?}{:?}", p.0, p.1, p.2).as_bytes());
    }
    for v in &result.final_velocities {
        data.extend_from_slice(format!("{:?}{:?}{:?}", v.0, v.1, v.2).as_bytes());
    }

    // SHA-256 (FIPS 180-4, pure Rust)
    sha256::sha256_hex(&data)
}

pub fn export_csv(config: &ExperimentConfig, result: &SimulationResult, path: &str) {
    let mut csv = String::new();
    csv.push_str("body,pos_x,pos_y,pos_z,vel_x,vel_y,vel_z\n");

    for (i, (pos, vel)) in result.final_positions.iter()
        .zip(result.final_velocities.iter())
        .enumerate()
    {
        if i < config.bodies.len() {
            csv.push_str(&format!("{},{:?},{:?},{:?},{:?},{:?},{:?}\n",
                i, pos.0, pos.1, pos.2, vel.0, vel.1, vel.2));
        }
    }

    match fs::write(path, &csv) {
        Ok(_) => println!("  [OK] Results exported to: {}", path),
        Err(e) => eprintln!("  [ERRO] Failed to write CSV: {}", e),
    }
}

pub fn export_json(config: &ExperimentConfig, result: &SimulationResult, path: &str) {
    let hash = compute_state_hash(result);
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"experiment\": \"{}\",\n", config.experiment_name));
    out.push_str(&format!("  \"integrator\": \"{}\",\n", config.integrator));
    out.push_str(&format!("  \"steps\": {},\n", result.steps_executed));
    out.push_str(&format!("  \"initial_energy\": \"{:?}\",\n", result.initial_energy));
    out.push_str(&format!("  \"final_energy\": \"{:?}\",\n", result.final_energy));
    out.push_str(&format!("  \"energy_drift\": \"{:?}\",\n", result.energy_drift));
    out.push_str(&format!("  \"state_hash_sha256\": \"{}\",\n", hash));
    out.push_str("  \"bodies\": [\n");

    for (i, (pos, vel)) in result.final_positions.iter()
        .zip(result.final_velocities.iter())
        .enumerate()
    {
        if i < config.bodies.len() {
            let comma = if i < config.bodies.len() - 1 { "," } else { "" };
            out.push_str(&format!("    {{\"pos\": [\"{:?}\", \"{:?}\", \"{:?}\"], \"vel\": [\"{:?}\", \"{:?}\", \"{:?}\"]}}{}\n",
                pos.0, pos.1, pos.2, vel.0, vel.1, vel.2, comma));
        }
    }

    out.push_str("  ]\n");
    out.push_str("}\n");

    match fs::write(path, &out) {
        Ok(_) => println!("  [OK] Results exported to: {}", path),
        Err(e) => eprintln!("  [ERRO] Failed to write JSON: {}", e),
    }
}
