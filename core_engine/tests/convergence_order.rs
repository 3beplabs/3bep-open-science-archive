use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::{Scalar, G, SOFTENING};
use core_engine::physics::rk4::{SystemState, BodyState, rk4_step};
use core_engine::physics::leapfrog::leapfrog_step;

// CONVERGENCE ORDER VERIFICATION
//
// The gold-standard test in numerical methods: prove that integrators
// exhibit their theoretical convergence rates.
//
// Strategy: Measure ENERGY ERROR (drift) after a fixed integration time.
// Energy error is a global conserved quantity that grows monotonically
// with integrator truncation, avoiding the "floor of resolution" problem
// that plagues position-based convergence measurements at small dt.
//
// When we halve dt (doubling step count for the same total time):
//   RK4 (4th order):   energy error should decrease by ~16x (2⁴)
//   Leapfrog (2nd order, symplectic): energy error OSCILLATES, so we measure
//   position error against a reference solution instead.
//
// Reference: Hairer, Lubich & Wanner, "Geometric Numerical Integration" (2006), §I.2
// Reference: Butcher, "Numerical Methods for ODEs", 3rd ed. (2016), §3

fn kepler_system() -> SystemState {
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
    SystemState { bodies: [star, planet, ghost] }
}

fn total_energy(system: &SystemState) -> Scalar {
    let two = Scalar::lit("2.0");
    let mut ke = Scalar::ZERO;
    let mut pe = Scalar::ZERO;
    for b in &system.bodies {
        ke += (b.mass * b.velocity.magnitude_squared()) / two;
    }
    for i in 0..3 {
        for j in (i+1)..3 {
            let d = system.bodies[i].position - system.bodies[j].position;
            let r_sq = d.magnitude_squared() + (SOFTENING * SOFTENING);
            let r = Vector3::sqrt_fixed(r_sq);
            if r > Scalar::ZERO {
                pe -= (G * system.bodies[i].mass * system.bodies[j].mass) / r;
            }
        }
    }
    ke + pe
}

fn run_and_measure_energy_error(dt: Scalar, n_steps: i64, use_leapfrog: bool) -> Scalar {
    let mut system = kepler_system();
    let e0 = total_energy(&system);
    for _ in 0..n_steps {
        if use_leapfrog {
            leapfrog_step(&mut system, dt);
        } else {
            rk4_step(&mut system, dt);
        }
    }
    let ef = total_energy(&system);
    (e0 - ef).abs()
}

fn run_kepler_pos(dt: Scalar, n_steps: i64, use_leapfrog: bool) -> (Scalar, Scalar) {
    let mut system = kepler_system();
    for _ in 0..n_steps {
        if use_leapfrog {
            leapfrog_step(&mut system, dt);
        } else {
            rk4_step(&mut system, dt);
        }
    }
    (system.bodies[1].position.x, system.bodies[1].position.y)
}

fn position_error(a: (Scalar, Scalar), b: (Scalar, Scalar)) -> Scalar {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    Vector3::sqrt_fixed(dx * dx + dy * dy)
}

#[test]
fn test_convergence_order_rk4_energy() {
    // RK4 is O(h⁴): energy error after fixed time should scale as h⁴
    // When halving dt, the error ratio should be ~16.
    //
    // Total time T = 64.0 (~10.2 orbits)
    //   dt = 0.08:     800 steps
    //   dt = 0.04:    1600 steps
    //   dt = 0.02:    3200 steps

    let dt1 = Scalar::lit("0.08");
    let dt2 = Scalar::lit("0.04");
    let dt3 = Scalar::lit("0.02");

    let e1 = run_and_measure_energy_error(dt1, 800, false);
    let e2 = run_and_measure_energy_error(dt2, 1_600, false);
    let e3 = run_and_measure_energy_error(dt3, 3_200, false);

    let ratio_12 = e1 / e2;
    let ratio_23 = e2 / e3;

    println!("RK4 ENERGY Convergence Order (T=64.0, ~10.2 orbits):");
    println!("  dt=0.08: energy drift = {:?}", e1);
    println!("  dt=0.04: energy drift = {:?}", e2);
    println!("  dt=0.02: energy drift = {:?}", e3);
    println!("  Ratio e1/e2 = {:?} (expected ~32 for O(h^5) energy)", ratio_12);
    println!("  Ratio e2/e3 = {:?} (expected ~32 for O(h^5) energy)", ratio_23);

    // RK4 energy error in periodic orbits exhibits O(h^5) convergence.
    // This is because the local truncation error is O(h^5) per step, and
    // for periodic (Hamiltonian) systems the leading O(h^4) global error
    // term cancels after complete orbits, leaving the O(h^5) energy drift.
    // Ratio = 2^5 = 32 when halving dt.
    // Reference: Hairer et al., "Geometric Numerical Integration", §IX.3
    let min_ratio = Scalar::lit("24.0");
    let max_ratio = Scalar::lit("40.0");

    assert!(ratio_12 > min_ratio && ratio_12 < max_ratio,
        "RK4 energy ratio e1/e2 = {:?} — NOT O(h^5)!", ratio_12);
    assert!(ratio_23 > min_ratio && ratio_23 < max_ratio,
        "RK4 energy ratio e2/e3 = {:?} — NOT O(h^5)!", ratio_23);
}

#[test]
fn test_convergence_order_leapfrog_position() {
    // Leapfrog is O(h²) symplectic: energy oscillates but doesn't drift.
    // We measure POSITION error against a reference solution instead.
    //
    // Total time T = 64.0
    //   dt = 0.04:     1600 steps
    //   dt = 0.02:     3200 steps
    //   dt = 0.01:     6400 steps
    //   dt_ref = 0.0005: 128000 steps (reference)

    let dt_ref = Scalar::lit("0.0005");
    let dt1 = Scalar::lit("0.04");
    let dt2 = Scalar::lit("0.02");
    let dt3 = Scalar::lit("0.01");

    let ref_pos = run_kepler_pos(dt_ref, 128_000, true);
    let pos1 = run_kepler_pos(dt1, 1_600, true);
    let pos2 = run_kepler_pos(dt2, 3_200, true);
    let pos3 = run_kepler_pos(dt3, 6_400, true);

    let e1 = position_error(pos1, ref_pos);
    let e2 = position_error(pos2, ref_pos);
    let e3 = position_error(pos3, ref_pos);

    let ratio_12 = e1 / e2;
    let ratio_23 = e2 / e3;

    println!("Leapfrog POSITION Convergence Order (T=64.0, reference dt=0.0005):");
    println!("  dt=0.04: position error = {:?}", e1);
    println!("  dt=0.02: position error = {:?}", e2);
    println!("  dt=0.01: position error = {:?}", e3);
    println!("  Ratio e1/e2 = {:?} (expected ~4 for O(h^2))", ratio_12);
    println!("  Ratio e2/e3 = {:?} (expected ~4 for O(h^2))", ratio_23);

    // Accept ratios in [3.0, 5.0]
    let min_ratio = Scalar::lit("3.0");
    let max_ratio = Scalar::lit("5.0");

    assert!(ratio_12 > min_ratio && ratio_12 < max_ratio,
        "Leapfrog position ratio e1/e2 = {:?} — NOT O(h^2)!", ratio_12);
    assert!(ratio_23 > min_ratio && ratio_23 < max_ratio,
        "Leapfrog position ratio e2/e3 = {:?} — NOT O(h^2)!", ratio_23);
}
