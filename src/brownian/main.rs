use nannou::prelude::*;
use nannou::rand;
use rand_distr;
use rand_distr::Distribution;
use std::collections::VecDeque;

const POINTS: usize = 1000;

struct Model {
    rng: rand::rngs::ThreadRng,
    normal: rand_distr::StandardNormal,
    points: VecDeque<Point2>,
}

impl Model {
    fn update(&mut self) {
        let last = *self.points.front().unwrap();
        let new = last
            + vec2(
                self.normal.sample(&mut self.rng),
                self.normal.sample(&mut self.rng),
            ) * 4.0; //vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        self.points.push_front(new);
        if self.points.len() > POINTS {
            self.points.pop_back();
        }
    }

    fn draw(&self, app: &App) {
        let draw = app.draw();
        draw.background().color(WHITE);
        // Draw the lines in gradient
        let vertices: Vec<_> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let fract = 1.0 - i as f32 / self.points.len() as f32;
                let rgba = srgba(
                    STEELBLUE.red,
                    STEELBLUE.green,
                    STEELBLUE.blue,
                    (fract * 255.0) as u8,
                );
                (p, rgba)
            })
            .collect();
        draw.polyline()
            .weight(1.0)
            .join_round()
            .points_colored(vertices);
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let points = VecDeque::from(vec![pt2(0.0, 0.0)]);
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
    model.draw(&app);
    draw.to_frame(app, &frame).unwrap();
}
