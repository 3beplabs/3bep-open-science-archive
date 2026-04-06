use fixed::types::I64F64;

// Base type for all physical calculations (128 bits total: 64 int, 64 frac)
pub type Scalar = I64F64;

// Gravitational Constant (G) normalized to prevent extremely small floats.
// In astrophysical simulations, G is often set to 1.0 for simplification.
pub const G: Scalar = Scalar::ONE;

// Softening parameter to prevent division by zero at r=0
pub const SOFTENING: Scalar = Scalar::lit("0.05");

// Fixed time step (dt) for the integrator
pub const DT: Scalar = Scalar::lit("0.01");
