use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};

/// Computes total linear momentum P = sum(m_i * v_i) for all 3 bodies.
fn total_momentum(system: &SystemState) -> Vector3 {
    let mut p = Vector3::zero();
    for body in &system.bodies {
        p = p + body.velocity * body.mass;
    }
    p
}

#[test]
fn test_linear_momentum_conservation_rk4() {
    // Objective: Newton's Third Law requires that total linear momentum P = Σ(mi * vi) 
    // is conserved in an isolated system with no external forces.
    // Any violation means the integrator is fabricating force from nothing.

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

    let p0 = total_momentum(&system);

    // Run 10,000 steps
    for _ in 0..10_000 {
        rk4_step(&mut system, DT);
    }

    let pf = total_momentum(&system);
    let dp_x = (p0.x - pf.x).abs();
    let dp_y = (p0.y - pf.y).abs();

    println!("RK4 Linear Momentum Conservation (10,000 steps):");
    println!("  P0 = ({:?}, {:?})", p0.x, p0.y);
    println!("  PF = ({:?}, {:?})", pf.x, pf.y);
    println!("  dPx = {:?}, dPy = {:?}", dp_x, dp_y);

    // Momentum should be conserved to very high precision
    let tolerance = Scalar::lit("0.001");
    assert!(dp_x < tolerance, "Px not conserved: dPx = {:?}", dp_x);
    assert!(dp_y < tolerance, "Py not conserved: dPy = {:?}", dp_y);
}

#[test]
fn test_linear_momentum_conservation_chaos() {
    // Objective: Verify momentum conservation in a chaotic 3-body system.
    // This is harder than the Kepler case because close encounters create extreme forces.

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
    let p0 = total_momentum(&system);

    for _ in 0..5_000 {
        rk4_step(&mut system, DT);
    }

    let pf = total_momentum(&system);
    let dp_x = (p0.x - pf.x).abs();
    let dp_y = (p0.y - pf.y).abs();

    println!("Chaotic 3-Body Momentum Conservation (5,000 steps):");
    println!("  P0 = ({:?}, {:?})", p0.x, p0.y);
    println!("  PF = ({:?}, {:?})", pf.x, pf.y);
    println!("  dPx = {:?}, dPy = {:?}", dp_x, dp_y);

    // Momentum must be conserved even in chaotic regimes
    let tolerance = Scalar::lit("0.01");
    assert!(dp_x < tolerance, "Chaotic Px violated: dPx = {:?}", dp_x);
    assert!(dp_y < tolerance, "Chaotic Py violated: dPy = {:?}", dp_y);
}
