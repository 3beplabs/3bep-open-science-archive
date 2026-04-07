// Motor de execucao: converte config JSON em simulacao I64F64

use crate::config::ExperimentConfig;
use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, G, SOFTENING};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};
use core_engine::physics::leapfrog::leapfrog_step;
use core_engine::physics::nbody::{NBodySystem, nbody_rk4_step, nbody_leapfrog_step};
use std::io::Write;

/// Resultado da simulacao
pub struct SimulationResult {
    pub initial_energy: Scalar,
    pub final_energy: Scalar,
    pub energy_drift: Scalar,
    pub initial_momentum: (Scalar, Scalar),
    pub final_momentum: (Scalar, Scalar),
    pub final_positions: Vec<(Scalar, Scalar, Scalar)>,
    pub final_velocities: Vec<(Scalar, Scalar, Scalar)>,
    pub steps_executed: u64,
}

fn scalar_from_f64(v: f64) -> Scalar {
    let s = format!("{:.15}", v);
    Scalar::lit(&s)
}

fn total_energy_bodies(bodies: &[BodyState]) -> Scalar {
    let two = Scalar::lit("2.0");
    let n = bodies.len();
    let mut ke = Scalar::ZERO;
    let mut pe = Scalar::ZERO;
    for b in bodies {
        ke += (b.mass * b.velocity.magnitude_squared()) / two;
    }
    for i in 0..n {
        for j in (i+1)..n {
            let d = bodies[i].position - bodies[j].position;
            let r_sq = d.magnitude_squared() + (SOFTENING * SOFTENING);
            let r = Vector3::sqrt_fixed(r_sq);
            if r > Scalar::ZERO {
                pe -= (G * bodies[i].mass * bodies[j].mass) / r;
            }
        }
    }
    ke + pe
}

fn momentum_bodies(bodies: &[BodyState]) -> (Scalar, Scalar) {
    let px = bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.x);
    let py = bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.y);
    (px, py)
}

pub fn run_simulation(config: &ExperimentConfig) -> SimulationResult {
    let n = config.bodies.len();
    let dt = Scalar::lit(&config.dt);
    let use_leapfrog = config.integrator.to_lowercase() == "leapfrog";

    if n <= 3 {
        run_3body(config, dt, use_leapfrog)
    } else {
        run_nbody(config, dt, use_leapfrog)
    }
}

/// Executa a simulacao e grava trajetoria completa em CSV via streaming
pub fn run_with_trajectory(config: &ExperimentConfig, trajectory_path: &str) -> SimulationResult {
    let n = config.bodies.len();
    let dt = Scalar::lit(&config.dt);
    let use_leapfrog = config.integrator.to_lowercase() == "leapfrog";
    let interval = config.export_interval.unwrap_or(1);

    // Abrir arquivo CSV para streaming (nao acumula em memoria)
    let file = std::fs::File::create(trajectory_path).expect("Falha ao criar arquivo de trajetoria");
    let mut writer = std::io::BufWriter::new(file);

    // Header
    writeln!(writer, "step,time,body,pos_x,pos_y,pos_z,vel_x,vel_y,vel_z,energy,momentum_x,momentum_y").unwrap();

    if n <= 3 {
        run_3body_trajectory(config, dt, use_leapfrog, interval, &mut writer)
    } else {
        run_nbody_trajectory(config, dt, use_leapfrog, interval, &mut writer)
    }
}

fn run_3body(config: &ExperimentConfig, dt: Scalar, use_leapfrog: bool) -> SimulationResult {
    let mut system = build_3body_system(config);
    let e0 = total_energy_bodies(&system.bodies);
    let (p0x, p0y) = momentum_bodies(&system.bodies);
    let report_interval = if config.steps > 10000 { config.steps / 10 } else { config.steps };

    for step in 0..config.steps {
        if use_leapfrog { leapfrog_step(&mut system, dt); } else { rk4_step(&mut system, dt); }
        if (step + 1) % report_interval == 0 {
            let ef = total_energy_bodies(&system.bodies);
            println!("  [TICK {:>10}] Energy drift = {:?}", step + 1, (e0 - ef).abs());
        }
    }

    build_result(&system.bodies, e0, p0x, p0y, config.steps)
}

fn run_3body_trajectory(config: &ExperimentConfig, dt: Scalar, use_leapfrog: bool, interval: u64, writer: &mut impl Write) -> SimulationResult {
    let mut system = build_3body_system(config);
    let e0 = total_energy_bodies(&system.bodies);
    let (p0x, p0y) = momentum_bodies(&system.bodies);
    let dt_f64: f64 = config.dt.parse().unwrap_or(0.01);
    let n_bodies = config.bodies.len();

    // Gravar estado inicial (step 0)
    write_trajectory_row(writer, 0, 0.0, &system.bodies, n_bodies, e0, p0x, p0y);

    let report_interval = if config.steps > 10000 { config.steps / 10 } else { config.steps };

    for step in 0..config.steps {
        if use_leapfrog { leapfrog_step(&mut system, dt); } else { rk4_step(&mut system, dt); }

        let s = step + 1;
        if s % interval == 0 {
            let e = total_energy_bodies(&system.bodies);
            let (px, py) = momentum_bodies(&system.bodies);
            write_trajectory_row(writer, s, s as f64 * dt_f64, &system.bodies, n_bodies, e, px, py);
        }
        if s % report_interval == 0 {
            let ef = total_energy_bodies(&system.bodies);
            println!("  [TICK {:>10}] Energy drift = {:?}", s, (e0 - ef).abs());
        }
    }

    build_result(&system.bodies, e0, p0x, p0y, config.steps)
}

fn run_nbody(config: &ExperimentConfig, dt: Scalar, use_leapfrog: bool) -> SimulationResult {
    let mut system = build_nbody_system(config);
    let e0 = total_energy_bodies(&system.bodies);
    let (p0x, p0y) = momentum_bodies(&system.bodies);
    let report_interval = if config.steps > 10000 { config.steps / 10 } else { config.steps };

    for step in 0..config.steps {
        if use_leapfrog { nbody_leapfrog_step(&mut system, dt); } else { nbody_rk4_step(&mut system, dt); }
        if (step + 1) % report_interval == 0 {
            let ef = total_energy_bodies(&system.bodies);
            println!("  [TICK {:>10}] Energy drift = {:?}", step + 1, (e0 - ef).abs());
        }
    }

    build_result(&system.bodies, e0, p0x, p0y, config.steps)
}

fn run_nbody_trajectory(config: &ExperimentConfig, dt: Scalar, use_leapfrog: bool, interval: u64, writer: &mut impl Write) -> SimulationResult {
    let mut system = build_nbody_system(config);
    let e0 = total_energy_bodies(&system.bodies);
    let (p0x, p0y) = momentum_bodies(&system.bodies);
    let dt_f64: f64 = config.dt.parse().unwrap_or(0.01);
    let n_bodies = config.bodies.len();

    write_trajectory_row(writer, 0, 0.0, &system.bodies, n_bodies, e0, p0x, p0y);

    let report_interval = if config.steps > 10000 { config.steps / 10 } else { config.steps };

    for step in 0..config.steps {
        if use_leapfrog { nbody_leapfrog_step(&mut system, dt); } else { nbody_rk4_step(&mut system, dt); }

        let s = step + 1;
        if s % interval == 0 {
            let e = total_energy_bodies(&system.bodies);
            let (px, py) = momentum_bodies(&system.bodies);
            write_trajectory_row(writer, s, s as f64 * dt_f64, &system.bodies, n_bodies, e, px, py);
        }
        if s % report_interval == 0 {
            let ef = total_energy_bodies(&system.bodies);
            println!("  [TICK {:>10}] Energy drift = {:?}", s, (e0 - ef).abs());
        }
    }

    build_result(&system.bodies, e0, p0x, p0y, config.steps)
}

// --- Helpers ---

fn build_3body_system(config: &ExperimentConfig) -> SystemState {
    let mut bodies_arr: [BodyState; 3] = [
        BodyState { position: Vector3::zero(), velocity: Vector3::zero(), mass: Scalar::ZERO },
        BodyState { position: Vector3::zero(), velocity: Vector3::zero(), mass: Scalar::ZERO },
        BodyState { position: Vector3::new(Scalar::lit("99999.0"), Scalar::ZERO, Scalar::ZERO), velocity: Vector3::zero(), mass: Scalar::ZERO },
    ];
    for (i, b) in config.bodies.iter().enumerate().take(3) {
        bodies_arr[i] = BodyState {
            position: Vector3::new(scalar_from_f64(b.pos[0]), scalar_from_f64(b.pos[1]), scalar_from_f64(b.pos[2])),
            velocity: Vector3::new(scalar_from_f64(b.vel[0]), scalar_from_f64(b.vel[1]), scalar_from_f64(b.vel[2])),
            mass: scalar_from_f64(b.mass),
        };
    }
    SystemState { bodies: bodies_arr }
}

fn build_nbody_system(config: &ExperimentConfig) -> NBodySystem {
    let bodies: Vec<BodyState> = config.bodies.iter().map(|b| BodyState {
        position: Vector3::new(scalar_from_f64(b.pos[0]), scalar_from_f64(b.pos[1]), scalar_from_f64(b.pos[2])),
        velocity: Vector3::new(scalar_from_f64(b.vel[0]), scalar_from_f64(b.vel[1]), scalar_from_f64(b.vel[2])),
        mass: scalar_from_f64(b.mass),
    }).collect();
    NBodySystem { bodies }
}

fn build_result(bodies: &[BodyState], e0: Scalar, p0x: Scalar, p0y: Scalar, steps: u64) -> SimulationResult {
    let ef = total_energy_bodies(bodies);
    let (pfx, pfy) = momentum_bodies(bodies);
    SimulationResult {
        initial_energy: e0,
        final_energy: ef,
        energy_drift: (e0 - ef).abs(),
        initial_momentum: (p0x, p0y),
        final_momentum: (pfx, pfy),
        final_positions: bodies.iter().map(|b| (b.position.x, b.position.y, b.position.z)).collect(),
        final_velocities: bodies.iter().map(|b| (b.velocity.x, b.velocity.y, b.velocity.z)).collect(),
        steps_executed: steps,
    }
}

fn write_trajectory_row(writer: &mut impl Write, step: u64, time: f64, bodies: &[BodyState], n_bodies: usize, energy: Scalar, px: Scalar, py: Scalar) {
    for i in 0..n_bodies {
        writeln!(writer, "{},{:.6},{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
            step, time, i,
            bodies[i].position.x, bodies[i].position.y, bodies[i].position.z,
            bodies[i].velocity.x, bodies[i].velocity.y, bodies[i].velocity.z,
            energy, px, py
        ).unwrap();
    }
}
