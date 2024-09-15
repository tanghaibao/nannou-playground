extern crate pest;
#[macro_use]
extern crate pest_derive;

#[allow(dead_code)]
mod koch;

use koch::{Koch, TurtleStep};

use crate::koch::KochModel;
use nannou::prelude::*;

struct Turtle {
    xy: Vec2,
    angle: f32,
    koch: Koch,
    length: f32,                    // length of step
    stack: Vec<(Vec2, f32)>,        // stack of (xy, angle)
    steps: Vec<(TurtleStep, Vec2)>, // all steps to draw
}

impl Turtle {
    fn new(x: f32, y: f32, angle: f32, koch: Koch, length: f32) -> Turtle {
        Self {
            xy: vec2(x, y),
            angle,
            koch,
            length,
            stack: vec![(vec2(x, y), angle)],
            steps: Vec::new(),
        }
    }

    fn get_delta(&self) -> f32 {
        self.koch.get_delta()
    }

    fn set_koch(&mut self, koch: Koch) {
        self.reset();
        self.koch = koch;
    }

    fn update(&mut self) {
        let length = self.length;
        let delta = self.get_delta();
        let step = self.koch.next_step();
        match step {
            TurtleStep::Forward(_) | TurtleStep::ForwardNoLine => self.forward_no_line(length),
            TurtleStep::TurnLeft => self.turn_left(delta),
            TurtleStep::TurnRight => self.turn_right(delta),
            TurtleStep::Push => self.push(),
            TurtleStep::Pop => self.pop(),
            TurtleStep::Reset => self.reset(),
            TurtleStep::Node(_) => (),
        }
        self.steps.push((step, self.xy));
    }

    fn draw(&self, app: &App, draw: &Draw) {
        let mut prev_point = pt2(0.0, 0.0);
        for (step, point) in self.steps.iter() {
            match step {
                TurtleStep::Forward(_) => self.forward(draw, prev_point, *point),
                _ => (),
            }
            prev_point = *point;
        }
        self.draw_title(app, draw);
    }

    fn draw_title(&self, app: &App, draw: &Draw) {
        let win = app.window_rect();
        let text_pos = Rect::from_w_h(100.0, 100.0).top_left_of(win.pad(30.0)).xy();
        draw.text(self.koch.get_name())
            .center_justify()
            .color(WHITE)
            .font_size(24)
            .xy(text_pos);
    }

    fn forward(&self, draw: &Draw, start_point: Point2, end_point: Point2) {
        draw.line().start(start_point).end(end_point).color(GREEN);
    }

    fn forward_no_line(&mut self, length: f32) {
        self.xy += {
            let angle = self.angle;
            vec2(angle.cos(), angle.sin()) * length
        };
    }

    fn turn_left(&mut self, delta: f32) {
        self.angle += delta;
    }

    fn turn_right(&mut self, delta: f32) {
        self.angle -= delta;
    }

    fn push(&mut self) {
        self.stack.push((self.xy, self.angle));
    }

    fn pop(&mut self) {
        let (xy, angle) = self.stack.pop().unwrap();
        self.xy = xy;
        self.angle = angle;
    }

    fn reset(&mut self) {
        while !self.stack.is_empty() {
            self.pop();
        }
        self.push();
        self.steps.clear();
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    turtle: Turtle,
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(1024, 768)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let turtle = Turtle::new(0.0, 0.0, 0.0, Koch::model(KochModel::Cyclone), 4.0);
    Model { turtle }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            let koch_model: KochModel = rand::random();
            let koch = Koch::model(koch_model);
            model.turtle.set_koch(koch);
        }
        Key::Q => app.quit(),
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.turtle.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.turtle.draw(app, &draw);
    draw.to_frame(app, &frame).unwrap();
}
