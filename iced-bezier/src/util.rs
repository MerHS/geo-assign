use iced::Point;

pub const RESOLUTION: usize = 100;
pub const RES_4: usize = RESOLUTION / 4;
pub const PTS_RADIUS: f32 = 3.0;

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

/// Calculate angle aligned to +x-axis
/// return (-pi, pi]
pub fn point_angle(center: &Point, vec: &Point) -> f64 {
    if vec.x == center.x {
        if vec.y >= center.y {
            std::f64::consts::FRAC_PI_2
        } else {
            -std::f64::consts::FRAC_PI_2
        }
    } else {
        let dy = vec.y - center.y;
        let dx = vec.x - center.x;
        if dx < 0.0 {
            if dy < 0.0 {
                -std::f64::consts::PI + ((dy / dx) as f64).atan()
            } else {
                std::f64::consts::PI - ((-dy / dx) as f64).atan()
            }
        } else {
            ((dy / dx) as f64).atan()
        }
    }
}

const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

/// Calculate the angle between two vectors.
/// return [0, pi]
pub fn vec_angle(v0: &Point, v1: &Point) -> f64 {
    let theta_1 = point_angle(&ORIGIN, v0);
    let theta_2 = point_angle(&ORIGIN, v1);

    let mut theta = (theta_2 - theta_1).abs();
    if theta > std::f64::consts::PI {
        theta = 2.0 * std::f64::consts::PI - theta;
    }
    theta
}

pub fn norm(point: &Point) -> f32 {
    f64::sqrt((point.x * point.x + point.y * point.y) as f64) as f32
}

pub fn normalize(point: &mut Point) -> () {
    let len = norm(point);
    point.x = point.x / len;
    point.y = point.y / len;
}

// add 180 degree to angle
pub fn invert_angle(angle: f64) -> f64 {
    let mut ret = angle + std::f64::consts::PI;
    if ret > std::f64::consts::PI {
        ret -= 2.0 * std::f64::consts::PI;
    } else if ret < -std::f64::consts::PI {
        ret += 2.0 * std::f64::consts::PI;
    }
    ret
}

// calculate distance of angle (<= pi)
pub fn diff_angle(angle0: f64, angle1: f64) -> f64 {
    let result = if angle0 > angle1 {
        angle0 - angle1
    } else {
        angle1 - angle0
    };
    if result > std::f64::consts::PI {
        2.0 * std::f64::consts::PI - result
    } else {
        result
    }
}
