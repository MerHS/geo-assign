use iced::Point;

pub fn point_clear(point: &mut Point) -> () {
    point.x = 0.0;
    point.y = 0.0;
}

pub fn point_add_weight_vec(point: &mut Point, weight: f32, vec: Point) {
    point.x += weight * vec.x;
    point.y += weight * vec.y;
}
