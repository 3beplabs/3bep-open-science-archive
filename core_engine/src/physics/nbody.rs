use alloc::vec::Vec;
use super::constants::{Scalar, G, SOFTENING};
use super::vector3::Vector3;
use super::rk4::BodyState;

/// Generic N-body system supporting arbitrary number of gravitational bodies.
/// Uses `alloc::vec::Vec` for dynamic sizing while remaining `no_std` compatible.
#[derive(Clone, Debug)]
pub struct NBodySystem {
    pub bodies: Vec<BodyState>,
}

impl NBodySystem {
    /// Creates a new N-body system from a vector of bodies.
    pub fn new(bodies: Vec<BodyState>) -> Self {
        Self { bodies }
    }

    /// Computes total mechanical energy (kinetic + potential) for the system.
    pub fn total_energy(&self) -> Scalar {
        let n = self.bodies.len();
        let mut kinetic = Scalar::ZERO;
        let mut potential = Scalar::ZERO;

        for body in &self.bodies {
            let v_sq = body.velocity.magnitude_squared();
            kinetic += (body.mass * v_sq) / Scalar::lit("2.0");
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let r_vec = self.bodies[i].position - self.bodies[j].position;
                let r_sq = r_vec.magnitude_squared() + (SOFTENING * SOFTENING);
                let r = Vector3::sqrt_fixed(r_sq);
                if r > Scalar::ZERO {
                    potential -= (G * self.bodies[i].mass * self.bodies[j].mass) / r;
                }
            }
        }

        kinetic + potential
    }
}

/// Computes gravitational accelerations for all N bodies using O(N²) direct summation.
fn calculate_nbody_accelerations(system: &NBodySystem) -> Vec<Vector3> {
    let n = system.bodies.len();
    let mut accelerations = Vec::with_capacity(n);
    for _ in 0..n {
        accelerations.push(Vector3::zero());
    }

    for i in 0..n {
        for j in (i + 1)..n {
            let d = system.bodies[j].position - system.bodies[i].position;
            let dist_sq = d.magnitude_squared() + (SOFTENING * SOFTENING);
            let dist = Vector3::sqrt_fixed(dist_sq);
            let dist_cube = dist.checked_mul(dist)
                .and_then(|d2| d2.checked_mul(dist))
                .unwrap_or(Scalar::MAX);

            // Force on body i due to body j (toward j)
            let factor_i = (G * system.bodies[j].mass) / dist_cube;
            accelerations[i] = accelerations[i] + d * factor_i;

            // Force on body j due to body i (toward i, opposite of d)
            let factor_j = (G * system.bodies[i].mass) / dist_cube;
            accelerations[j] = accelerations[j] - d * factor_j;
        }
    }

    accelerations
}

/// RK4 integrator for the N-body system.
pub fn nbody_rk4_step(system: &mut NBodySystem, dt: Scalar) {
    let n = system.bodies.len();
    let div_2 = Scalar::lit("2.0");
    let div_6 = Scalar::lit("6.0");

    // K1
    let k1_v: Vec<Vector3> = system.bodies.iter().map(|b| b.velocity).collect();
    let k1_a = calculate_nbody_accelerations(system);

    // K2
    let mut s2 = system.clone();
    for i in 0..n {
        s2.bodies[i].position = system.bodies[i].position + ((k1_v[i] * dt) / div_2);
        s2.bodies[i].velocity = system.bodies[i].velocity + ((k1_a[i] * dt) / div_2);
    }
    let k2_v: Vec<Vector3> = s2.bodies.iter().map(|b| b.velocity).collect();
    let k2_a = calculate_nbody_accelerations(&s2);

    // K3
    let mut s3 = system.clone();
    for i in 0..n {
        s3.bodies[i].position = system.bodies[i].position + ((k2_v[i] * dt) / div_2);
        s3.bodies[i].velocity = system.bodies[i].velocity + ((k2_a[i] * dt) / div_2);
    }
    let k3_v: Vec<Vector3> = s3.bodies.iter().map(|b| b.velocity).collect();
    let k3_a = calculate_nbody_accelerations(&s3);

    // K4
    let mut s4 = system.clone();
    for i in 0..n {
        s4.bodies[i].position = system.bodies[i].position + (k3_v[i] * dt);
        s4.bodies[i].velocity = system.bodies[i].velocity + (k3_a[i] * dt);
    }
    let k4_v: Vec<Vector3> = s4.bodies.iter().map(|b| b.velocity).collect();
    let k4_a = calculate_nbody_accelerations(&s4);

    // Final integration: y = y + dt/6 * (k1 + 2*k2 + 2*k3 + k4)
    for i in 0..n {
        let dv = k1_a[i] + (k2_a[i] * Scalar::lit("2.0")) + (k3_a[i] * Scalar::lit("2.0")) + k4_a[i];
        let dp = k1_v[i] + (k2_v[i] * Scalar::lit("2.0")) + (k3_v[i] * Scalar::lit("2.0")) + k4_v[i];

        system.bodies[i].velocity = system.bodies[i].velocity + ((dv * dt) / div_6);
        system.bodies[i].position = system.bodies[i].position + ((dp * dt) / div_6);
    }
}

/// Leapfrog (Velocity Verlet) symplectic integrator for the N-body system.
pub fn nbody_leapfrog_step(system: &mut NBodySystem, dt: Scalar) {
    let n = system.bodies.len();
    let half_dt = dt / Scalar::lit("2.0");

    // 1. KICK: Half-step velocity update
    let acc_current = calculate_nbody_accelerations(system);
    for i in 0..n {
        system.bodies[i].velocity = system.bodies[i].velocity + acc_current[i] * half_dt;
    }

    // 2. DRIFT: Full-step position update
    for i in 0..n {
        system.bodies[i].position = system.bodies[i].position + system.bodies[i].velocity * dt;
    }

    // 3. KICK: Complete velocity update at new positions
    let acc_new = calculate_nbody_accelerations(system);
    for i in 0..n {
        system.bodies[i].velocity = system.bodies[i].velocity + acc_new[i] * half_dt;
    }
}
