use super::constants::{Scalar, G, SOFTENING};
use super::vector3::Vector3;
use super::rk4::SystemState;

// Calculates gravitational accelerations for all 3 bodies (same physics as RK4, independent implementation)
#[inline(always)]
fn calculate_accelerations(system: &SystemState) -> [Vector3; 3] {
    let b0 = system.bodies[0];
    let b1 = system.bodies[1];
    let b2 = system.bodies[2];

    // Pair (0, 1)
    let d01 = b1.position - b0.position;
    let dist01_sq = d01.magnitude_squared() + (SOFTENING * SOFTENING);
    let dist01 = Vector3::sqrt_fixed(dist01_sq);
    let dist01_cube = dist01.checked_mul(dist01).and_then(|d2| d2.checked_mul(dist01)).unwrap_or(Scalar::MAX);
    let f01 = (G * b1.mass) / dist01_cube;
    let f10 = (G * b0.mass) / dist01_cube;
    let acc0_due_1 = d01 * f01;
    let acc1_due_0 = d01 * f10;

    // Pair (1, 2)
    let d12 = b2.position - b1.position;
    let dist12_sq = d12.magnitude_squared() + (SOFTENING * SOFTENING);
    let dist12 = Vector3::sqrt_fixed(dist12_sq);
    let dist12_cube = dist12.checked_mul(dist12).and_then(|d2| d2.checked_mul(dist12)).unwrap_or(Scalar::MAX);
    let f12 = (G * b2.mass) / dist12_cube;
    let f21 = (G * b1.mass) / dist12_cube;
    let acc1_due_2 = d12 * f12;
    let acc2_due_1 = d12 * f21;

    // Pair (0, 2)
    let d02 = b2.position - b0.position;
    let dist02_sq = d02.magnitude_squared() + (SOFTENING * SOFTENING);
    let dist02 = Vector3::sqrt_fixed(dist02_sq);
    let dist02_cube = dist02.checked_mul(dist02).and_then(|d2| d2.checked_mul(dist02)).unwrap_or(Scalar::MAX);
    let f02 = (G * b2.mass) / dist02_cube;
    let f20 = (G * b0.mass) / dist02_cube;
    let acc0_due_2 = d02 * f02;
    let acc2_due_0 = d02 * f20;

    // Sign convention: d_ij points from i to j, so acc_i_due_j = d_ij * factor is correct.
    // For the reverse (j attracted toward i), we need -d_ij, handled by subtraction/negation below.
    [
        acc0_due_1 + acc0_due_2,      // Body 0: attracted toward 1 and 2
        acc1_due_2 - acc1_due_0,      // Body 1: attracted toward 2 (+d12) and toward 0 (-d01)
        -(acc2_due_1 + acc2_due_0)    // Body 2: attracted toward 1 (-d12) and toward 0 (-d02)
    ]
}

/// Velocity Verlet (Kick-Drift-Kick) symplectic integrator.
/// 
/// Unlike RK4, this integrator preserves the symplectic structure of Hamiltonian systems,
/// resulting in bounded energy oscillation with ZERO secular drift over arbitrary timescales.
/// 
/// Algorithm:
///   1. KICK:  v(t + dt/2) = v(t) + a(t) * dt/2
///   2. DRIFT: x(t + dt)   = x(t) + v(t + dt/2) * dt
///   3. KICK:  v(t + dt)   = v(t + dt/2) + a(t + dt) * dt/2
pub fn leapfrog_step(system: &mut SystemState, dt: Scalar) {
    let half_dt = dt / Scalar::lit("2.0");

    // 1. KICK: Half-step velocity update using current accelerations
    let acc_current = calculate_accelerations(system);
    for i in 0..3 {
        system.bodies[i].velocity = system.bodies[i].velocity + acc_current[i] * half_dt;
    }

    // 2. DRIFT: Full-step position update using half-step velocities
    for i in 0..3 {
        system.bodies[i].position = system.bodies[i].position + system.bodies[i].velocity * dt;
    }

    // 3. KICK: Complete the velocity update using accelerations at NEW positions
    let acc_new = calculate_accelerations(system);
    for i in 0..3 {
        system.bodies[i].velocity = system.bodies[i].velocity + acc_new[i] * half_dt;
    }
}
