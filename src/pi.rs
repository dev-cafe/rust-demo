use crate::random;

fn distance_squared(x: f64, y: f64) -> f64 {
    return x * x + y * y;
}

#[no_mangle]
pub extern "C" fn pi_approximation(num_points: i32) -> f64 {
    let points = random::random_points(num_points);

    let num_points_inside = points
        .iter()
        .map(|p| distance_squared(p.x, p.y))
        .filter(|&d| d <= 1.0)
        .count();

    return 4.0 * (num_points_inside as f64 / num_points as f64);
}
