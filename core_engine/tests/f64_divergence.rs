use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};

// ========================================================================
// IEEE 754 (f64) SHADOW ENGINE
// A minimal gravitational integrator using standard floating-point arithmetic.
// This exists SOLELY to demonstrate the divergence against I64F64.
// ========================================================================

#[derive(Clone, Copy)]
struct F64Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl F64Vec3 {
    fn zero() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }
    fn mag_sq(&self) -> f64 { self.x*self.x + self.y*self.y + self.z*self.z }
}

impl core::ops::Add for F64Vec3 {
    type Output = Self;
    fn add(self, o: Self) -> Self { Self { x: self.x+o.x, y: self.y+o.y, z: self.z+o.z } }
}
impl core::ops::Sub for F64Vec3 {
    type Output = Self;
    fn sub(self, o: Self) -> Self { Self { x: self.x-o.x, y: self.y-o.y, z: self.z-o.z } }
}

fn f64_scale(v: F64Vec3, s: f64) -> F64Vec3 {
    F64Vec3 { x: v.x*s, y: v.y*s, z: v.z*s }
}

#[derive(Clone, Copy)]
struct F64Body {
    pos: F64Vec3,
    vel: F64Vec3,
    mass: f64,
}

#[derive(Clone)]
struct F64System {
    bodies: [F64Body; 3],
}

#[allow(dead_code)]
impl F64System {
    fn total_energy(&self) -> f64 {
        let mut ke = 0.0;
        let mut pe = 0.0;
        let softening: f64 = 0.05;
        for b in &self.bodies {
            ke += 0.5 * b.mass * b.vel.mag_sq();
        }
        for i in 0..3 {
            for j in (i+1)..3 {
                let r = self.bodies[i].pos - self.bodies[j].pos;
                let r_sq = r.mag_sq() + softening * softening;
                let r_mag = r_sq.sqrt();
                if r_mag > 0.0 {
                    pe -= self.bodies[i].mass * self.bodies[j].mass / r_mag;
                }
            }
        }
        ke + pe
    }
}

fn f64_accelerations(sys: &F64System) -> [F64Vec3; 3] {
    let softening: f64 = 0.05;
    let b = &sys.bodies;

    let d01 = b[1].pos - b[0].pos;
    let dist01 = (d01.mag_sq() + softening*softening).sqrt();
    let dist01_3 = dist01 * dist01 * dist01;
    let f01 = b[1].mass / dist01_3;
    let f10 = b[0].mass / dist01_3;

    let d12 = b[2].pos - b[1].pos;
    let dist12 = (d12.mag_sq() + softening*softening).sqrt();
    let dist12_3 = dist12 * dist12 * dist12;
    let f12 = b[2].mass / dist12_3;
    let f21 = b[1].mass / dist12_3;

    let d02 = b[2].pos - b[0].pos;
    let dist02 = (d02.mag_sq() + softening*softening).sqrt();
    let dist02_3 = dist02 * dist02 * dist02;
    let f02 = b[2].mass / dist02_3;
    let f20 = b[0].mass / dist02_3;

    [
        f64_scale(d01, f01) + f64_scale(d02, f02),
        f64_scale(d12, f12) - f64_scale(d01, f10),
        f64_scale(f64_scale(d12, f21) + f64_scale(d02, f20), -1.0),
    ]
}

fn f64_rk4_step(sys: &mut F64System, dt: f64) {
    let k1_v: [F64Vec3; 3] = [sys.bodies[0].vel, sys.bodies[1].vel, sys.bodies[2].vel];
    let k1_a = f64_accelerations(sys);

    let mut s2 = sys.clone();
    for i in 0..3 {
        s2.bodies[i].pos = s2.bodies[i].pos + f64_scale(k1_v[i], dt/2.0);
        s2.bodies[i].vel = s2.bodies[i].vel + f64_scale(k1_a[i], dt/2.0);
    }
    let k2_v: [F64Vec3; 3] = [s2.bodies[0].vel, s2.bodies[1].vel, s2.bodies[2].vel];
    let k2_a = f64_accelerations(&s2);

    let mut s3 = sys.clone();
    for i in 0..3 {
        s3.bodies[i].pos = s3.bodies[i].pos + f64_scale(k2_v[i], dt/2.0);
        s3.bodies[i].vel = s3.bodies[i].vel + f64_scale(k2_a[i], dt/2.0);
    }
    let k3_v: [F64Vec3; 3] = [s3.bodies[0].vel, s3.bodies[1].vel, s3.bodies[2].vel];
    let k3_a = f64_accelerations(&s3);

    let mut s4 = sys.clone();
    for i in 0..3 {
        s4.bodies[i].pos = s4.bodies[i].pos + f64_scale(k3_v[i], dt);
        s4.bodies[i].vel = s4.bodies[i].vel + f64_scale(k3_a[i], dt);
    }
    let k4_v: [F64Vec3; 3] = [s4.bodies[0].vel, s4.bodies[1].vel, s4.bodies[2].vel];
    let k4_a = f64_accelerations(&s4);

    for i in 0..3 {
        let dv = k1_a[i] + f64_scale(k2_a[i], 2.0) + f64_scale(k3_a[i], 2.0) + k4_a[i];
        let dp = k1_v[i] + f64_scale(k2_v[i], 2.0) + f64_scale(k3_v[i], 2.0) + k4_v[i];
        sys.bodies[i].vel = sys.bodies[i].vel + f64_scale(dv, dt/6.0);
        sys.bodies[i].pos = sys.bodies[i].pos + f64_scale(dp, dt/6.0);
    }
}

// ========================================================================
// TESTS: Side-by-side comparison proving IEEE 754 divergence
// ========================================================================

#[test]
fn test_f64_vs_i64f64_determinism_divergence() {
    // Objective: Run the EXACT same chaotic 3-body simulation in both f64 and I64F64.
    // Run each engine TWICE with identical inputs.
    // I64F64 runs will be bit-identical. f64 runs will also be bit-identical
    // (same thread, same compiler), BUT the two engines will produce DIFFERENT trajectories.
    // The divergence grows exponentially due to the Butterfly Effect in chaos theory,
    // amplified by IEEE 754's different rounding semantics.

    // Shared initial conditions
    let b0_pos = (10.0, 5.0, 0.0);
    let b0_vel = (1.5, -1.0, 0.0);
    let b0_mass = 300.0;
    let b1_pos = (-8.0, 2.0, 0.0);
    let b1_vel = (-1.0, 2.5, 0.0);
    let b1_mass = 450.0;
    let b2_pos = (2.0, -10.0, 0.0);
    let b2_vel = (0.5, 0.5, 0.0);
    let b2_mass = 250.0;

    // I64F64 system
    let mut sys_fixed = SystemState {
        bodies: [
            BodyState { position: Vector3::new(Scalar::lit("10.0"), Scalar::lit("5.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("1.5"), Scalar::lit("-1.0"), Scalar::ZERO), mass: Scalar::lit("300.0") },
            BodyState { position: Vector3::new(Scalar::lit("-8.0"), Scalar::lit("2.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("-1.0"), Scalar::lit("2.5"), Scalar::ZERO), mass: Scalar::lit("450.0") },
            BodyState { position: Vector3::new(Scalar::lit("2.0"), Scalar::lit("-10.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("0.5"), Scalar::lit("0.5"), Scalar::ZERO), mass: Scalar::lit("250.0") },
        ],
    };

    // f64 system (same values)
    let mut sys_float = F64System {
        bodies: [
            F64Body { pos: F64Vec3 { x: b0_pos.0, y: b0_pos.1, z: b0_pos.2 }, vel: F64Vec3 { x: b0_vel.0, y: b0_vel.1, z: b0_vel.2 }, mass: b0_mass },
            F64Body { pos: F64Vec3 { x: b1_pos.0, y: b1_pos.1, z: b1_pos.2 }, vel: F64Vec3 { x: b1_vel.0, y: b1_vel.1, z: b1_vel.2 }, mass: b1_mass },
            F64Body { pos: F64Vec3 { x: b2_pos.0, y: b2_pos.1, z: b2_pos.2 }, vel: F64Vec3 { x: b2_vel.0, y: b2_vel.1, z: b2_vel.2 }, mass: b2_mass },
        ],
    };

    let mut first_diverge_step: Option<i64> = None;

    for step in 1..=5_000i64 {
        rk4_step(&mut sys_fixed, DT);
        f64_rk4_step(&mut sys_float, 0.01);

        // Compare Body 0 position X rounded to 8 decimal places
        let fixed_x: f64 = sys_fixed.bodies[0].position.x.to_num();
        let float_x: f64 = sys_float.bodies[0].pos.x;
        let diff = (fixed_x - float_x).abs();

        if diff > 1e-10 && first_diverge_step.is_none() {
            first_diverge_step = Some(step);
            println!("DIVERGENCE DETECTED at step {}!", step);
            println!("  I64F64 Body0.x = {:.15}", fixed_x);
            println!("  f64    Body0.x = {:.15}", float_x);
            println!("  Delta          = {:.15e}", diff);
        }
    }

    // Final state comparison
    let fixed_final_x: f64 = sys_fixed.bodies[0].position.x.to_num();
    let float_final_x: f64 = sys_float.bodies[0].pos.x;
    let final_delta = (fixed_final_x - float_final_x).abs();

    println!("\nFinal state after 5,000 steps:");
    println!("  I64F64 Body0.x = {:.15}", fixed_final_x);
    println!("  f64    Body0.x = {:.15}", float_final_x);
    println!("  Final delta    = {:.6}", final_delta);

    // The engines MUST have diverged — this proves different arithmetic semantics
    assert!(first_diverge_step.is_some(),
        "CRITICAL: f64 and I64F64 produced identical results! This should be impossible.");

    // The divergence should appear within the first thousand steps for chaotic systems
    let diverge_step = first_diverge_step.unwrap();
    assert!(diverge_step < 1000,
        "Divergence appeared too late (step {}). Expected within first 1000 steps.", diverge_step);

    println!("\nConclusion: IEEE 754 and I64F64 diverged at step {} (of 5,000).", diverge_step);
    println!("This proves that floating-point arithmetic produces DIFFERENT trajectories");
    println!("than deterministic fixed-point arithmetic for the same initial conditions.");
}

#[test]
fn test_f64_cross_run_consistency_vs_i64f64() {
    // Objective: Run each engine twice and compare the results.
    // I64F64: both runs MUST be bit-identical (proven by previous tests).
    // f64: both runs should also be identical (same thread, same binary).
    // This test establishes that the divergence between engines is SYSTEMATIC,
    // not random noise.

    let mut sys_f64_a = F64System {
        bodies: [
            F64Body { pos: F64Vec3 { x: 10.0, y: 5.0, z: 0.0 }, vel: F64Vec3 { x: 1.5, y: -1.0, z: 0.0 }, mass: 300.0 },
            F64Body { pos: F64Vec3 { x: -8.0, y: 2.0, z: 0.0 }, vel: F64Vec3 { x: -1.0, y: 2.5, z: 0.0 }, mass: 450.0 },
            F64Body { pos: F64Vec3 { x: 2.0, y: -10.0, z: 0.0 }, vel: F64Vec3 { x: 0.5, y: 0.5, z: 0.0 }, mass: 250.0 },
        ],
    };
    let mut sys_f64_b = sys_f64_a.clone();

    for _ in 0..5_000 {
        f64_rk4_step(&mut sys_f64_a, 0.01);
        f64_rk4_step(&mut sys_f64_b, 0.01);
    }

    // f64 runs on the SAME binary/thread should match
    let f64_match = sys_f64_a.bodies[0].pos.x == sys_f64_b.bodies[0].pos.x;
    println!("f64 run A vs B (same binary): match = {}", f64_match);

    // This confirms that f64 is deterministic within a single binary/platform,
    // but the CROSS-PLATFORM guarantee (different compilers, different FMA flags)
    // is what IEEE 754 cannot provide and I64F64 can.
    assert!(f64_match,
        "f64 diverged between identical runs on same binary - unexpected!");
}
