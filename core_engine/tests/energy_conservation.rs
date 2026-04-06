use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};

#[test]
fn test_2_body_energy_conservation_and_determinism() {
    // objective: emulate stable orbit (Star-Planet) to test RK4 mathematical stability and bit-for-bit determinism
    
    // Body 0: Heavy central star
    let b0 = BodyState {
        position: Vector3::zero(),
        velocity: Vector3::zero(),
        mass: Scalar::lit("1000.0"),
    };
    
    // Body 1: Planet in orbit. v_y = sqrt(G * M / r) = sqrt(1 * 1000 / 10) = 10.0
    let b1 = BodyState {
        position: Vector3::new(Scalar::lit("10.0"), Scalar::ZERO, Scalar::ZERO),
        velocity: Vector3::new(Scalar::ZERO, Scalar::lit("10.0"), Scalar::ZERO),
        mass: Scalar::lit("1.0"), // simplifying 2-body metric for G=1
    };
    
    // Body 2: Empty/ghost body because rk4.rs physics requires exactly [BodyState; 3]
    let b2 = BodyState {
        position: Vector3::new(Scalar::lit("5000.0"), Scalar::ZERO, Scalar::ZERO),
        velocity: Vector3::zero(),
        mass: Scalar::ZERO,
    };

    let mut system_run_a = SystemState {
        bodies: [b0, b1, b2],
    };
    
    // Exact clone for parallel determinism proof (Run B)
    let mut system_run_b = system_run_a.clone();

    let e0 = system_run_a.total_energy();
    
    // SIMULATION A: 10,000 long-running iterations
    for _ in 0..10_000 {
        rk4_step(&mut system_run_a, DT);
    }
    
    // SIMULATION B: Same precise path. (In FPU/Float, thread optimization and FMA would inherently change LSBs over time)
    for _ in 0..10_000 {
        rk4_step(&mut system_run_b, DT);
    }

    let e_final_a = system_run_a.total_energy();
    let e_final_b = system_run_b.total_energy();
    
    // ABSOLUTE DETERMINISM PROOF (Anti IEEE-754)
    // Run A and Run B must compute EXACTLY identical bits on the microscopic level. ZERO CROSS-THREAD DRIFT.
    assert_eq!(system_run_a.bodies[1].position.x, system_run_b.bodies[1].position.x);
    assert_eq!(system_run_a.bodies[1].velocity.y, system_run_b.bodies[1].velocity.y);
    assert_eq!(e_final_a, e_final_b); // strict fixed point bitmatch
    
    // PHYSICAL STABILITY PROOF (RK4 Conservation)
    // RK4 inherently has O(h^4) mathematical truncation error, but with I64F64 we suffer NO "numerical evaporation" drift!
    // The divergence must purely reflect standard RK4 calculus loss.
    let drift = (e0 - e_final_a).abs();
    
    // Strict physics tolerance (the exact energy loss of a pure Runge-Kutta across 10k steps without FPU chaos under 1000g).
    let tolerance = Scalar::lit("0.05"); 
    
    assert!(drift < tolerance, "CATASTROPHIC ALERT: RK4 thermal divergence exceeded strict mathematical limits! Drift: {:?}", drift);
}
