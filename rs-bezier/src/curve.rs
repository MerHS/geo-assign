pub type Point = [f32; 2];

#[derive(Debug)]
pub struct CubicBezierCurve {
    pub control_pts: [Point; 4],
}

pub fn evaluate(curve: &CubicBezierCurve, t: f32, value: &mut Point) -> () {
    let t_inv = 1.0 - t;
    let t_inv_sq = t_inv * t_inv;
    let t_sq = t * t;
    let b0 = t_inv_sq * t_inv;
    let b1 = 3.0 * t_inv_sq * t;
    let b2 = 3.0 * t_inv * t_sq;
    let b3 = t_sq * t;
    set_vector2(value, 0.0, 0.0);
    vector2_x_scalar_add(value, &curve.control_pts[0], b0);
    vector2_x_scalar_add(value, &curve.control_pts[1], b1);
    vector2_x_scalar_add(value, &curve.control_pts[2], b2);
    vector2_x_scalar_add(value, &curve.control_pts[3], b3);
}

fn set_vector2(v: &mut Point, v1: f32, v2: f32) -> () {
    v[0] = v1;
    v[1] = v2;
}

fn vector2_x_scalar_add(o: &mut Point, v: &Point, s: f32) -> () {
    o[0] += s * v[0];
    o[1] += s * v[1];
}
