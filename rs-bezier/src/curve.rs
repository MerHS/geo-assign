use crate::settings::Ids;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub value: [f32; 2],
}

#[derive(Debug, PartialEq)]
pub struct CubicBezierCurve {
    pub control_pts: [Point; 4],
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { value: [x, y] }
    }
}

impl CubicBezierCurve {
    pub fn new() -> Self {
        CubicBezierCurve {
            control_pts: [
                Point::new(50., 100.),
                Point::new(200., 300.),
                Point::new(400., 300.),
                Point::new(550., 100.),
            ],
        }
    }

    pub fn evaluate(&self, t: f32, point: &mut Point) -> () {
        let t_inv = 1.0 - t;
        let t_inv_sq = t_inv * t_inv;
        let t_sq = t * t;
        let b0 = t_inv_sq * t_inv;
        let b1 = 3.0 * t_inv_sq * t;
        let b2 = 3.0 * t_inv * t_sq;
        let b3 = t_sq * t;

        point.set_value(0.0, 0.0);
        point.vec_scalar_add(&self.control_pts[0], b0);
        point.vec_scalar_add(&self.control_pts[1], b1);
        point.vec_scalar_add(&self.control_pts[2], b2);
        point.vec_scalar_add(&self.control_pts[3], b3);
    }
}

impl Point {
    fn set_value(&mut self, v1: f32, v2: f32) -> () {
        self.value[0] = v1;
        self.value[1] = v2;
    }

    fn vec_scalar_add(&mut self, v: &Point, s: f32) -> () {
        self.value[0] += s * v.value[0];
        self.value[1] += s * v.value[1];
    }
}
