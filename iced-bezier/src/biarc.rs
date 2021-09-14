use iced::{
    canvas::event::{self, Event},
    canvas::{self, Cursor, Frame, Geometry, Path, Stroke},
    mouse, Color, Point, Rectangle,
};

use crate::util::*;

const RESOLUTION: usize = 100;
const RES_4: usize = RESOLUTION / 4;
const PTS_RADIUS: f32 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    Moving(usize, Point),
    Static,
}

#[derive(Debug)]
pub struct State {
    cache: canvas::Cache,
    curve: BezierCurve,
    arcs: Vec<Biarc>,
    control: Control,
    pub is_dotted: bool,
    pub is_meshed: bool,
    pub num_biarc: usize,
}

impl State {
    pub fn new() -> State {
        let default_num_biarc = 1;
        let curve = BezierCurve::default();
        let mut arcs = Vec::<Biarc>::new();
        curve.build_biarc(&mut arcs, default_num_biarc);
        State {
            cache: Default::default(),
            curve,
            arcs,
            control: Control::Static,
            is_dotted: false,
            is_meshed: true,
            num_biarc: default_num_biarc,
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
        self.num_biarc = num_biarc;
        self.curve.build_biarc(&mut self.arcs, self.num_biarc);
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
            for arc in self.arcs.iter() {
                arc.draw(frame);
            }
        }

        // draw control points
        for ctr_point in self.curve.control_pts {
            let point_circ = Path::circle(ctr_point, PTS_RADIUS * 2.0);
            frame.fill(&point_circ, Color::from_rgba8(255, 0, 0, 1.0));
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
                        self.curve.build_biarc(&mut self.arcs, self.num_biarc);
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

    fn build_biarc(&self, arcs: &mut Vec<Biarc>, num_biarc: usize) -> () {
        let mut pow_biarc = 2;
        for _ in 1..num_biarc {
            pow_biarc *= 2;
        }

        let len = arcs.len();
        if pow_biarc != len {
            arcs.resize(pow_biarc, Biarc::default());
        }

        let delta = 1.0 / (pow_biarc as f32);
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
        for (i, arc) in arcs.iter_mut().enumerate() {
            let t = delta * (i as f32);
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
            mid0.x = (start.x + control.x) / 2.0;
            mid0.y = (start.y + control.y) / 2.0;
            v0.x = u0.y;
            v0.y = -u0.x;
            v1.x = control.y - start.y;
            v1.y = -(control.x - start.x);
            ray_intersection(&start, &v0, &mid0, &v1, &mut arc.center_left);

            let mid_left = Point {
                x: (control.x + start.x) / 2.0,
                y: (control.y + start.y) / 2.0,
            };
            arc.rad_left = distance(&arc.center_left, &start) as f32;
            arc.theta_l0 = point_angle(&arc.center_left, &start);
            arc.theta_l2 = point_angle(&arc.center_left, &control);

            // is left arc is larger than half-circle?
            mid1.x = control.x - start.x;
            mid1.y = control.y - start.y;
            let l1_angle = vec_angle(&mid1, &u0);
            arc.theta_l1 = point_angle(&arc.center_left, &mid_left);

            // then invert mid-angle
            if l1_angle > std::f64::consts::FRAC_PI_2 {
                arc.theta_l1 = invert_angle(arc.theta_l1);
            }

            // calculate the center and angles of right arc
            mid1.x = (end.x + control.x) / 2.0;
            mid1.y = (end.y + control.y) / 2.0;
            v1.x = u1.y;
            v1.y = -u1.x;
            v0.x = control.y - end.y;
            v0.y = -(control.x - end.x);
            ray_intersection(&end, &v1, &mid1, &v0, &mut arc.center_right);

            let mid_right = Point {
                x: (control.x + end.x) / 2.0,
                y: (control.y + end.y) / 2.0,
            };
            arc.rad_right = distance(&arc.center_right, &end) as f32;
            arc.theta_r0 = point_angle(&arc.center_right, &control);
            arc.theta_r2 = point_angle(&arc.center_right, &end);

            // is right arc is larger than half-circle?
            mid0.x = end.x - control.x;
            mid0.y = end.y - control.y;
            let r1_angle = vec_angle(&mid0, &u1);
            arc.theta_r1 = point_angle(&arc.center_right, &mid_right);

            // then invert mid-angle
            if r1_angle > std::f64::consts::FRAC_PI_2 {
                arc.theta_r1 = invert_angle(arc.theta_r1);
            }
        }
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

#[derive(Debug, Copy, Clone, Default)]
pub struct Biarc {
    theta_l0: f64,
    theta_l1: f64,
    theta_l2: f64,
    theta_r0: f64,
    theta_r1: f64,
    theta_r2: f64,
    rad_left: f32,
    rad_right: f32,
    center_left: Point,
    center_right: Point,
}

impl Biarc {
    fn draw(&self, frame: &mut Frame) {
        let left_curve = Path::new(|p| {
            Biarc::draw_arc(
                p,
                &self.center_left,
                self.rad_left,
                self.theta_l0,
                self.theta_l1,
            );
            Biarc::draw_arc(
                p,
                &self.center_left,
                self.rad_left,
                self.theta_l1,
                self.theta_l2,
            );
            // let pi = std::f64::consts::PI;
            // println!(
            //     "{:>6.3} {:>6.3} {:>6.3}",
            //     180.0 * self.theta_l0 / pi,
            //     180.0 * self.theta_l1 / pi,
            //     180.0 * self.theta_l2 / pi
            // );
        });

        let right_curve = Path::new(|p| {
            Biarc::draw_arc(
                p,
                &self.center_right,
                self.rad_right as f32,
                self.theta_r0,
                self.theta_r1,
            );
            Biarc::draw_arc(
                p,
                &self.center_right,
                self.rad_right as f32,
                self.theta_r1,
                self.theta_r2,
            );
            // let pi = std::f64::consts::PI;
            // println!(
            //     "{:>6.3} {:>6.3} {:>6.3}",
            //     180.0 * self.theta_r0 / pi,
            //     180.0 * self.theta_r1 / pi,
            //     180.0 * self.theta_r2 / pi
            // );
        });

        frame.stroke(
            &left_curve,
            Stroke::default()
                .with_width(3.0)
                .with_color(Color::from_rgba8(40, 210, 0, 1.0)),
        );
        frame.stroke(
            &right_curve,
            Stroke::default()
                .with_width(3.0)
                .with_color(Color::from_rgba8(30, 0, 210, 1.0)),
        );
    }

    fn draw_arc(
        p: &mut iced::canvas::path::Builder,
        center: &Point,
        radius: f32,
        theta0: f64,
        theta1: f64,
    ) -> () {
        let mut angle0 = theta0;
        let angle1 = theta1;

        let mut delta = angle1 - angle0;

        // let delta below 180 degree.
        if angle0 > 0.0 {
            if delta < -std::f64::consts::PI {
                delta = std::f64::consts::PI * 2.0 + delta;
            }
        } else {
            if delta > std::f64::consts::PI {
                delta = delta - 2.0 * std::f64::consts::PI;
            }
        }

        delta = delta / (RES_4 as f64);

        let mut point = Point {
            x: center.x + radius * (angle0.cos() as f32),
            y: center.y + radius * (angle0.sin() as f32),
        };
        p.move_to(point);
        for _ in 1..=RES_4 {
            angle0 += delta;
            point.x = center.x + radius * (angle0.cos() as f32);
            point.y = center.y + radius * (angle0.sin() as f32);
            p.line_to(point);
        }
    }
}
