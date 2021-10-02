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

    pub fn draw_arc(
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

    pub fn aabb(&self) -> AABB {
        let mut aabb_left = self.aabb_inner(self.angle0, self.angle1);
        let aabb_right = self.aabb_inner(self.angle1, self.angle2);

        aabb_left.merge(&aabb_right);
        aabb_left
    }

    fn aabb_inner(&self, a0: f64, a1: f64) -> AABB {
        let pi = std::f64::consts::PI;
        let pi2 = std::f64::consts::FRAC_PI_2;

        let mut aabb = aabb = AABB::new_point(
            Point {
                x: self.center.x + self.radius * f32::cos(a0 as f32),
                y: self.center.y + self.radius * f32::sin(a0 as f32),
            },
            Point {
                x: self.center.x + self.radius * f32::cos(a1 as f32),
                y: self.center.y + self.radius * f32::sin(a1 as f32),
            },
        );
        if pi >= a0 && a0 >= pi2 {
            if pi >= a1 && a1 >= pi2 {
                // nop
            } else if pi2 > a1 && a1 >= 0.0 {

            } else if 0.0 > a1 && a1 >= -pi2 {
            } else if -pi2 > a1 && a1 >= -pi {
            }
        } else if pi2 > a0 && a0 >= 0.0 {
            if pi >= a1 && a1 >= pi2 {
            } else if pi2 > a1 && a1 >= 0.0 {
                // nop
            } else if 0.0 > a1 && a1 >= -pi2 {
            } else if -pi2 > a1 && a1 >= -pi {
            }
        } else if 0.0 > a0 && a0 >= -pi2 {
            if pi >= a1 && a1 >= pi2 {
            } else if pi2 > a1 && a1 >= 0.0 {
            } else if 0.0 > a1 && a1 >= -pi2 {
                // nop
            } else if -pi2 > a1 && a1 >= -pi {
            }
        } else if -pi2 > a0 && a0 >= -pi {
            if pi >= a1 && a1 >= pi2 {
            } else if pi2 > a1 && a1 >= 0.0 {
            } else if 0.0 > a1 && a1 >= -pi2 {
            } else if -pi2 > a1 && a1 >= -pi {
                // nop
            }
        }
        aabb
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

impl AABB {
    fn new_point(p0: Point, p1: Point) -> Self {
        AABB {
            x: if p0.x < p1.x { p0.x } else { p1.x },
            y: if p0.y < p1.y { p0.y } else { p1.y },
            h: f32::abs(p0.y - p1.y),
            w: f32::abs(p0.x - p1.x),
        }
    }

    fn merge(&mut self, other: &AABB) {
        if other.x < self.x {
            self.w += self.x - other.x;
            self.x = other.x;
        }
        if other.y < self.y {
            self.h += self.y - other.y;
            self.y = other.y;
        }
        if self.y + self.h < other.y + other.h {
            self.h = other.y + other.h - self.y;
        }
        if self.x + self.w < other.x + other.w {
            self.w = other.x + other.w - self.x;
        }
    }
}

#[derive(Debug, Default)]
pub struct ArcNode {
    pub arc: Option<ArcData>,
    pub aabb: AABB,
    pub radius: f32,
}

impl ArcNode {
    pub fn arc_builder(depth: usize) -> Box<dyn Fn(usize) -> ArcNode> {
        let leaf_id = 2usize.pow(depth as u32) - 1;
        Box::new(move |node_id| ArcNode {
            arc: if node_id >= leaf_id {
                Some(ArcData::default())
            } else {
                None
            },
            aabb: Default::default(),
            radius: 0.0,
        })
    }

    pub fn draw_arc(&self, frame: &mut Frame, color: &Color) {
        if let Some(ref arc) = self.arc {
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
                p.line_to(Point { x, y: y + h + r });
                p.move_to(Point { x: x - r, y: y + h });
                p.line_to(Point { x: x - r, y });

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
                    center: Point { x, y: y + h },
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
