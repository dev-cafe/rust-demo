use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::random;

fn distance_squared(x: f64, y: f64) -> f64 {
    return x * x + y * y;
}

#[no_mangle]
#[pyfunction]
pub extern "C" fn pi_approximation(num_points: usize) -> f64 {
    let points = random::random_points(num_points);

    let num_points_inside = points
        .iter()
        .map(|p| distance_squared(p.x, p.y))
        .filter(|&d| d <= 1.0)
        .count();

    return 4.0 * (num_points_inside as f64 / num_points as f64);
}

#[pymodule]
fn pi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_wrapped(wrap_pyfunction!(pi_approximation))?;

    Ok(())
}
