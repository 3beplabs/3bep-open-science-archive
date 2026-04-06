use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, DT};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};

#[test]
fn test_singularity_survival() {
    // Objective: Force a direct, head-on collision to trigger r -> 0.
    // In IEEE 754 Float, this often produces a NaN cascade due to float overflow on division by near-zero.
    // In our I64F64 Sanctuary, the SOFTENING factor and strict bit boundaries must survive linearly.
    
    // Body 0: Coming from the left
    let b0 = BodyState {
        position: Vector3::new(Scalar::lit("-5.0"), Scalar::ZERO, Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("2.0"), Scalar::ZERO, Scalar::ZERO),
        mass: Scalar::lit("500.0"), // Heavy to cause massive gravity
    };
    
    // Body 1: Coming from the right
    let b1 = BodyState {
        position: Vector3::new(Scalar::lit("5.0"), Scalar::ZERO, Scalar::ZERO),
        velocity: Vector3::new(Scalar::lit("-2.0"), Scalar::ZERO, Scalar::ZERO),
        mass: Scalar::lit("500.0"), 
    };
    
    // Body 2: Empty/ghost
    let b2 = BodyState {
        position: Vector3::new(Scalar::lit("5000.0"), Scalar::ZERO, Scalar::ZERO),
        velocity: Vector3::zero(),
        mass: Scalar::ZERO,
    };

    let mut system = SystemState {
        bodies: [b0, b1, b2],
    };

    // We will advance the system through the exact point of intersection.
    let mut min_dist_sq = Scalar::MAX;
    
    for _ in 0..500 {
        rk4_step(&mut system, DT);
        
        // In I64F64, `NaN` doesn't even exist natively. We are proving there is no PANIC / Division by Zero.
        let dist_vec = system.bodies[0].position - system.bodies[1].position;
        let d_sq = dist_vec.magnitude_squared();
        
        if d_sq < min_dist_sq {
            min_dist_sq = d_sq;
        }
    }

    // The bodies approached, hit the maximum softening gradient (a ~ 40,000), and due to the discrete dt=0.01
    // they experienced a mathematical "bounce" (velocity inversion) instead of a Float Cascade.
    
    // Proving they got extremely close
    assert!(min_dist_sq < Scalar::lit("0.1"), "The bodies did not get close enough to trigger the singularity gradient.");
    
    // Proving they survived and now hold definitive fixed point values, no NaN
    assert!(system.bodies[0].position.x < Scalar::lit("0.0")); // Bounced back to negative due to discrete O(h^4) overshoot
    assert!(system.bodies[1].position.x > Scalar::lit("0.0")); // Bounced back to positive
    
    // Velocities are huge but perfectly tracked within bounds, avoiding Float Overflow
    let v_mag_sq = system.bodies[0].velocity.magnitude_squared();
    assert!(v_mag_sq > Scalar::lit("100.0") && v_mag_sq < Scalar::MAX);
}
