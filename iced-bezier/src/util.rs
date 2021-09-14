use iced::Point;

pub fn point_clear(point: &mut Point) -> () {
    point.x = 0.0;
    point.y = 0.0;
}

pub fn point_add_weight_vec(point: &mut Point, weight: f32, vec: Point) {
    point.x += weight * vec.x;
    point.y += weight * vec.y;
}

/// Calculate intersection of two rays.
/// If two lines are parallel or the same, we give a slight perturbation to original rays.
/// # Arguments
/// * `p0`, `p1`: Initial points
/// * `v0`, `v1`: Vector; No needs to be a unit vector.
pub fn ray_intersection(p0: &Point, v0: &Point, p1: &Point, v1: &Point) -> Point {
    let mut determinant = v0.x * v1.y - v1.x * v0.y;
    if determinant == 0.0 {
        determinant += 0.0001;
    }
    let scalar = ((p0.y - p1.y) * v1.x - (p0.x - p1.x) * v1.y) / determinant;
    let x = p0.x + scalar * v0.x;
    let y = p0.y + scalar * v0.y;

    Point { x, y }
}

/// Calculate distance of two vector
pub fn distance(p0: &Point, p1: &Point) -> f32 {
    let x = (p1.x - p0.x) as f64;
    let y = (p1.y - p0.y) as f64;
    f64::sqrt(x * x + y * y) as f32
}
