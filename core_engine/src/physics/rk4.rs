use super::constants::{Scalar, G, SOFTENING};
use super::vector3::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct BodyState {
    pub position: Vector3,
    pub velocity: Vector3,
    pub mass: Scalar,
}

#[derive(Clone, Debug)]
pub struct SystemState {
    pub bodies: [BodyState; 3], // Fixed at 3 bodies for the 3BEP proof
}

impl SystemState {
    pub fn total_energy(&self) -> Scalar {
        let mut kinetic = Scalar::ZERO;
        let mut potential = Scalar::ZERO;

        // 1. Kinetic Energy sum(0.5 * m * v^2)
        for body in self.bodies.iter() {
            let v_sq = body.velocity.magnitude_squared();
            kinetic += (body.mass * v_sq) / Scalar::lit("2.0");
        }

        // 2. Potential Energy sum(-G * m1 * m2 / r)
        for i in 0..3 {
            for j in (i + 1)..3 {
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

// Calculates the acceleration of all bodies simultaneously (Pair optimization)
#[inline(always)]
fn calculate_all_accelerations(system: &SystemState) -> [Vector3; 3] {
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

    [
        acc0_due_1 + acc0_due_2,      // Acc Body 0
        acc1_due_2 - acc1_due_0,      // Acc Body 1
        -(acc2_due_1 + acc2_due_0)    // Acc Body 2
    ]
}

pub fn rk4_step(system: &mut SystemState, dt: Scalar) {
    // For each body, calculate independent RK4 (simple N-Body approach)
    
    // K1
    let mut k1_v = [Vector3::zero(); 3];
    for i in 0..3 { k1_v[i] = system.bodies[i].velocity; }
    let k1_a = calculate_all_accelerations(system);
    
    // K2 (State + 0.5 * K1 * dt)
    let mut system_k2 = system.clone();
    let div_2 = Scalar::lit("2.0");
    for i in 0..3 {
        system_k2.bodies[i].position = system.bodies[i].position + ((k1_v[i] * dt) / div_2);
        system_k2.bodies[i].velocity = system.bodies[i].velocity + ((k1_a[i] * dt) / div_2);
    }
    let mut k2_v = [Vector3::zero(); 3];
    for i in 0..3 { k2_v[i] = system_k2.bodies[i].velocity; }
    let k2_a = calculate_all_accelerations(&system_k2);

    // K3 (State + 0.5 * K2 * dt)
    let mut system_k3 = system.clone();
    for i in 0..3 {
        system_k3.bodies[i].position = system.bodies[i].position + ((k2_v[i] * dt) / div_2);
        system_k3.bodies[i].velocity = system.bodies[i].velocity + ((k2_a[i] * dt) / div_2);
    }
    let mut k3_v = [Vector3::zero(); 3];
    for i in 0..3 { k3_v[i] = system_k3.bodies[i].velocity; }
    let k3_a = calculate_all_accelerations(&system_k3);

    // K4 (State + K3 * dt)
    let mut system_k4 = system.clone();
    for i in 0..3 {
        system_k4.bodies[i].position = system.bodies[i].position + (k3_v[i] * dt);
        system_k4.bodies[i].velocity = system.bodies[i].velocity + (k3_a[i] * dt);
    }
    let mut k4_v = [Vector3::zero(); 3];
    for i in 0..3 { k4_v[i] = system_k4.bodies[i].velocity; }
    let k4_a = calculate_all_accelerations(&system_k4);

    // Final Integration
    // y = y + dt/6 * (k1 + 2k2 + 2k3 + k4)
    let dt_scalar = dt;
    let div_6 = Scalar::lit("6.0");
    for i in 0..3 {
        let dv = k1_a[i] + (k2_a[i] * Scalar::lit("2.0")) + (k3_a[i] * Scalar::lit("2.0")) + k4_a[i];
        let dp = k1_v[i] + (k2_v[i] * Scalar::lit("2.0")) + (k3_v[i] * Scalar::lit("2.0")) + k4_v[i];

        system.bodies[i].velocity = system.bodies[i].velocity + ((dv * dt_scalar) / div_6);
        system.bodies[i].position = system.bodies[i].position + ((dp * dt_scalar) / div_6);
    }
}
