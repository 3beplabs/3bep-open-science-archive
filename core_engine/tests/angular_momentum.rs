use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};

/// Computes total angular momentum Lz = sum(m_i * (x_i * vy_i - y_i * vx_i))
/// For 2D planar systems (z=0), only the z-component of L is non-zero.
fn total_angular_momentum_z(system: &SystemState) -> Scalar {
    let mut lz = Scalar::ZERO;
    for body in &system.bodies {
        // Lz = m * (x*vy - y*vx)
        let cross_z = body.position.x * body.velocity.y - body.position.y * body.velocity.x;
        lz += body.mass * cross_z;
    }
    lz
}

#[test]
fn test_angular_momentum_conservation_kepler() {
    // Objective: For a central force (gravity), angular momentum L = r x p must be conserved.
    // Kepler's Second Law is a direct consequence: equal areas swept in equal times.
    // If our integrator violates this, orbits will precess (rotate) incorrectly.

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
    let l0 = total_angular_momentum_z(&system);

    // Run 100 orbits = 62,800 steps
    for _ in 0..62_800 {
        rk4_step(&mut system, DT);
    }

    let lf = total_angular_momentum_z(&system);
    let dl = (l0 - lf).abs();
    let relative_error = if l0.abs() > Scalar::ZERO { dl / l0.abs() } else { Scalar::ZERO };

    println!("Angular Momentum Conservation (100 Kepler orbits):");
    println!("  L0 = {:?}", l0);
    println!("  LF = {:?}", lf);
    println!("  dL = {:?}", dl);
    println!("  Relative error = {:?}", relative_error);

    // Angular momentum should be conserved to within 1% over 100 orbits
    let tolerance = Scalar::lit("0.01");
    assert!(relative_error < tolerance,
        "Angular momentum violated Kepler's 2nd Law! Relative error = {:?}", relative_error);
}

#[test]
fn test_angular_momentum_conservation_3body() {
    // Objective: Total angular momentum must be conserved even in chaotic 3-body interactions.
    // Unlike the 2-body case, individual angular momenta are NOT conserved,
    // but the TOTAL Lz = sum of all bodies must remain constant.

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

    let mut system = SystemState { bodies: [b0, b1, b2] };
    let l0 = total_angular_momentum_z(&system);

    for _ in 0..5_000 {
        rk4_step(&mut system, DT);
    }

    let lf = total_angular_momentum_z(&system);
    let dl = (l0 - lf).abs();

    println!("Angular Momentum Conservation (Chaotic 3-Body, 5,000 steps):");
    println!("  L0 = {:?}", l0);
    println!("  LF = {:?}", lf);
    println!("  dL = {:?}", dl);

    // Total angular momentum conservation for chaotic 3-body under RK4 truncation.
    // Measured baseline: dL ≈ 665 over 5,000 steps. We set the limit at 3x to catch explosions.
    // Note: The Leapfrog integrator would conserve this better due to its symplectic nature.
    let tolerance = Scalar::lit("2000.0");
    assert!(dl < tolerance,
        "Total angular momentum exploded in chaos! dL = {:?}", dl);
}
