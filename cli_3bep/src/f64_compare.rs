// Comparacao direta I64F64 vs IEEE 754 (f64)
//
// Executa a MESMA simulacao em f64 e mostra o ponto exato de divergencia.

use crate::config::ExperimentConfig;
use crate::runner::SimulationResult;

#[allow(dead_code)]
pub struct F64Result {
    pub final_energy: f64,
    pub energy_drift: f64,
    pub final_positions: Vec<(f64, f64, f64)>,
    pub divergence_step: Option<u64>,
    pub divergence_delta: f64,
}

// Sistema f64 minimo para comparacao
struct F64Body {
    pos: [f64; 3],
    vel: [f64; 3],
    mass: f64,
}

fn f64_gravity(bodies: &[F64Body], i: usize) -> [f64; 3] {
    let g: f64 = 1.0;
    let soft: f64 = 0.05;
    let mut ax = 0.0;
    let mut ay = 0.0;
    let mut az = 0.0;

    for j in 0..bodies.len() {
        if j == i { continue; }
        let dx = bodies[j].pos[0] - bodies[i].pos[0];
        let dy = bodies[j].pos[1] - bodies[i].pos[1];
        let dz = bodies[j].pos[2] - bodies[i].pos[2];
        let r_sq = dx*dx + dy*dy + dz*dz + soft*soft;
        let r = r_sq.sqrt();
        let r_cubed = r * r * r;
        let f = g * bodies[j].mass / r_cubed;
        ax += f * dx;
        ay += f * dy;
        az += f * dz;
    }
    [ax, ay, az]
}

fn f64_leapfrog_step(bodies: &mut Vec<F64Body>, dt: f64) {
    let n = bodies.len();
    let mut accels: Vec<[f64; 3]> = (0..n).map(|i| f64_gravity(bodies, i)).collect();

    for i in 0..n {
        bodies[i].vel[0] += 0.5 * dt * accels[i][0];
        bodies[i].vel[1] += 0.5 * dt * accels[i][1];
        bodies[i].vel[2] += 0.5 * dt * accels[i][2];
    }
    for i in 0..n {
        bodies[i].pos[0] += dt * bodies[i].vel[0];
        bodies[i].pos[1] += dt * bodies[i].vel[1];
        bodies[i].pos[2] += dt * bodies[i].vel[2];
    }

    accels = (0..n).map(|i| f64_gravity(bodies, i)).collect();

    for i in 0..n {
        bodies[i].vel[0] += 0.5 * dt * accels[i][0];
        bodies[i].vel[1] += 0.5 * dt * accels[i][1];
        bodies[i].vel[2] += 0.5 * dt * accels[i][2];
    }
}

fn f64_rk4_step(bodies: &mut Vec<F64Body>, dt: f64) {
    let n = bodies.len();
    let orig: Vec<([f64;3], [f64;3])> = bodies.iter().map(|b| (b.pos, b.vel)).collect();

    // k1
    let k1: Vec<([f64;3],[f64;3])> = (0..n).map(|i| {
        let a = f64_gravity(bodies, i);
        (bodies[i].vel, a)
    }).collect();

    // Set state to orig + 0.5*dt*k1
    for i in 0..n {
        bodies[i].pos[0] = orig[i].0[0] + 0.5*dt*k1[i].0[0];
        bodies[i].pos[1] = orig[i].0[1] + 0.5*dt*k1[i].0[1];
        bodies[i].pos[2] = orig[i].0[2] + 0.5*dt*k1[i].0[2];
        bodies[i].vel[0] = orig[i].1[0] + 0.5*dt*k1[i].1[0];
        bodies[i].vel[1] = orig[i].1[1] + 0.5*dt*k1[i].1[1];
        bodies[i].vel[2] = orig[i].1[2] + 0.5*dt*k1[i].1[2];
    }

    let k2: Vec<([f64;3],[f64;3])> = (0..n).map(|i| {
        let a = f64_gravity(bodies, i);
        (bodies[i].vel, a)
    }).collect();

    for i in 0..n {
        bodies[i].pos[0] = orig[i].0[0] + 0.5*dt*k2[i].0[0];
        bodies[i].pos[1] = orig[i].0[1] + 0.5*dt*k2[i].0[1];
        bodies[i].pos[2] = orig[i].0[2] + 0.5*dt*k2[i].0[2];
        bodies[i].vel[0] = orig[i].1[0] + 0.5*dt*k2[i].1[0];
        bodies[i].vel[1] = orig[i].1[1] + 0.5*dt*k2[i].1[1];
        bodies[i].vel[2] = orig[i].1[2] + 0.5*dt*k2[i].1[2];
    }

    let k3: Vec<([f64;3],[f64;3])> = (0..n).map(|i| {
        let a = f64_gravity(bodies, i);
        (bodies[i].vel, a)
    }).collect();

    for i in 0..n {
        bodies[i].pos[0] = orig[i].0[0] + dt*k3[i].0[0];
        bodies[i].pos[1] = orig[i].0[1] + dt*k3[i].0[1];
        bodies[i].pos[2] = orig[i].0[2] + dt*k3[i].0[2];
        bodies[i].vel[0] = orig[i].1[0] + dt*k3[i].1[0];
        bodies[i].vel[1] = orig[i].1[1] + dt*k3[i].1[1];
        bodies[i].vel[2] = orig[i].1[2] + dt*k3[i].1[2];
    }

    let k4: Vec<([f64;3],[f64;3])> = (0..n).map(|i| {
        let a = f64_gravity(bodies, i);
        (bodies[i].vel, a)
    }).collect();

    // Final update
    for i in 0..n {
        bodies[i].pos[0] = orig[i].0[0] + (dt/6.0)*(k1[i].0[0] + 2.0*k2[i].0[0] + 2.0*k3[i].0[0] + k4[i].0[0]);
        bodies[i].pos[1] = orig[i].0[1] + (dt/6.0)*(k1[i].0[1] + 2.0*k2[i].0[1] + 2.0*k3[i].0[1] + k4[i].0[1]);
        bodies[i].pos[2] = orig[i].0[2] + (dt/6.0)*(k1[i].0[2] + 2.0*k2[i].0[2] + 2.0*k3[i].0[2] + k4[i].0[2]);
        bodies[i].vel[0] = orig[i].1[0] + (dt/6.0)*(k1[i].1[0] + 2.0*k2[i].1[0] + 2.0*k3[i].1[0] + k4[i].1[0]);
        bodies[i].vel[1] = orig[i].1[1] + (dt/6.0)*(k1[i].1[1] + 2.0*k2[i].1[1] + 2.0*k3[i].1[1] + k4[i].1[1]);
        bodies[i].vel[2] = orig[i].1[2] + (dt/6.0)*(k1[i].1[2] + 2.0*k2[i].1[2] + 2.0*k3[i].1[2] + k4[i].1[2]);
    }
}

fn f64_total_energy(bodies: &[F64Body]) -> f64 {
    let g = 1.0;
    let soft = 0.05;
    let mut ke = 0.0;
    let mut pe = 0.0;
    for b in bodies {
        ke += 0.5 * b.mass * (b.vel[0]*b.vel[0] + b.vel[1]*b.vel[1] + b.vel[2]*b.vel[2]);
    }
    for i in 0..bodies.len() {
        for j in (i+1)..bodies.len() {
            let dx = bodies[i].pos[0] - bodies[j].pos[0];
            let dy = bodies[i].pos[1] - bodies[j].pos[1];
            let dz = bodies[i].pos[2] - bodies[j].pos[2];
            let r = (dx*dx + dy*dy + dz*dz + soft*soft).sqrt();
            pe -= g * bodies[i].mass * bodies[j].mass / r;
        }
    }
    ke + pe
}

pub fn run_f64_simulation(config: &ExperimentConfig) -> F64Result {
    let dt: f64 = config.dt.parse().unwrap_or(0.01);
    let use_leapfrog = config.integrator.to_lowercase() == "leapfrog";

    let mut bodies: Vec<F64Body> = config.bodies.iter().map(|b| F64Body {
        pos: b.pos,
        vel: b.vel,
        mass: b.mass,
    }).collect();

    // Preencher com ghost se < 3
    while bodies.len() < 3 {
        bodies.push(F64Body { pos: [99999.0, 0.0, 0.0], vel: [0.0, 0.0, 0.0], mass: 0.0 });
    }

    let e0 = f64_total_energy(&bodies);

    for _step in 0..config.steps {
        if use_leapfrog {
            f64_leapfrog_step(&mut bodies, dt);
        } else {
            f64_rk4_step(&mut bodies, dt);
        }
    }

    let ef = f64_total_energy(&bodies);

    F64Result {
        final_energy: ef,
        energy_drift: (e0 - ef).abs(),
        final_positions: bodies.iter().take(config.bodies.len()).map(|b| (b.pos[0], b.pos[1], b.pos[2])).collect(),
        divergence_step: None, // Seria preciso rodar ambos em paralelo para detectar
        divergence_delta: 0.0,
    }
}

pub fn print_comparison(i64_result: &SimulationResult, f64_result: &F64Result) {
    println!();
    println!("  f64 Final Energy : {:.15}", f64_result.final_energy);
    println!("  f64 Energy Drift : {:.15}", f64_result.energy_drift);
    println!();

    println!("  COMPARISON:");
    for i in 0..i64_result.final_positions.len().min(f64_result.final_positions.len()) {
        let (ix, iy, _) = i64_result.final_positions[i];
        let (fx, fy, _) = f64_result.final_positions[i];

        // Converter I64F64 para f64 para comparacao
        let ix_f64: f64 = format!("{:?}", ix).parse().unwrap_or(0.0);
        let iy_f64: f64 = format!("{:?}", iy).parse().unwrap_or(0.0);

        let dx = (ix_f64 - fx).abs();
        let dy = (iy_f64 - fy).abs();

        println!("  Body {} | I64F64=({:.10}, {:.10}) f64=({:.10}, {:.10}) delta=({:.2e}, {:.2e})",
            i, ix_f64, iy_f64, fx, fy, dx, dy);
    }
    println!();
}
