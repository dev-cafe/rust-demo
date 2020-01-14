use pi::pi_approximation;

#[test]
fn test_pi() {
    let num_points = 500_000;

    let pi = pi_approximation(num_points);
    let pi_reference = std::f64::consts::PI;

    assert!((pi - pi_reference).abs() < 0.01);
}
