use nannou::prelude::*;
use nannou::rand;
use rand_distr;
use rand_distr::Distribution;

struct Model {
    rng: rand::rngs::ThreadRng,
    normal: rand_distr::StandardNormal,
    points: Vec<Point2>,
}

impl Model {
    fn update(&mut self) {
        let last = self.points.last().unwrap();
        let new = *last
            + vec2(
                self.normal.sample(&mut self.rng),
                self.normal.sample(&mut self.rng),
            ) * 4.0; //vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        self.points.push(new);
    }

    fn draw(&self, draw: &Draw) {
        draw.background().color(WHITE);
        draw.polyline()
            .weight(1.0)
            .points_colored(self.points.iter().map(|p| (*p, STEELBLUE)));
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let points = vec![pt2(0.0, 0.0)];
    let normal = rand_distr::StandardNormal;
    let rng = rand::thread_rng();
    Model {
        rng,
        normal,
        points,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
