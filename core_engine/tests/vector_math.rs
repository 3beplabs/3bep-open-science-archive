use core_engine::physics::vector3::Vector3;
use core_engine::physics::constants::Scalar;

#[test]
fn test_vector_addition_and_subtraction() {
    // using base-2 powers (0.5, 0.25) to avoid infinite binary decimals in fixed-point macro lit()
    let v1 = Vector3::new(Scalar::lit("1.5"), Scalar::lit("-2.0"), Scalar::lit("3.25"));
    let v2 = Vector3::new(Scalar::lit("0.5"), Scalar::lit("4.0"), Scalar::lit("-1.25"));

    let sum = v1 + v2;
    assert_eq!(sum.x, Scalar::lit("2.0"));
    assert_eq!(sum.y, Scalar::lit("2.0"));
    assert_eq!(sum.z, Scalar::lit("2.0"));

    let diff = v1 - v2;
    assert_eq!(diff.x, Scalar::lit("1.0"));
    assert_eq!(diff.y, Scalar::lit("-6.0"));
    assert_eq!(diff.z, Scalar::lit("4.5"));
}

#[test]
fn test_vector_dot_product() {
    let v1 = Vector3::new(Scalar::lit("1.0"), Scalar::lit("2.0"), Scalar::lit("3.0"));
    let v2 = Vector3::new(Scalar::lit("-1.0"), Scalar::lit("0.5"), Scalar::lit("2.0"));
    
    // Dot = (1*-1) + (2*0.5) + (3*2) = -1 + 1 + 6 = 6.0
    let dot = v1.dot(v2);
    assert_eq!(dot, Scalar::lit("6.0"));
}

#[test]
fn test_vector_magnitude_squared() {
    let v = Vector3::new(Scalar::lit("2.0"), Scalar::lit("3.0"), Scalar::lit("4.0"));
    // mag_sq = 4 + 9 + 16 = 29
    assert_eq!(v.magnitude_squared(), Scalar::lit("29.0"));
}

#[test]
fn test_sqrt_fixed_newton_raphson() {
    let val = Scalar::lit("25.0");
    let root = Vector3::sqrt_fixed(val);
    assert_eq!(root, Scalar::lit("5.0"));

    let val2 = Scalar::lit("2.0");
    let root2 = Vector3::sqrt_fixed(val2);
    // external scalar comparison for anti-tautology
    let expected = Scalar::lit("1.4142135623730950488");
    let diff = root2 - expected;
    assert!(diff.abs() < Scalar::lit("0.0000000000000000001"));
}

#[test]
fn test_vector_normalize() {
    let v = Vector3::new(Scalar::lit("0.0"), Scalar::lit("3.0"), Scalar::lit("4.0"));
    let n = v.normalize();
    
    // tolerance for Least Significant Bit (LSB) truncation in fixed-point division
    let eps = Scalar::lit("0.000000000001"); 
    let diff_y = n.y - Scalar::lit("0.6");
    let diff_z = n.z - Scalar::lit("0.8");
    
    assert!(diff_y.abs() < eps, "Y dimension failed: {:?}", diff_y);
    assert!(diff_z.abs() < eps, "Z dimension failed: {:?}", diff_z);
    
    // exact magnitude check - precision must be extremely close to 1.0
    let n_mag_sq = n.magnitude_squared();
    let mag_diff = n_mag_sq - Scalar::ONE;
    assert!(mag_diff.abs() < eps, "Magnitude drift: {:?}", mag_diff);
}
