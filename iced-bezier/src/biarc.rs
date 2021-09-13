use iced::{
    canvas::event::{self, Event},
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke},
    mouse, Element, Length, Point, Rectangle,
};

use crate::util::*;

const RESOLUTION: usize = 100;

#[derive(Debug, Default)]
pub struct State {
    cache: canvas::Cache,
    curve: BezierCurve,
    is_dotted: bool,
    is_meshed: bool,
    num_biarc: usize,
}

impl State {
    pub fn new() -> State {
        State {
            cache: Default::default(),
            curve: Default::default(),
            is_dotted: false,
            is_meshed: true,
            num_biarc: 2,
        }
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
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
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => None,
                    //     match self.state.pending {
                    //     None => {
                    //         self.state.pending = Some(Pending::One {
                    //             from: cursor_position,
                    //         });

                    //         None
                    //     }
                    //     Some(Pending::One { from }) => {
                    //         self.state.pending = Some(Pending::Two {
                    //             from,
                    //             to: cursor_position,
                    //         });

                    //         None
                    //     }
                    //     Some(Pending::Two { from, to }) => {
                    //         self.state.pending = None;

                    //         Some(Curve {
                    //             from,
                    //             to,
                    //             control: cursor_position,
                    //         })
                    //     }
                    // },
                    _ => None,
                };

                (event::Status::Captured, message)
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.cache.draw(bounds.size(), |frame: &mut Frame| {
            self.curve.draw(frame);

            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default(),
            );
        });

        vec![content]

        // if let Some(pending) = &self.state.pending {
        //     let pending_curve = pending.draw(bounds, cursor);

        //     vec![content, pending_curve]
        // } else {
        //     vec![content]
        // }
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
    fn draw(&self, frame: &mut Frame) {
        let curve = Path::new(|p| {
            let mut point = Point::default();
            p.move_to(self.control_pts[0]);
            for i in 1..=RESOLUTION {
                let t = (i as f32) / (RESOLUTION as f32);
                self.cubic_curve_to(&mut point, t);
                p.line_to(point);
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
