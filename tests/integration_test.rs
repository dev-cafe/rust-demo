#[test]
fn test_pi() {
    let num_points = 500_000;

    let result = pi::pi_approximation(num_points);
    let reference = std::f64::consts::PI;

    assert!((result - reference).abs() < 0.01);
}
