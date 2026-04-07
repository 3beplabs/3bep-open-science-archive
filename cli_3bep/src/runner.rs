// Motor de execucao: converte config JSON em simulacao I64F64

use crate::config::ExperimentConfig;
use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, G, SOFTENING};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};
use core_engine::physics::leapfrog::leapfrog_step;
use core_engine::physics::nbody::{NBodySystem, nbody_rk4_step, nbody_leapfrog_step};

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
    // Converter f64 -> string -> Scalar para manter precisao
    let s = format!("{:.15}", v);
    Scalar::lit(&s)
}

fn total_energy_3body(system: &SystemState) -> Scalar {
    let two = Scalar::lit("2.0");
    let mut ke = Scalar::ZERO;
    let mut pe = Scalar::ZERO;
    for b in &system.bodies {
        ke += (b.mass * b.velocity.magnitude_squared()) / two;
    }
    for i in 0..3 {
        for j in (i+1)..3 {
            let d = system.bodies[i].position - system.bodies[j].position;
            let r_sq = d.magnitude_squared() + (SOFTENING * SOFTENING);
            let r = Vector3::sqrt_fixed(r_sq);
            if r > Scalar::ZERO {
                pe -= (G * system.bodies[i].mass * system.bodies[j].mass) / r;
            }
        }
    }
    ke + pe
}

fn total_energy_nbody(system: &NBodySystem) -> Scalar {
    let two = Scalar::lit("2.0");
    let mut ke = Scalar::ZERO;
    let mut pe = Scalar::ZERO;
    let n = system.bodies.len();
    for b in &system.bodies {
        ke += (b.mass * b.velocity.magnitude_squared()) / two;
    }
    for i in 0..n {
        for j in (i+1)..n {
            let d = system.bodies[i].position - system.bodies[j].position;
            let r_sq = d.magnitude_squared() + (SOFTENING * SOFTENING);
            let r = Vector3::sqrt_fixed(r_sq);
            if r > Scalar::ZERO {
                pe -= (G * system.bodies[i].mass * system.bodies[j].mass) / r;
            }
        }
    }
    ke + pe
}



pub fn run_simulation(config: &ExperimentConfig) -> SimulationResult {
    let n = config.bodies.len();
    let dt = Scalar::lit(&config.dt);
    let use_leapfrog = config.integrator.to_lowercase() == "leapfrog";

    if n <= 3 {
        // Usar sistema de 3 corpos (preenche com ghost se < 3)
        run_3body(config, dt, use_leapfrog)
    } else {
        // Usar sistema N-body generico
        run_nbody(config, dt, use_leapfrog)
    }
}

fn run_3body(config: &ExperimentConfig, dt: Scalar, use_leapfrog: bool) -> SimulationResult {
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

    let mut system = SystemState { bodies: bodies_arr };
    let e0 = total_energy_3body(&system);

    let p0x = system.bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.x);
    let p0y = system.bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.y);

    // Telemetria progressiva
    let report_interval = if config.steps > 10000 { config.steps / 10 } else { config.steps };

    for step in 0..config.steps {
        if use_leapfrog {
            leapfrog_step(&mut system, dt);
        } else {
            rk4_step(&mut system, dt);
        }

        if (step + 1) % report_interval == 0 {
            let ef = total_energy_3body(&system);
            let drift = (e0 - ef).abs();
            println!("  [TICK {:>10}] Energy drift = {:?}", step + 1, drift);
        }
    }

    let ef = total_energy_3body(&system);
    let pfx = system.bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.x);
    let pfy = system.bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.y);

    SimulationResult {
        initial_energy: e0,
        final_energy: ef,
        energy_drift: (e0 - ef).abs(),
        initial_momentum: (p0x, p0y),
        final_momentum: (pfx, pfy),
        final_positions: system.bodies.iter().map(|b| (b.position.x, b.position.y, b.position.z)).collect(),
        final_velocities: system.bodies.iter().map(|b| (b.velocity.x, b.velocity.y, b.velocity.z)).collect(),
        steps_executed: config.steps,
    }
}

fn run_nbody(config: &ExperimentConfig, dt: Scalar, use_leapfrog: bool) -> SimulationResult {
    let bodies: Vec<BodyState> = config.bodies.iter().map(|b| BodyState {
        position: Vector3::new(scalar_from_f64(b.pos[0]), scalar_from_f64(b.pos[1]), scalar_from_f64(b.pos[2])),
        velocity: Vector3::new(scalar_from_f64(b.vel[0]), scalar_from_f64(b.vel[1]), scalar_from_f64(b.vel[2])),
        mass: scalar_from_f64(b.mass),
    }).collect();

    let mut system = NBodySystem { bodies };
    let e0 = total_energy_nbody(&system);

    let p0x = system.bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.x);
    let p0y = system.bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.y);

    let report_interval = if config.steps > 10000 { config.steps / 10 } else { config.steps };

    for step in 0..config.steps {
        if use_leapfrog {
            nbody_leapfrog_step(&mut system, dt);
        } else {
            nbody_rk4_step(&mut system, dt);
        }

        if (step + 1) % report_interval == 0 {
            let ef = total_energy_nbody(&system);
            let drift = (e0 - ef).abs();
            println!("  [TICK {:>10}] Energy drift = {:?}", step + 1, drift);
        }
    }

    let ef = total_energy_nbody(&system);
    let pfx = system.bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.x);
    let pfy = system.bodies.iter().fold(Scalar::ZERO, |acc, b| acc + b.mass * b.velocity.y);

    SimulationResult {
        initial_energy: e0,
        final_energy: ef,
        energy_drift: (e0 - ef).abs(),
        initial_momentum: (p0x, p0y),
        final_momentum: (pfx, pfy),
        final_positions: system.bodies.iter().map(|b| (b.position.x, b.position.y, b.position.z)).collect(),
        final_velocities: system.bodies.iter().map(|b| (b.velocity.x, b.velocity.y, b.velocity.z)).collect(),
        steps_executed: config.steps,
    }
}
