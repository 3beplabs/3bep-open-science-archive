// NOTE: Examples are standalone binaries and may use std.
// The core_engine library itself is strictly #![no_std].
use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};
use std::time::Instant;

fn main() {
    println!("===========================================================");
    println!(" 3BEP LABS: EXTREME TEMPORAL STRESS TEST (CPU BURN-IN) ");
    println!(" WARNING: This execution evaluates the absolute limit of");
    println!("          I64F64 deterministic thermodynamic entropy.");
    println!("          (Requires Native FPU-Free Processing)");
    println!("===========================================================\n");

    // Extreme Chaos Setup
    let b0 = BodyState {
        position: Vector3::new(Scalar::lit("50.0"), Scalar::lit("-25.0"), Scalar::lit("10.0")),
        velocity: Vector3::new(Scalar::lit("5.5"), Scalar::lit("-7.0"), Scalar::lit("3.1")),
        mass: Scalar::lit("1200.0"),
    };
    
    let b1 = BodyState {
        position: Vector3::new(Scalar::lit("-40.0"), Scalar::lit("30.0"), Scalar::lit("-15.0")),
        velocity: Vector3::new(Scalar::lit("-3.0"), Scalar::lit("9.5"), Scalar::lit("2.2")),
        mass: Scalar::lit("950.0"), 
    };
    
    let b2 = BodyState {
        position: Vector3::new(Scalar::lit("15.0"), Scalar::lit("-60.0"), Scalar::lit("25.0")),
        velocity: Vector3::new(Scalar::lit("1.5"), Scalar::lit("2.5"), Scalar::lit("-5.0")),
        mass: Scalar::lit("1500.0"),
    };

    let mut system = SystemState {
        bodies: [b0, b1, b2],
    };

    let total_ticks: u64 = 50_000_000;
    let report_interval: u64 = 250_000;

    println!("Target: {} algorithmic ticks of pure 3-body chaos.", total_ticks);
    println!("Engine: Sanctuary Module (I64F64 pure fixed point)");
    println!("Executing...\n");

    let start_time = Instant::now();
    let original_energy = system.total_energy();

    for i in 1..=total_ticks {
        rk4_step(&mut system, DT);
        
        if i % report_interval == 0 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let current_energy = system.total_energy();
            let drift = (original_energy - current_energy).abs();
            
            println!("[{:>9} ticks] Elapsed: {:>6.2}s | Energy Drift: {:?} | Core B0 Pos X: {:?}", 
                     i, elapsed, drift, system.bodies[0].position.x);
        }
    }

    let total_elapsed = start_time.elapsed().as_secs_f64();
    println!("\n===========================================================");
    println!(" STRESS TEST COMPLETED SUCCESSFULLY ");
    println!(" Elapsed Time: {:.3} seconds", total_elapsed);
    println!(" Iterations:   {}", total_ticks);
    println!(" Final Drift:  {:?}", (original_energy - system.total_energy()).abs());
    println!(" EPS (Evals/s): {:.0}", (total_ticks as f64) / total_elapsed);
    println!("===========================================================");
    println!("Conclusion: Fixed-point mathematical boundary endured without crash/NaN.");
}
