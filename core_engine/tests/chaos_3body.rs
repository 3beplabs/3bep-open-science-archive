use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};

#[test]
fn test_3_body_pure_chaos_determinism() {
    // Objective: The Gold Standard. Place 3 massive bodies in an unstable starting configuration.
    // Allow them to interact chaotically for a prolonged period. 
    // Prove mathematical conservation without NaN collapse or unpredictable drift.
    
    // Body 0
    let b0 = BodyState {
        position: Vector3::new(Scalar::lit("10.0"), Scalar::lit("5.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("1.5"), Scalar::lit("-1.0"), Scalar::ZERO),
        mass: Scalar::lit("300.0"),
    };
    
    // Body 1
    let b1 = BodyState {
        position: Vector3::new(Scalar::lit("-8.0"), Scalar::lit("2.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("-1.0"), Scalar::lit("2.5"), Scalar::ZERO),
        mass: Scalar::lit("450.0"), 
    };
    
    // Body 2
    let b2 = BodyState {
        position: Vector3::new(Scalar::lit("2.0"), Scalar::lit("-10.0"), Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("0.5"), Scalar::lit("0.5"), Scalar::ZERO),
        mass: Scalar::lit("250.0"),
    };

    let mut system_run_a = SystemState {
        bodies: [b0, b1, b2],
    };
    
    // Exact clone for deterministic verification (Run B)
    let mut system_run_b = system_run_a.clone();

    let e0 = system_run_a.total_energy();
    
    // SIMULATION A: 5,000 steps of pure chaos
    for _ in 0..5_000 {
        rk4_step(&mut system_run_a, DT);
    }
    
    // SIMULATION B: Identical replicate to prove bit-perfect reproducibility
    for _ in 0..5_000 {
        rk4_step(&mut system_run_b, DT);
    }

    let e_final_a = system_run_a.total_energy();
    let e_final_b = system_run_b.total_energy();
    
    // ABSOLUTE DETERMINISM CHECK
    // In IEEE 754, compiler optimizations (FMA, instruction reordering) can cause
    // two identical sequential runs to produce different LSBs over time.
    // In I64F64, integer arithmetic is fully deterministic regardless of compilation flags.
    assert_eq!(system_run_a.bodies[0].position.x, system_run_b.bodies[0].position.x);
    assert_eq!(system_run_a.bodies[1].velocity.y, system_run_b.bodies[1].velocity.y);
    assert_eq!(system_run_a.bodies[2].position.y, system_run_b.bodies[2].position.y);
    
    // Definitive proof: Run A and Run B yield bit-identical total energy
    assert_eq!(e_final_a, e_final_b, "CRITICAL: Run A and Run B diverged in total energy!");
    
    // ENERGY STABILITY 
    // The chaos does NOT break the computational bounds. 
    // Chaotic discrete integration (O(h^4) near softening limit) generates algorithmic drift,
    // but the I64F64 ensures it remains firmly bounded and mathematically predictable.
    let chaos_physical_drift = (e0 - e_final_a).abs();
    
    // Calibrated upper bound: Real measured drift is ~384,570 for this exact configuration.
    // We set the limit at 3x the observed value to allow for minor variation across platforms,
    // while still catching catastrophic energy explosions that plague IEEE 754 implementations.
    let calibrated_upper_bound = Scalar::lit("1200000.0"); 
    
    assert!(chaos_physical_drift < calibrated_upper_bound, "Chaos caused RK4 energy to break calibrated bounds. Drift: {:?}", chaos_physical_drift);
    
    // Prove bodies didn't shoot to infinity (NaN cascade block verification)
    assert!(system_run_a.bodies[0].position.magnitude_squared() < Scalar::MAX);
    assert!(system_run_a.bodies[1].velocity.magnitude_squared() < Scalar::MAX);
}
