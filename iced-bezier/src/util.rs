use iced::Point;

pub fn point_clear(point: &mut Point) -> () {
    point.x = 0.0;
    point.y = 0.0;
}

pub fn point_add_weight_vec(point: &mut Point, weight: f32, vec: &Point) {
    point.x += weight * vec.x;
    point.y += weight * vec.y;
}

/// Calculate intersection of two rays.
/// If two lines are parallel or the same, we give a slight perturbation to original rays.
/// # Arguments
/// * `p0`, `p1`: Initial points
/// * `v0`, `v1`: Vector; No needs to be a unit vector.
/// * `to`: save to this.
pub fn ray_intersection(p0: &Point, v0: &Point, p1: &Point, v1: &Point, to: &mut Point) -> () {
    let mut determinant = v0.x * v1.y - v1.x * v0.y;
    if determinant == 0.0 {
        determinant += 0.0001;
    }
    let scalar = ((p0.y - p1.y) * v1.x - (p0.x - p1.x) * v1.y) / determinant;
    let x = p0.x + scalar * v0.x;
    let y = p0.y + scalar * v0.y;

    to.x = x;
    to.y = y;
}

/// Calculate distance of two vector
pub fn distance(p0: &Point, p1: &Point) -> f64 {
    let x = (p1.x - p0.x) as f64;
    let y = (p1.y - p0.y) as f64;
    f64::sqrt(x * x + y * y)
}

/// Calculate angle align to x-axis
/// return (-pi, pi]
pub fn vec_angle(center: &Point, vec: &Point) -> f64 {
    if vec.x == center.x {
        if vec.y >= vec.x {
            std::f64::consts::FRAC_PI_2
        } else {
            -std::f64::consts::FRAC_PI_2
        }
    } else {
        let tan = ((vec.y - center.y) / (vec.x - center.x)) as f64;
        tan.atan()
    }
}

pub fn norm(point: &Point) -> f32 {
    f64::sqrt((point.x * point.x + point.y * point.y) as f64) as f32
}

pub fn normalize(point: &mut Point) -> () {
    let len = norm(point);
    point.x = point.x / len;
    point.y = point.y / len;
}
