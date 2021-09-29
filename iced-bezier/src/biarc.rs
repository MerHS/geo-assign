use iced::{
    canvas::path::Arc,
    canvas::{Frame, Path, Stroke},
    Color, Point,
};

use crate::util::*;

#[derive(Debug, Default)]
pub struct ArcData {
    pub angle0: f64,
    pub angle1: f64,
    pub angle2: f64,
    pub radius: f32,
    pub center: Point,
}

impl ArcData {
    pub fn draw(&self, frame: &mut Frame, color: &Color) {
        let curve = Path::new(|p| {
            ArcData::draw_arc(p, &self.center, self.radius, self.angle0, self.angle1);
            ArcData::draw_arc(p, &self.center, self.radius, self.angle1, self.angle2);
        });

        frame.stroke(&curve, Stroke::default().with_width(3.0).with_color(*color));
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

// AABB origin is bottom-left
#[derive(Debug, Default)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub h: f32,
    pub w: f32,
}

#[derive(Debug, Default)]
pub struct ArcNode {
    pub arc: Option<ArcData>,
    pub aabb: AABB,
    pub radius: f32,
}

impl ArcNode {
    pub fn draw_arc(&self, frame: &mut Frame, color: &Color) {
        if let Some(arc) = self.arc {
            arc.draw(frame, color)
        }
    }

    pub fn draw_aabb(&self, frame: &mut Frame, color: &Color) {
        let AABB { x, y, h, w } = self.aabb;
        let r = self.radius;
        if r <= 0.0 {
            let bound_box = Path::new(|p| {
                p.move_to(Point { x, y });
                p.line_to(Point { x: x + w, y });
                p.line_to(Point { x: x + w, y: y + h });
                p.line_to(Point { x: x, y: y + h });
                p.line_to(Point { x: x, y: y });
            });
            frame.stroke(
                &bound_box,
                Stroke::default().with_width(2.0).with_color(*color),
            );
        } else {
            let bound_box = Path::new(|p| {
                // draw edges
                p.move_to(Point { x: x, y: y - r });
                p.line_to(Point { x: x + w, y: y - r });
                p.move_to(Point { x: x + w + r, y });
                p.line_to(Point {
                    x: x + w + r,
                    y: y + h,
                });
                p.move_to(Point {
                    x: x + w,
                    y: y + h + r,
                });
                p.line_to(Point { x: x, y: y + h + r });
                p.move_to(Point { x: x - r, y: y + h });
                p.line_to(Point { x: x - r, y: y });

                // draw circle on vertex
                // CAVEAT: the entire canvas is flipped upside-down
                p.arc(Arc {
                    center: Point { x, y },
                    radius: r,
                    start_angle: std::f32::consts::FRAC_PI_2,
                    end_angle: std::f32::consts::PI,
                });

                p.arc(Arc {
                    center: Point { x: x + w, y },
                    radius: r,
                    start_angle: 0.0,
                    end_angle: std::f32::consts::FRAC_PI_2,
                });

                p.arc(Arc {
                    center: Point { x: x + w, y: y + h },
                    radius: r,
                    start_angle: -std::f32::consts::FRAC_PI_2,
                    end_angle: 0.0,
                });

                p.arc(Arc {
                    center: Point { x: x, y: y + h },
                    radius: r,
                    start_angle: -std::f32::consts::FRAC_PI_2,
                    end_angle: -std::f32::consts::PI,
                });
            });
            frame.stroke(
                &bound_box,
                Stroke::default().with_width(2.0).with_color(*color),
            );
        }
    }
}
