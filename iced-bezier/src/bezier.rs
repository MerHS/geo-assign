use iced::{
    canvas::event::{self, Event},
    canvas::{self, Cursor, Frame, Geometry, Path, Stroke},
    mouse, Color, Point, Rectangle,
};

use std::cell::RefCell;
use std::rc::Rc;

use crate::biarc::*;
use crate::tree::*;
use crate::util::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    Moving(usize, Point),
    Static,
}

#[derive(Debug)]
pub struct State {
    cache: canvas::Cache,
    curve: BezierCurve,
    arcs: Rc<RefCell<Tree<ArcBox>>>,
    control: Control,
    pub is_dotted: bool,
    pub is_meshed: bool,
    pub num_split: usize,
    pub aabb_depth: usize,
    pub left_color: Color,
    pub right_color: Color,
}

impl State {
    pub fn new() -> State {
        let default_num_split = 1;
        let curve = BezierCurve::default();
        let depth = default_num_split + 1;
        let arcs = Rc::new(RefCell::new(Tree::new_complete(
            depth,
            ArcBox::arc_builder(depth),
        )));

        curve.build_biarc(arcs.clone(), default_num_split);

        State {
            cache: Default::default(),
            curve,
            arcs,
            control: Control::Static,
            is_dotted: false,
            is_meshed: true,
            num_split: default_num_split,
            aabb_depth: 1usize,
            left_color: Color::from_rgba8(40, 210, 0, 1.0),
            right_color: Color::from_rgba8(30, 0, 210, 1.0),
        }
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

    pub fn toggle_dotted(&mut self) {
        self.is_dotted = !self.is_dotted;
        self.request_redraw();
    }

    pub fn toggle_meshed(&mut self) {
        self.is_meshed = !self.is_meshed;
        self.request_redraw();
    }

    pub fn set_num_biarc(&mut self, num_biarc: usize) {
        self.num_split = num_biarc;
        if self.aabb_depth > num_biarc + 1 {
            self.aabb_depth = num_biarc + 1;
        }
        self.curve.build_biarc(self.arcs.clone(), self.num_split);
        self.request_redraw();
    }

    pub fn set_aabb_depth(&mut self, aabb_depth: usize) {
        self.aabb_depth = aabb_depth;
        self.request_redraw();
    }

    fn draw_frame(&self, frame: &mut Frame) {
        // draw control meshes
        if self.is_meshed {
            let mesh = Path::new(|p| {
                let pts = self.curve.control_pts;
                p.move_to(pts[0]);
                for i in 1..4 {
                    p.line_to(pts[i]);
                }
            });
            frame.stroke(
                &mesh,
                Stroke::default()
                    .with_width(2.0)
                    .with_color(Color::from_rgba8(20, 210, 0, 1.0)),
            );
        }

        // draw bezier curve
        self.curve.draw(frame, self.is_dotted);
        frame.stroke(
            &Path::rectangle(Point::ORIGIN, frame.size()),
            Stroke::default(),
        );

        // draw biarcs
        if self.is_meshed {
            let mut color_idx: i64 = 0;
            self.draw_node(frame, self.arcs.borrow().get(0).unwrap(), &mut color_idx, 0);
        }

        // draw control points
        for ctr_point in self.curve.control_pts {
            let point_circ = Path::circle(ctr_point, PTS_RADIUS * 2.0);
            frame.fill(&point_circ, Color::from_rgba8(255, 0, 0, 1.0));
        }
    }

    fn draw_node(&self, frame: &mut Frame, node: &Node<ArcBox>, color_idx: &mut i64, depth: usize) {
        let tree = self.arcs.borrow();

        if let Some(left_node) = tree.left(node) {
            self.draw_node(frame, left_node, color_idx, depth + 1);
        }

        if let Some(right_node) = tree.right(node) {
            self.draw_node(frame, right_node, color_idx, depth + 1);
        }

        if node.arc.is_some() {
            let color = if *color_idx % 2 == 0 {
                self.left_color
            } else {
                self.right_color
            };
            node.draw_arc(frame, &color);
            *color_idx += 1;
        }

        if depth + self.aabb_depth >= self.num_split + 2 {
            node.draw_aabb(frame, &Color::from_rgba8(0, 30, 220, 1.0));
        }
    }
}

impl<Message> canvas::Program<Message> for State {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if cursor.position_in(&bounds).is_none() {
            return (event::Status::Ignored, None);
        }

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    for i in 0..4 {
                        let ctr_pts = self.curve.control_pts[i];

                        // make clickable range * 1.5
                        let local_rad = PTS_RADIUS * 1.5;
                        let pts_bound = Rectangle {
                            x: bounds.x + ctr_pts.x - local_rad,
                            y: bounds.y + ctr_pts.y - local_rad,
                            height: 2.0 * local_rad,
                            width: 2.0 * local_rad,
                        };
                        if let Some(in_pos) = cursor.position_in(&pts_bound) {
                            self.control = Control::Moving(
                                i,
                                Point {
                                    x: pts_bound.x + in_pos.x,
                                    y: pts_bound.y + in_pos.y,
                                },
                            );
                            break;
                        }
                    }
                    (event::Status::Captured, None)
                }
                mouse::Event::CursorMoved { position } => {
                    if let Control::Moving(idx, _) = self.control {
                        let pts = Point {
                            x: position.x - bounds.x,
                            y: position.y - bounds.y,
                        };
                        self.control = Control::Moving(idx, pts);
                        self.curve.control_pts[idx] = pts;
                        self.curve.build_biarc(self.arcs.clone(), self.num_split);
                    }
                    (event::Status::Captured, None)
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    if let Control::Moving(_, _) = self.control {
                        self.control = Control::Static;
                        self.cache.clear();
                    }
                    (event::Status::Captured, None)
                }
                _ => (event::Status::Ignored, None),
            },
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        if Control::Static == self.control {
            let content = self.cache.draw(bounds.size(), |frame: &mut Frame| {
                self.draw_frame(frame);
            });
            vec![content]
        } else {
            let mut frame = Frame::new(bounds.size());
            self.draw_frame(&mut frame);
            let content = frame.into_geometry();
            vec![content]
        }
    }

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

#[derive(Debug)]
pub struct BezierCurve {
    control_pts: [Point; 4],
}

impl BezierCurve {
    fn draw(&self, frame: &mut Frame, is_dotted: bool) {
        let curve = Path::new(|p| {
            let mut point = Point::default();
            let mut dot_start = true;
            p.move_to(self.control_pts[0]);
            for i in 1..=RESOLUTION {
                let t = (i as f32) / (RESOLUTION as f32);
                self.cubic_curve_to(&mut point, t);

                if is_dotted {
                    if dot_start {
                        p.line_to(point);
                    } else {
                        p.move_to(point);
                    }
                    dot_start = !dot_start;
                } else {
                    p.line_to(point);
                }
            }
        });

        frame.stroke(&curve, Stroke::default().with_width(1.2));
    }

    fn cubic_curve_to(&self, point: &mut Point, t: f32) -> () {
        let t_inv = 1.0 - t;
        let t_inv_sq = t_inv * t_inv;
        let t_sq = t * t;
        let b0 = t_inv_sq * t_inv;
        let b1 = 3.0 * t_inv_sq * t;
        let b2 = 3.0 * t_inv * t_sq;
        let b3 = t_sq * t;
        point_clear(point);
        point_add_weight_vec(point, b0, &self.control_pts[0]);
        point_add_weight_vec(point, b1, &self.control_pts[1]);
        point_add_weight_vec(point, b2, &self.control_pts[2]);
        point_add_weight_vec(point, b3, &self.control_pts[3]);
    }

    fn cubic_deriv_to(&self, point: &mut Point, t: f32) -> () {
        let t_inv = 1.0 - t;

        let b0 = 3.0 * t_inv * t_inv;
        let b1 = 6.0 * t * t_inv;
        let b2 = 3.0 * t * t;
        let p0 = &self.control_pts[0];
        let p1 = &self.control_pts[1];
        let p2 = &self.control_pts[2];
        let p3 = &self.control_pts[3];
        point_clear(point);
        point.x = b0 * (p1.x - p0.x) + b1 * (p2.x - p1.x) + b2 * (p3.x - p2.x);
        point.y = b0 * (p1.y - p0.y) + b1 * (p2.y - p1.y) + b2 * (p3.y - p2.y);
    }

    fn build_biarc(&self, arc_cell: Rc<RefCell<Tree<ArcBox>>>, split_num: usize) -> () {
        let depth = split_num + 1;
        let node_n = 2usize.pow((depth + 1) as u32) - 1;
        let biarc_n = 2usize.pow(split_num as u32);

        if node_n != arc_cell.borrow().len() {
            let mut arc_mut = arc_cell.borrow_mut();
            arc_mut.set_new_complete(depth, ArcBox::arc_builder(depth));
        }

        let delta = 1.0 / (biarc_n as f32);
        let mut start = Point::default();
        let mut end = Point::default();
        let mut control = Point::default();
        let mut u0 = Point::default();
        let mut u1 = Point::default();
        let mut mid0 = Point::default();
        let mut mid1 = Point::default();
        let mut v0 = Point::default();
        let mut v1 = Point::default();
        let mut center = Point::default();

        let mut arc_mid = Point::default();
        let mut tangent_left = Point::default();
        let mut tangent_right = Point::default();
        let mut tangent_mid = Point::default();

        let mut i: i32 = 0;
        let mut is_left: bool = true;

        Tree::post_trav(arc_cell.clone(), |node_id| {
            // TODO: merge radius
            let mut left_aabb: Option<AABB> = None;
            let mut right_aabb: Option<AABB> = None;
            let mut aabb_radius: f32 = 0.0;

            {
                let tree = arc_cell.borrow();
                let node = tree.get(node_id).unwrap();
                if let Some(left_node) = tree.left(node) {
                    left_aabb = Some(left_node.aabb.clone());
                    aabb_radius = left_node.radius;
                }
                if let Some(right_node) = tree.right(node) {
                    right_aabb = Some(right_node.aabb.clone());
                    if aabb_radius < right_node.radius {
                        aabb_radius = right_node.radius;
                    }
                }

                if let Some(ref left_value) = left_aabb {
                    if let Some(ref right_value) = right_aabb {
                        left_aabb = Some(AABB::merge_two(left_value, right_value));
                    }
                } else {
                    if right_aabb.is_some() {
                        left_aabb = right_aabb
                    }
                }
            }

            let mut tree = arc_cell.borrow_mut();
            let arc_node = &mut tree.get_mut(node_id).unwrap().value;

            // leaf node
            if let Some(ref mut arc) = arc_node.arc {
                // cache joint circle
                if is_left {
                    let t = delta * (i as f32);
                    i += 1;
                    self.cubic_curve_to(&mut start, t);
                    self.cubic_deriv_to(&mut u0, t);
                    normalize(&mut u0);

                    self.cubic_curve_to(&mut end, t + delta);
                    self.cubic_deriv_to(&mut u1, t + delta);
                    normalize(&mut u1);

                    // calculate the center of joint circle
                    mid0.x = (start.x + end.x) / 2.0;
                    mid0.y = (start.y + end.y) / 2.0;
                    mid1.x = (start.x + u0.x + end.x + u1.x) / 2.0;
                    mid1.y = (start.y + u0.y + end.y + u1.y) / 2.0;
                    v0.x = end.y - start.y;
                    v0.y = -(end.x - start.x);
                    v1.x = (end.y + u1.y) - (start.y + u0.y);
                    v1.y = (start.x + u0.x) - (end.x + u1.x);
                    ray_intersection(&mid0, &v0, &mid1, &v1, &mut center);

                    // calculate radius and control point
                    let radius = distance(&center, &start);
                    let theta = point_angle(&center, &mid0);
                    control.x = center.x + (radius * theta.cos()) as f32;
                    control.y = center.y + (radius * theta.sin()) as f32;

                    // calculate the center and angles of left arc
                    arc_mid.x = (start.x + control.x) / 2.0;
                    arc_mid.y = (start.y + control.y) / 2.0;
                    tangent_left.x = u0.y;
                    tangent_left.y = -u0.x;
                    tangent_right.x = control.y - start.y;
                    tangent_right.y = -(control.x - start.x);
                    ray_intersection(
                        &start,
                        &tangent_left,
                        &arc_mid,
                        &tangent_right,
                        &mut arc.center,
                    );

                    arc.radius = distance(&arc.center, &start) as f32;
                    arc.angle0 = point_angle(&arc.center, &start);
                    arc.angle1 = point_angle(&arc.center, &arc_mid);
                    arc.angle2 = point_angle(&arc.center, &control);
                    arc_mid.x = arc.center.x + (radius * arc.angle1.cos()) as f32;
                    arc_mid.y = arc.center.y + (radius * arc.angle1.sin()) as f32;

                    // is left arc is larger than half-circle?
                    let chord_vec = Point {
                        x: control.x - start.x,
                        y: control.y - start.y,
                    };
                    let l1_angle = vec_angle(&chord_vec, &u0);

                    // then invert mid-angle
                    if l1_angle > std::f64::consts::FRAC_PI_2 {
                        arc.angle1 = invert_angle(arc.angle1);
                    }

                    // calculate aabb radius
                    tangent_right.x = control.y - center.y;
                    tangent_right.y = center.x - control.x;
                    ray_intersection(&start, &u0, &control, &tangent_right, &mut tangent_mid);

                    tangent_left.x = (start.x + tangent_mid.x * 2.0) / 3.0;
                    tangent_left.y = (start.y + tangent_mid.y * 2.0) / 3.0;
                    tangent_right.x = (control.x + tangent_mid.x * 2.0) / 3.0;
                    tangent_right.y = (control.y + tangent_mid.y * 2.0) / 3.0;

                    // inlined cubic curve calculation
                    point_clear(&mut tangent_mid);
                    point_add_weight_vec(&mut tangent_mid, 0.125, &start);
                    point_add_weight_vec(&mut tangent_mid, 0.375, &tangent_left);
                    point_add_weight_vec(&mut tangent_mid, 0.375, &tangent_right);
                    point_add_weight_vec(&mut tangent_mid, 0.375, &control);

                    arc_node.radius = distance(&arc_mid, &tangent_mid) as f32;

                    println!("rad: {}", arc_node.radius);

                    let mut dist_max = 0.0;

                    {
                        let tn = t + delta;
                        let tn_inv = 1.0 - tn;
                        let t_sq = t * t;
                        let tn_sq = tn * tn;
                        let t_inv = 1.0 - t;
                        let t_inv_sq = t_inv * t_inv;
                        let tn_inv_sq = tn_inv * tn_inv;
                        let t_t_inv = t * t_inv;
                        let tn_tn_inv = tn * tn_inv;

                        let mid_control_left = Point {
                            x: tn_inv
                                * (t_inv_sq * self.control_pts[0].x
                                    + 2. * t_t_inv * self.control_pts[1].x
                                    + t_sq * self.control_pts[2].x)
                                + tn * (t_inv_sq * self.control_pts[1].x
                                    + 2. * t_t_inv * self.control_pts[2].x
                                    + t_sq * self.control_pts[3].x),
                            y: tn_inv
                                * (t_inv_sq * self.control_pts[0].y
                                    + 2. * t_t_inv * self.control_pts[1].y
                                    + t_sq * self.control_pts[2].y)
                                + tn * (t_inv_sq * self.control_pts[1].y
                                    + 2. * t_t_inv * self.control_pts[2].y
                                    + t_sq * self.control_pts[3].y),
                        };
                        let mid_control_right = Point {
                            x: t_inv
                                * (tn_inv_sq * self.control_pts[0].x
                                    + 2. * tn_tn_inv * self.control_pts[1].x
                                    + tn_sq * self.control_pts[2].x)
                                + t * (tn_inv_sq * self.control_pts[1].x
                                    + 2. * tn_tn_inv * self.control_pts[2].x
                                    + tn_sq * self.control_pts[3].x),
                            y: t_inv
                                * (tn_inv_sq * self.control_pts[0].y
                                    + 2. * tn_tn_inv * self.control_pts[1].y
                                    + tn_sq * self.control_pts[2].y)
                                + t * (tn_inv_sq * self.control_pts[1].y
                                    + 2. * tn_tn_inv * self.control_pts[2].y
                                    + tn_sq * self.control_pts[3].y),
                        };
                        let mut dist_left = distance(&mid_control_left, &tangent_left);
                        if dist_max < dist_left {
                            dist_max = dist_left;
                        }
                        dist_left = distance(&mid_control_right, &tangent_right);
                        if dist_max < dist_left {
                            dist_max = dist_left;
                        }
                    }

                    arc_node.radius += dist_max as f32;
                    println!("rad: {}", arc_node.radius);
                } else {
                    // calculate the center and angles of right arc
                    arc_mid.x = (end.x + control.x) / 2.0;
                    arc_mid.y = (end.y + control.y) / 2.0;
                    tangent_left.x = u1.y;
                    tangent_left.y = -u1.x;
                    tangent_right.x = control.y - end.y;
                    tangent_right.y = -(control.x - end.x);
                    ray_intersection(
                        &end,
                        &tangent_left,
                        &arc_mid,
                        &tangent_right,
                        &mut arc.center,
                    );

                    arc.radius = distance(&arc.center, &end) as f32;
                    arc.angle0 = point_angle(&arc.center, &control);
                    arc.angle1 = point_angle(&arc.center, &arc_mid);
                    arc.angle2 = point_angle(&arc.center, &end);

                    // is right arc is larger than half-circle?
                    let chord_vec = Point {
                        x: end.x - control.x,
                        y: end.y - control.y,
                    };
                    let r1_angle = vec_angle(&chord_vec, &u1);

                    // then invert mid-angle
                    if r1_angle > std::f64::consts::FRAC_PI_2 {
                        arc.angle1 = invert_angle(arc.angle1);
                    }
                }
                is_left = !is_left;

                // calculate aabb
                arc_node.aabb = arc.aabb();
            } else {
                if let Some(left_value) = left_aabb {
                    arc_node.aabb = left_value;
                    arc_node.radius = aabb_radius;
                }
            }
        });
    }
}

impl Default for BezierCurve {
    fn default() -> Self {
        Self {
            control_pts: [
                Point { x: 50.0, y: 100.0 },
                Point { x: 200.0, y: 300.0 },
                Point { x: 400.0, y: 300.0 },
                Point { x: 550.0, y: 100.0 },
            ],
        }
    }
}
