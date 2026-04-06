use fixed::types::I64F64;
use serde::{Deserialize, Serialize};
use core::ops::{Add, Sub, Mul, Div, Neg};

// Defining the base type for all physical coordinates.
// I64F64 = 64 bits for integer part, 64 bits for fractional part.
// Range: +/- 9.22 * 10^18. Precision: 5.42 * 10^-20.
pub type Scalar = I64F64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vector3 {
    /// Creates a new vector (0, 0, 0)
    pub fn zero() -> Self {
        Self {
            x: Scalar::ZERO,
            y: Scalar::ZERO,
            z: Scalar::ZERO,
        }
    }

    /// Creates a new vector with specific coordinates
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    /// Dot Product
    pub fn dot(&self, other: Self) -> Scalar {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Magnitude squared (avoids slow/complex square root)
    pub fn magnitude_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    // Alias for existing codebase mapping
    pub fn d_sq(&self) -> Scalar {
        self.magnitude_squared()
    }

    /// Returns a normalized copy of the vector (Magnitude = 1)
    /// Uses iterative Newton-Raphson in I64F64 (pure no_std, deterministic).
    pub fn normalize(&self) -> Self {
        let mag_sq = self.d_sq();
        if mag_sq == Scalar::ZERO {
            return Self::zero();
        }
        // Newton-Raphson sqrt in fixed point (pure no_std)
        let mag = Self::sqrt_fixed(mag_sq);
        if mag == Scalar::ZERO {
            return Self::zero();
        }
        self.div(mag)
    }
    
    /// Iterative Newton-Raphson for sqrt in I64F64 (pure no_std).
    /// Converges in ~12 iterations for the type's maximum precision (64 fractional bits).
    /// x_{n+1} = (x_n + S/x_n) / 2
    pub fn sqrt_fixed(value: Scalar) -> Scalar {
        if value <= Scalar::from_num(0) {
            return Scalar::from_num(0);
        }
        let mut guess = value >> 1;
        if guess == Scalar::from_num(0) {
            guess = Scalar::from_num(1);
        }
        let two = Scalar::from_num(2);
        for _ in 0..12 {
            let next = (guess + value / guess) / two;
            if next == guess {
                break;
            }
            guess = next;
        }
        guess
    }
}

// Operator Implementations

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Scalar> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: Scalar) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<Scalar> for Vector3 {
    type Output = Self;
    fn div(self, scalar: Scalar) -> Self {
        if scalar == Scalar::ZERO {
            panic!("Vector division by zero!");
        }
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
