use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};

#[test]
fn test_kepler_single_orbit_return() {
    // Objective: Validate the engine reproduces a known analytical circular orbit.
    // For a circular orbit: v_circular = sqrt(G*M/r)
    // With G=1, M=1000, r=10: v = sqrt(100) = 10.0
    // Analytical period: T = 2*pi*r/v = 2*pi ≈ 6.2832
    // At dt=0.01, one orbit ≈ 628 steps.
    // After ONE orbit, the planet must return close to its starting position.

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

    let mut system = SystemState {
        bodies: [star, planet, ghost],
    };

    let initial_pos = system.bodies[1].position;

    // Run exactly 628 steps (≈ 1 full orbit, T=6.28)
    for _ in 0..628 {
        rk4_step(&mut system, DT);
    }

    let final_pos = system.bodies[1].position;
    let return_error = final_pos - initial_pos;
    let return_dist_sq = return_error.magnitude_squared();
    let return_dist = Vector3::sqrt_fixed(return_dist_sq);

    println!("After 1 orbit (628 steps):");
    println!("  Initial pos: ({:?}, {:?})", initial_pos.x, initial_pos.y);
    println!("  Final pos:   ({:?}, {:?})", final_pos.x, final_pos.y);
    println!("  Return error: {:?}", return_dist);

    // After a SINGLE orbit, RK4 at dt=0.01 should return within 2% of the orbital radius.
    // Note: 628 steps = 6.28 time units, while exact T = 2π ≈ 6.2832, so there's 0.0032
    // of "leftover" arc. The measured return error includes both integration error AND
    // this discretization offset, which is physically expected.
    let max_error = Scalar::lit("0.2"); // 2% of r=10
    assert!(return_dist < max_error,
        "Single Kepler orbit failed: return error = {:?} (limit: 0.2)", return_dist);
}

#[test]
fn test_kepler_accumulated_drift_rate() {
    // Objective: Measure HOW FAST the RK4 orbital error accumulates.
    // If the drift is linear per orbit, the integrator is well-behaved.
    // If the drift is exponential, the integrator is broken.
    // This provides the exact baseline that Leapfrog will improve upon.

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

    let mut system = SystemState {
        bodies: [star, planet, ghost],
    };

    let initial_energy = system.total_energy();
    let steps_per_orbit: i64 = 628;

    let mut drift_at_10 = Scalar::ZERO;
    let mut drift_at_50 = Scalar::ZERO;

    for orbit in 1..=50 {
        for _ in 0..steps_per_orbit {
            rk4_step(&mut system, DT);
        }
        let current_energy = system.total_energy();
        let drift = (initial_energy - current_energy).abs();

        if orbit == 10 {
            drift_at_10 = drift;
            println!("Energy drift after 10 orbits: {:?}", drift);
        }
        if orbit == 50 {
            drift_at_50 = drift;
            println!("Energy drift after 50 orbits: {:?}", drift);
        }
    }

    // KEY ASSERTION: The drift at 50 orbits should be roughly 5x the drift at 10 orbits
    // (linear growth), NOT 25x or more (exponential growth).
    // We allow up to 8x to account for RK4's secular drift behavior.
    let ratio_limit = Scalar::lit("8.0");
    let actual_ratio = drift_at_50 / drift_at_10;

    println!("Drift ratio (50/10 orbits): {:?} (limit: 8.0)", actual_ratio);

    assert!(actual_ratio < ratio_limit,
        "RK4 drift is SUPER-LINEAR (ratio {:?}), indicating integrator instability", actual_ratio);
}

#[test]
fn test_kepler_velocity_magnitude_conservation() {
    // Objective: In a perfect circular orbit, the speed |v| should remain constant.
    // This is a direct consequence of energy + angular momentum conservation.
    // We verify |v| stays close to v_circular = 10.0 throughout the orbit.

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

    let mut system = SystemState {
        bodies: [star, planet, ghost],
    };

    let expected_v_sq = Scalar::lit("100.0"); // |v|² = 100
    let mut max_v_deviation = Scalar::ZERO;

    // Run 1 full orbit, sampling velocity every step
    for _ in 0..628 {
        rk4_step(&mut system, DT);
        let v_sq = system.bodies[1].velocity.magnitude_squared();
        let deviation = (v_sq - expected_v_sq).abs();
        if deviation > max_v_deviation {
            max_v_deviation = deviation;
        }
    }

    println!("Max |v|² deviation from 100.0 during orbit: {:?}", max_v_deviation);

    // Speed squared should stay within 0.1% of 100.0 during a single orbit
    let tolerance = Scalar::lit("0.1");
    assert!(max_v_deviation < tolerance,
        "Velocity not conserved during Kepler orbit: max deviation = {:?}", max_v_deviation);
}
