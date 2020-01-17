use rand::{rngs::StdRng, Rng, SeedableRng};

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn random_points(num_points: i32) -> Vec<Point> {
    let mut rng = StdRng::from_seed([0; 32]);

    let mut points = Vec::new();

    for _ in 0..num_points {
        points.push(Point {
            x: rng.gen(),
            y: rng.gen(),
        });
    }

    return points;
}
