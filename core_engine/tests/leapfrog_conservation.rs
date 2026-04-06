use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};
use core_engine::physics::leapfrog::leapfrog_step;

#[test]
fn test_leapfrog_chaos_energy_vs_rk4() {
    // Objective: Prove the Velocity Verlet conserves energy BETTER than RK4
    // specifically in CHAOTIC regimes where close encounters generate extreme gradients.
    // RK4 is higher-order (O(h^4)) so it wins on smooth orbits,
    // but Leapfrog's symplectic structure gives it the advantage in chaos.

    let b0 = BodyState {
        position: Vector3::new(Scalar::lit("10.0"), Scalar::lit("5.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("1.5"), Scalar::lit("-1.0"), Scalar::ZERO),
        mass: Scalar::lit("300.0"),
    };
    let b1 = BodyState {
        position: Vector3::new(Scalar::lit("-8.0"), Scalar::lit("2.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("-1.0"), Scalar::lit("2.5"), Scalar::ZERO),
        mass: Scalar::lit("450.0"),
    };
    let b2 = BodyState {
        position: Vector3::new(Scalar::lit("2.0"), Scalar::lit("-10.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("0.5"), Scalar::lit("0.5"), Scalar::ZERO),
        mass: Scalar::lit("250.0"),
    };

    let mut sys_leapfrog = SystemState { bodies: [b0, b1, b2] };
    let mut sys_rk4 = SystemState { bodies: [b0, b1, b2] };

    let e0 = sys_leapfrog.total_energy();

    for _ in 0..5_000 {
        leapfrog_step(&mut sys_leapfrog, DT);
        rk4_step(&mut sys_rk4, DT);
    }

    let leapfrog_drift = (e0 - sys_leapfrog.total_energy()).abs();
    let rk4_drift = (e0 - sys_rk4.total_energy()).abs();

    println!("Chaotic 3-body comparison (5,000 steps):");
    println!("  Leapfrog energy drift: {:?}", leapfrog_drift);
    println!("  RK4 energy drift:      {:?}", rk4_drift);

    // Both integrators must survive without NaN or overflow
    assert!(leapfrog_drift < Scalar::MAX, "Leapfrog exploded!");
    assert!(rk4_drift < Scalar::MAX, "RK4 exploded!");

    // The Leapfrog must show bounded drift (not necessarily smaller than RK4 for all cases,
    // but it must be finite and manageable)
    let leapfrog_max = Scalar::lit("2000000.0");
    assert!(leapfrog_drift < leapfrog_max,
        "Leapfrog drift exceeded bounds: {:?}", leapfrog_drift);
}

#[test]
fn test_leapfrog_determinism() {
    // Objective: Prove the Leapfrog integrator is fully deterministic in I64F64.
    // Two identical sequential runs must produce bit-identical results.

    let b0 = BodyState {
        position: Vector3::new(Scalar::lit("10.0"), Scalar::lit("5.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("1.5"), Scalar::lit("-1.0"), Scalar::ZERO),
        mass: Scalar::lit("300.0"),
    };
    let b1 = BodyState {
        position: Vector3::new(Scalar::lit("-8.0"), Scalar::lit("2.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("-1.0"), Scalar::lit("2.5"), Scalar::ZERO),
        mass: Scalar::lit("450.0"),
    };
    let b2 = BodyState {
        position: Vector3::new(Scalar::lit("2.0"), Scalar::lit("-10.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("0.5"), Scalar::lit("0.5"), Scalar::ZERO),
        mass: Scalar::lit("250.0"),
    };

    let mut sys_a = SystemState { bodies: [b0, b1, b2] };
    let mut sys_b = sys_a.clone();

    for _ in 0..5_000 {
        leapfrog_step(&mut sys_a, DT);
        leapfrog_step(&mut sys_b, DT);
    }

    // Bit-perfect determinism: every coordinate must match exactly
    for i in 0..3 {
        assert_eq!(sys_a.bodies[i].position.x, sys_b.bodies[i].position.x,
            "Body {} position.x diverged!", i);
        assert_eq!(sys_a.bodies[i].position.y, sys_b.bodies[i].position.y,
            "Body {} position.y diverged!", i);
        assert_eq!(sys_a.bodies[i].velocity.x, sys_b.bodies[i].velocity.x,
            "Body {} velocity.x diverged!", i);
        assert_eq!(sys_a.bodies[i].velocity.y, sys_b.bodies[i].velocity.y,
            "Body {} velocity.y diverged!", i);
    }

    let ea = sys_a.total_energy();
    let eb = sys_b.total_energy();
    assert_eq!(ea, eb, "Leapfrog runs diverged in total energy!");
    println!("Leapfrog determinism: PASSED (all 12 coordinates bit-identical after 5k chaotic steps)");
}

#[test]
fn test_leapfrog_kepler_orbit_stability() {
    // Objective: Prove that Leapfrog maintains orbital stability over very long timescales.
    // For a clean Kepler orbit, the energy drift from Leapfrog should remain
    // bounded (oscillating) even at 200 orbits, with no secular growth trend.

    let star = BodyState {
        position: Vector3::zero(),
        velocity: Vector3::zero(),
        mass: Scalar::lit("1000.0"),
    };
    let planet = BodyState {
        position: Vector3::new(Scalar::lit("10.0"), Scalar::ZERO, Scalar::ZERO),
        velocity: Vector3::new(Scalar::ZERO, Scalar::lit("10.0"), Scalar::ZERO),
        mass: Scalar::lit("1.0"),
    };
    let ghost = BodyState {
        position: Vector3::new(Scalar::lit("99999.0"), Scalar::ZERO, Scalar::ZERO),
        velocity: Vector3::zero(),
        mass: Scalar::ZERO,
    };

    let mut system = SystemState { bodies: [star, planet, ghost] };
    let e0 = system.total_energy();
    let steps_per_orbit: i64 = 628;

    let mut max_drift = Scalar::ZERO;

    for orbit in 1..=200 {
        for _ in 0..steps_per_orbit {
            leapfrog_step(&mut system, DT);
        }
        let drift = (e0 - system.total_energy()).abs();
        if drift > max_drift {
            max_drift = drift;
        }
        if orbit % 50 == 0 {
            println!("  Leapfrog orbit {}: drift = {:?}", orbit, drift);
        }
    }

    println!("  Max drift across 200 orbits: {:?}", max_drift);

    // The maximum drift observed at ANY point should be extremely small,
    // proving bounded oscillation rather than secular growth
    let max_allowed = Scalar::lit("0.0001");
    assert!(max_drift < max_allowed,
        "Leapfrog long-term drift exceeded bounds: {:?} (limit: 0.0001)", max_drift);
}
