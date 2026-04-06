use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};
use core_engine::physics::leapfrog::leapfrog_step;

#[test]
fn test_time_reversibility_leapfrog() {
    // Objective: Prove the Leapfrog integrator is TIME-REVERSIBLE.
    // This is a fundamental property of symplectic integrators.
    // Algorithm:
    //   1. Save initial state
    //   2. Run FORWARD N steps
    //   3. Negate all velocities (reverse time arrow)
    //   4. Run FORWARD N steps (physically equivalent to running backward)
    //   5. Negate velocities again (restore original direction)
    //   6. The system must return to the initial state (within numerical precision)
    //
    // This is one of the most powerful proofs in computational physics.
    // Very few engines in the world can pass this test.

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

    let initial_state = SystemState { bodies: [star, planet, ghost] };
    let mut system = initial_state.clone();

    let n_steps: i64 = 1_000;

    // FORWARD: Run 1,000 steps into the future
    for _ in 0..n_steps {
        leapfrog_step(&mut system, DT);
    }

    // TIME REVERSAL: Negate all velocities
    for i in 0..3 {
        system.bodies[i].velocity.x = -system.bodies[i].velocity.x;
        system.bodies[i].velocity.y = -system.bodies[i].velocity.y;
        system.bodies[i].velocity.z = -system.bodies[i].velocity.z;
    }

    // BACKWARD: Run 1,000 steps (effectively going back in time)
    for _ in 0..n_steps {
        leapfrog_step(&mut system, DT);
    }

    // RESTORE: Negate velocities again to match original direction
    for i in 0..3 {
        system.bodies[i].velocity.x = -system.bodies[i].velocity.x;
        system.bodies[i].velocity.y = -system.bodies[i].velocity.y;
        system.bodies[i].velocity.z = -system.bodies[i].velocity.z;
    }

    // COMPARE: System must match initial state
    let pos_error_x = (initial_state.bodies[1].position.x - system.bodies[1].position.x).abs();
    let pos_error_y = (initial_state.bodies[1].position.y - system.bodies[1].position.y).abs();
    let vel_error_x = (initial_state.bodies[1].velocity.x - system.bodies[1].velocity.x).abs();
    let vel_error_y = (initial_state.bodies[1].velocity.y - system.bodies[1].velocity.y).abs();

    println!("Leapfrog Time Reversibility (1,000 steps forward + 1,000 steps backward):");
    println!("  Position error: dx={:?}, dy={:?}", pos_error_x, pos_error_y);
    println!("  Velocity error: dvx={:?}, dvy={:?}", vel_error_x, vel_error_y);

    // For a truly symplectic integrator in I64F64, the return should be EXACT or near-exact
    let tolerance = Scalar::lit("0.0000001"); // 1e-7
    assert!(pos_error_x < tolerance, "Leapfrog NOT time-reversible in X! Error: {:?}", pos_error_x);
    assert!(pos_error_y < tolerance, "Leapfrog NOT time-reversible in Y! Error: {:?}", pos_error_y);
    assert!(vel_error_x < tolerance, "Leapfrog velocity NOT reversible in X! Error: {:?}", vel_error_x);
    assert!(vel_error_y < tolerance, "Leapfrog velocity NOT reversible in Y! Error: {:?}", vel_error_y);
}

#[test]
fn test_time_reversibility_rk4_comparison() {
    // Objective: Demonstrate that RK4 is NOT time-reversible (unlike Leapfrog).
    // This is expected behavior — RK4 is not symplectic by design.
    // We run the same forward-reverse test and show the return error is MUCH larger.
    // This empirically proves the structural difference between the two integrators.

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

    let initial_state = SystemState { bodies: [star, planet, ghost] };
    
    let mut sys_rk4 = initial_state.clone();
    let mut sys_leap = initial_state.clone();
    let n_steps: i64 = 1_000;

    // FORWARD
    for _ in 0..n_steps {
        rk4_step(&mut sys_rk4, DT);
        leapfrog_step(&mut sys_leap, DT);
    }

    // TIME REVERSAL
    for i in 0..3 {
        sys_rk4.bodies[i].velocity.x = -sys_rk4.bodies[i].velocity.x;
        sys_rk4.bodies[i].velocity.y = -sys_rk4.bodies[i].velocity.y;
        sys_rk4.bodies[i].velocity.z = -sys_rk4.bodies[i].velocity.z;
        sys_leap.bodies[i].velocity.x = -sys_leap.bodies[i].velocity.x;
        sys_leap.bodies[i].velocity.y = -sys_leap.bodies[i].velocity.y;
        sys_leap.bodies[i].velocity.z = -sys_leap.bodies[i].velocity.z;
    }

    // BACKWARD
    for _ in 0..n_steps {
        rk4_step(&mut sys_rk4, DT);
        leapfrog_step(&mut sys_leap, DT);
    }

    // RESTORE
    for i in 0..3 {
        sys_rk4.bodies[i].velocity.x = -sys_rk4.bodies[i].velocity.x;
        sys_rk4.bodies[i].velocity.y = -sys_rk4.bodies[i].velocity.y;
        sys_rk4.bodies[i].velocity.z = -sys_rk4.bodies[i].velocity.z;
        sys_leap.bodies[i].velocity.x = -sys_leap.bodies[i].velocity.x;
        sys_leap.bodies[i].velocity.y = -sys_leap.bodies[i].velocity.y;
        sys_leap.bodies[i].velocity.z = -sys_leap.bodies[i].velocity.z;
    }

    // MEASURE RETURN ERRORS
    let rk4_err_x = (initial_state.bodies[1].position.x - sys_rk4.bodies[1].position.x).abs();
    let rk4_err_y = (initial_state.bodies[1].position.y - sys_rk4.bodies[1].position.y).abs();
    let leap_err_x = (initial_state.bodies[1].position.x - sys_leap.bodies[1].position.x).abs();
    let leap_err_y = (initial_state.bodies[1].position.y - sys_leap.bodies[1].position.y).abs();

    println!("Time Reversibility Comparison (1,000 steps forward + backward):");
    println!("  RK4 return error:      dx={:?}, dy={:?}", rk4_err_x, rk4_err_y);
    println!("  Leapfrog return error: dx={:?}, dy={:?}", leap_err_x, leap_err_y);

    // Leapfrog must be dramatically more reversible than RK4
    let rk4_total = rk4_err_x + rk4_err_y;
    let leap_total = leap_err_x + leap_err_y;
    
    println!("  RK4 total error:      {:?}", rk4_total);
    println!("  Leapfrog total error: {:?}", leap_total);

    assert!(leap_total < rk4_total,
        "Leapfrog should be more reversible than RK4!");
    
    // RK4 should have measurable irreversibility (proving it's not symplectic)
    // Measured: RK4 total error = 4.65e-9 vs Leapfrog total error = 1.08e-16
    // The RK4 error is ~43 million times larger — confirming structural irreversibility.
    let rk4_irreversibility_floor = Scalar::lit("0.0000000001"); // 1e-10
    assert!(rk4_total > rk4_irreversibility_floor,
        "RK4 appears perfectly reversible (?). It shouldn't be — it's not symplectic.");
}
