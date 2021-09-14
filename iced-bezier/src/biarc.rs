use iced::{
    canvas::event::{self, Event},
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke},
    mouse, Color, Element, Length, Point, Rectangle,
};

use crate::util::*;

const RESOLUTION: usize = 100;
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
    control: Control,
    pub is_dotted: bool,
    pub is_meshed: bool,
    pub num_biarc: usize,
}

impl State {
    pub fn new() -> State {
        State {
            cache: Default::default(),
            curve: Default::default(),
            control: Control::Static,
            is_dotted: false,
            is_meshed: true,
            num_biarc: 2,
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

        // draw control points
        for ctr_point in self.curve.control_pts {
            let point_circ = Path::circle(ctr_point, PTS_RADIUS * 2.0);
            frame.fill(&point_circ, Color::from_rgba8(255, 0, 0, 1.0));
        }

        // TODO: draw biarcs
    }
}

impl<Message> canvas::Program<Message> for State {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };

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

    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
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

#[derive(Debug, Clone, Copy)]
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

        frame.stroke(&curve, Stroke::default().with_width(2.0));
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
        point_add_weight_vec(point, b0, self.control_pts[0]);
        point_add_weight_vec(point, b1, self.control_pts[1]);
        point_add_weight_vec(point, b2, self.control_pts[2]);
        point_add_weight_vec(point, b3, self.control_pts[3]);
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

// #[derive(Debug, Clone, Copy)]
// enum DragState {
//     One { from: Point },
//     Two { from: Point, to: Point },
// }

// impl Pending {
//     fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
//         let mut frame = Frame::new(bounds.size());

//         if let Some(cursor_position) = cursor.position_in(&bounds) {
//             match *self {
//                 Pending::One { from } => {
//                     let line = Path::line(from, cursor_position);
//                     frame.stroke(&line, Stroke::default().with_width(2.0));
//                 }
//                 Pending::Two { from, to } => {
//                     let curve = Curve {
//                         from,
//                         to,
//                         control: cursor_position,
//                     };

//                     Curve::draw_all(&[curve], &mut frame);
//                 }
//             };
//         }

//         frame.into_geometry()
//     }
// }
