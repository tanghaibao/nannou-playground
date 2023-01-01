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
        let circle_color = LIGHTSTEELBLUE;
        let circle_stroke_weight = 1.0;

        draw.background().color(WHITE);

        for circle in &self.circles {
            draw.ellipse()
                .x_y(circle.center.x, circle.center.y)
                .radius(circle.radius)
                .stroke_color(circle_color)
                .stroke_weight(circle_stroke_weight)
                .no_fill();
            draw.line()
                .start(circle.center)
                .end(circle.tip())
                .color(circle_color)
                .weight(circle_stroke_weight);
        }

        // draw the set of points
        let point_color = DARKSLATEGRAY;
        let points = self.points.clone();
        draw.polyline()
            .color(point_color)
            .weight(3.0)
            .points(points);
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
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
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
