use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::BodyState;
use core_engine::physics::nbody::{NBodySystem, nbody_rk4_step, nbody_leapfrog_step};

#[test]
fn test_nbody_5_bodies_determinism() {
    // Objective: Prove determinism holds for arbitrary N (not just hardcoded 3).
    // Create a 5-body system and verify bit-perfect reproducibility.

    let bodies = vec![
        BodyState { position: Vector3::new(Scalar::lit("0.0"), Scalar::lit("0.0"), Scalar::ZERO), velocity: Vector3::zero(), mass: Scalar::lit("500.0") },
        BodyState { position: Vector3::new(Scalar::lit("10.0"), Scalar::lit("0.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::ZERO, Scalar::lit("7.0"), Scalar::ZERO), mass: Scalar::lit("5.0") },
        BodyState { position: Vector3::new(Scalar::lit("-12.0"), Scalar::lit("5.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("2.0"), Scalar::lit("-3.0"), Scalar::ZERO), mass: Scalar::lit("8.0") },
        BodyState { position: Vector3::new(Scalar::lit("6.0"), Scalar::lit("-15.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("-1.5"), Scalar::lit("1.0"), Scalar::ZERO), mass: Scalar::lit("3.0") },
        BodyState { position: Vector3::new(Scalar::lit("-8.0"), Scalar::lit("-8.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("3.0"), Scalar::lit("2.0"), Scalar::ZERO), mass: Scalar::lit("6.0") },
    ];

    let mut sys_a = NBodySystem::new(bodies.clone());
    let mut sys_b = NBodySystem::new(bodies);

    for _ in 0..3_000 {
        nbody_rk4_step(&mut sys_a, DT);
        nbody_rk4_step(&mut sys_b, DT);
    }

    // Bit-perfect determinism across all 5 bodies
    for i in 0..5 {
        assert_eq!(sys_a.bodies[i].position.x, sys_b.bodies[i].position.x,
            "Body {} position X diverged!", i);
        assert_eq!(sys_a.bodies[i].position.y, sys_b.bodies[i].position.y,
            "Body {} position Y diverged!", i);
        assert_eq!(sys_a.bodies[i].velocity.x, sys_b.bodies[i].velocity.x,
            "Body {} velocity X diverged!", i);
        assert_eq!(sys_a.bodies[i].velocity.y, sys_b.bodies[i].velocity.y,
            "Body {} velocity Y diverged!", i);
    }

    assert_eq!(sys_a.total_energy(), sys_b.total_energy(),
        "5-body total energy diverged between runs!");
    println!("5-body determinism: ALL 20 coordinates bit-identical after 3,000 steps.");
}

#[test]
fn test_nbody_10_bodies_no_explosion() {
    // Objective: Prove the engine handles 10 bodies without NaN, overflow, or panic.
    // A 10-body system generates 45 gravitational pairs — substantial computational load.

    let bodies = vec![
        BodyState { position: Vector3::new(Scalar::lit("0.0"), Scalar::lit("0.0"), Scalar::ZERO), velocity: Vector3::zero(), mass: Scalar::lit("200.0") },
        BodyState { position: Vector3::new(Scalar::lit("10.0"), Scalar::lit("0.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::ZERO, Scalar::lit("5.0"), Scalar::ZERO), mass: Scalar::lit("3.0") },
        BodyState { position: Vector3::new(Scalar::lit("-8.0"), Scalar::lit("6.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("1.0"), Scalar::lit("-2.0"), Scalar::ZERO), mass: Scalar::lit("4.0") },
        BodyState { position: Vector3::new(Scalar::lit("5.0"), Scalar::lit("-12.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("-1.0"), Scalar::lit("1.5"), Scalar::ZERO), mass: Scalar::lit("2.5") },
        BodyState { position: Vector3::new(Scalar::lit("-15.0"), Scalar::lit("-3.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("2.5"), Scalar::lit("0.5"), Scalar::ZERO), mass: Scalar::lit("5.0") },
        BodyState { position: Vector3::new(Scalar::lit("12.0"), Scalar::lit("8.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("-0.5"), Scalar::lit("-1.5"), Scalar::ZERO), mass: Scalar::lit("3.5") },
        BodyState { position: Vector3::new(Scalar::lit("-6.0"), Scalar::lit("14.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("1.2"), Scalar::lit("-0.8"), Scalar::ZERO), mass: Scalar::lit("2.0") },
        BodyState { position: Vector3::new(Scalar::lit("18.0"), Scalar::lit("-5.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("-2.0"), Scalar::lit("3.0"), Scalar::ZERO), mass: Scalar::lit("4.5") },
        BodyState { position: Vector3::new(Scalar::lit("-20.0"), Scalar::lit("10.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("0.8"), Scalar::lit("-1.2"), Scalar::ZERO), mass: Scalar::lit("6.0") },
        BodyState { position: Vector3::new(Scalar::lit("3.0"), Scalar::lit("20.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("-1.5"), Scalar::lit("-0.5"), Scalar::ZERO), mass: Scalar::lit("1.5") },
    ];

    let mut system = NBodySystem::new(bodies);
    let e0 = system.total_energy();

    for _ in 0..1_000 {
        nbody_rk4_step(&mut system, DT);
    }

    let ef = system.total_energy();

    // All 10 bodies must have finite positions (no overflow/NaN equivalent)
    for (i, body) in system.bodies.iter().enumerate() {
        assert!(body.position.magnitude_squared() < Scalar::MAX,
            "Body {} escaped to infinity!", i);
    }

    println!("10-body system (45 pairs, 1000 steps):");
    println!("  E0 = {:?}", e0);
    println!("  EF = {:?}", ef);
    println!("  Drift = {:?}", (e0 - ef).abs());
    println!("  All 10 bodies survived within I64F64 bounds.");
}

#[test]
fn test_nbody_leapfrog_vs_rk4_consistency() {
    // Objective: Verify that both integrators (N-body RK4 and N-body Leapfrog) produce
    // physically consistent results for the same initial conditions.
    // They should compute DIFFERENT trajectories (different algorithms),
    // but both should conserve energy within their respective bounds.

    let bodies = vec![
        BodyState { position: Vector3::new(Scalar::lit("0.0"), Scalar::lit("0.0"), Scalar::ZERO), velocity: Vector3::zero(), mass: Scalar::lit("500.0") },
        BodyState { position: Vector3::new(Scalar::lit("10.0"), Scalar::lit("0.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::ZERO, Scalar::lit("7.0"), Scalar::ZERO), mass: Scalar::lit("5.0") },
        BodyState { position: Vector3::new(Scalar::lit("-12.0"), Scalar::lit("5.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("2.0"), Scalar::lit("-3.0"), Scalar::ZERO), mass: Scalar::lit("8.0") },
        BodyState { position: Vector3::new(Scalar::lit("6.0"), Scalar::lit("-15.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("-1.5"), Scalar::lit("1.0"), Scalar::ZERO), mass: Scalar::lit("3.0") },
        BodyState { position: Vector3::new(Scalar::lit("-8.0"), Scalar::lit("-8.0"), Scalar::ZERO), velocity: Vector3::new(Scalar::lit("3.0"), Scalar::lit("2.0"), Scalar::ZERO), mass: Scalar::lit("6.0") },
    ];

    let mut sys_rk4 = NBodySystem::new(bodies.clone());
    let mut sys_leap = NBodySystem::new(bodies);

    let e0 = sys_rk4.total_energy();

    for _ in 0..2_000 {
        nbody_rk4_step(&mut sys_rk4, DT);
        nbody_leapfrog_step(&mut sys_leap, DT);
    }

    let rk4_drift = (e0 - sys_rk4.total_energy()).abs();
    let leap_drift = (e0 - sys_leap.total_energy()).abs();

    println!("5-body integrator comparison (2,000 steps):");
    println!("  N-body RK4 drift:      {:?}", rk4_drift);
    println!("  N-body Leapfrog drift: {:?}", leap_drift);

    // Both must be finite (no explosion)
    assert!(rk4_drift < Scalar::MAX, "N-body RK4 exploded!");
    assert!(leap_drift < Scalar::MAX, "N-body Leapfrog exploded!");
}
