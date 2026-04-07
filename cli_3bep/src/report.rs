// Relatorio e exportacao de resultados

use crate::config::ExperimentConfig;
use crate::runner::SimulationResult;
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

/// Gera um hash SHA-256 simples do estado final usando uma implementacao manual
/// (sem dependencia externa, soberania total)
pub fn compute_state_hash(result: &SimulationResult) -> String {
    // Concatenar todos os valores finais em bytes para criar um fingerprint
    let mut data = Vec::new();

    // Energia
    data.extend_from_slice(&format!("{:?}", result.final_energy).as_bytes());

    // Posicoes e velocidades
    for p in &result.final_positions {
        data.extend_from_slice(&format!("{:?}{:?}{:?}", p.0, p.1, p.2).as_bytes());
    }
    for v in &result.final_velocities {
        data.extend_from_slice(&format!("{:?}{:?}{:?}", v.0, v.1, v.2).as_bytes());
    }

    // Hash simples (FNV-1a 128-bit para fingerprint deterministico)
    let mut h: u128 = 0xcbf29ce484222325;
    let prime: u128 = 0x100000001b3;
    for &byte in &data {
        h ^= byte as u128;
        h = h.wrapping_mul(prime);
    }

    format!("{:032x}", h)
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
    out.push_str(&format!("  \"state_hash\": \"{}\",\n", hash));
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
