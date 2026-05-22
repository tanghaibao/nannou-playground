use nannou::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Circle {
    center: Point2,
    radius: f32,
    angular_speed: f32,
    theta: f32,
}

impl Circle {
    fn tip(&self) -> Point2 {
        let x = self.center.x + self.radius * self.theta.cos();
        let y = self.center.y + self.radius * self.theta.sin();
        pt2(x, y)
    }
}

struct Model {
    circles: Vec<Circle>,
    points: Vec<Point2>,
}

impl Model {
    fn new(circles: &[Circle]) -> Self {
        // make the circles nested
        let mut circles = circles.to_vec();
        for i in 1..circles.len() {
            circles[i].center = circles[i - 1].tip();
        }
        Self {
            circles,
            points: vec![],
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.background().color(srgb(0.043, 0.047, 0.075));

        // Faint epicycles and their radial arms.
        for circle in &self.circles {
            draw.ellipse()
                .x_y(circle.center.x, circle.center.y)
                .radius(circle.radius)
                .stroke_color(hsla(0.55, 0.5, 0.7, 0.18))
                .stroke_weight(1.0)
                .no_fill();
            draw.line()
                .start(circle.center)
                .end(circle.tip())
                .color(hsla(0.55, 0.6, 0.8, 0.35))
                .weight(1.2);
        }

        // Traced curve with a hue gradient sweeping along its length, plus glow.
        let n = self.points.len().max(1) as f32;
        let vertices = |alpha: f32| -> Vec<(Point2, Hsla)> {
            self.points
                .iter()
                .enumerate()
                .map(|(i, &p)| {
                    let t = i as f32 / n;
                    let hue = (0.55 + t * 0.6).fract();
                    (p, hsla(hue, 0.85, 0.6, alpha))
                })
                .collect()
        };
        draw.polyline().weight(6.0).points_colored(vertices(0.12));
        draw.polyline().weight(2.0).points_colored(vertices(0.95));

        // Glowing pen tip.
        if let Some(&tip) = self.points.last() {
            draw.ellipse()
                .xy(tip)
                .radius(5.0)
                .color(hsla(0.08, 0.9, 0.65, 0.3));
            draw.ellipse()
                .xy(tip)
                .radius(2.5)
                .color(hsla(0.12, 0.9, 0.95, 1.0));
        }
    }

    fn update(&mut self) {
        for circle in &mut self.circles {
            circle.theta += circle.angular_speed;
        }
        for i in 1..self.circles.len() {
            self.circles[i].center = self.circles[i - 1].tip();
        }
        let endpoint = self.circles.last().unwrap().tip();
        self.points.push(endpoint);
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(900, 900).view(view).build().unwrap();
    let circles: Vec<Circle> = (0..2)
        .rev()
        .map(|_| {
            let center = pt2(0.0, 0.0);
            let radius = random_range(10.0, 200.0);
            let angular_speed = random_range(0.03, 0.10);
            let theta = 0.0;
            Circle {
                center,
                radius,
                angular_speed,
                theta,
            }
        })
        .collect();
    Model::new(&circles)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
