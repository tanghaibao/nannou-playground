use nannou::color::{Gradient, LinSrgba};
use nannou::prelude::*;

const GRID_SIZE: usize = 161;
const SQUARE_SIZE: f32 = 5.0;
const DA: f32 = 0.5; // diffusion rate for A
const DB: f32 = 0.25; // diffusion rate for B

const FEEDA: f32 = 0.041; // feed rate for A
const KILLB: f32 = 0.099; // kill rate for B
const REACTION: f32 = 1.0; // reaction rate

struct Model {
    ma: Vec<Vec<f32>>,
    mb: Vec<Vec<f32>>,
    gradient: Gradient<LinSrgba<f64>>,
}

impl Model {
    /// Place a square of B's at the given coordinates
    fn add_square(&mut self, x: usize, y: usize, width: usize) {
        for i in x - width..x + width {
            for j in y - width..y + width {
                self.ma[i][j] = 0.0;
                self.mb[i][j] = 1.0;
            }
        }
    }

    fn init() -> Self {
        let ma = vec![vec![1.0; GRID_SIZE]; GRID_SIZE];
        let mb = vec![vec![0.0; GRID_SIZE]; GRID_SIZE];
        let steelblue = LinSrgba::<f64>::new(0.2745, 0.5098, 0.7059, 1.0);
        let white = LinSrgba::<f64>::new(1.0, 1.0, 1.0, 1.0);
        let firebrick = LinSrgba::<f64>::new(0.698, 0.133, 0.133, 1.0);
        let colors = vec![firebrick, white, steelblue];
        let gradient = Gradient::new(colors);
        // Place a center square of B's
        let mut s = Self { ma, mb, gradient };
        // s.add_square(GRID_SIZE / 2, GRID_SIZE / 2, GRID_SIZE / 20);
        for _ in 0..10 {
            // Add random clusters
            let x = random_range(0, GRID_SIZE);
            let y = random_range(0, GRID_SIZE);
            s.add_square(x, y, GRID_SIZE / 60);
        }
        s
    }

    /// Diffuse the given matrix by the given rate
    fn diffuse(
        m: &Vec<Vec<f32>>,
        diffusion_rate: f32,
        feed_rate: f32,
        kill_rate: f32,
        reaction_rate: f32,
        reaction: &Vec<Vec<f32>>,
    ) -> Vec<Vec<f32>> {
        let mut next = m.clone();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let mut nei = 0.0;
                let mut diag = 0.0;
                if i > 0 {
                    nei += m[i - 1][j];
                    if j > 0 {
                        diag += m[i - 1][j - 1];
                    }
                    if j < GRID_SIZE - 1 {
                        diag += m[i - 1][j + 1];
                    }
                }
                if i < GRID_SIZE - 1 {
                    nei += m[i + 1][j];
                    if j > 0 {
                        diag += m[i + 1][j - 1];
                    }
                    if j < GRID_SIZE - 1 {
                        diag += m[i + 1][j + 1];
                    }
                }
                if j > 0 {
                    nei += m[i][j - 1];
                }
                if j < GRID_SIZE - 1 {
                    nei += m[i][j + 1];
                }
                let diffusion = 0.2 * nei + 0.05 * diag - m[i][j];
                next[i][j] = m[i][j] + diffusion_rate * diffusion + feed_rate * (1.0 - m[i][j])
                    - kill_rate * m[i][j]
                    + reaction_rate * reaction[i][j];
            }
        }
        next
    }

    fn update(&mut self) {
        // Reaction matrix
        let mut reaction = vec![vec![0.0; GRID_SIZE]; GRID_SIZE];
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                reaction[i][j] = self.ma[i][j] * self.mb[i][j] * self.mb[i][j];
            }
        }
        // Diffuse A and B
        let next_a = Self::diffuse(&self.ma, DA, FEEDA, 0.0, -REACTION, &reaction);
        let next_b = Self::diffuse(&self.mb, DB, 0.0, KILLB, REACTION, &reaction);
        // Swap the matrices
        self.ma = next_a;
        self.mb = next_b;
    }

    fn draw(&self, app: &App) {
        let draw = app.draw();
        draw.background().color(WHITE);
        // Draw the heatmap with a gradient
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let b = self.mb[i][j] / (self.ma[i][j] + self.mb[i][j]);
                let rgba = self.gradient.get(b as f64);
                draw.rect()
                    .x_y(
                        (i as f32 - GRID_SIZE as f32 / 2.0) * SQUARE_SIZE,
                        (j as f32 - GRID_SIZE as f32 / 2.0) * SQUARE_SIZE,
                    )
                    .w_h(SQUARE_SIZE, SQUARE_SIZE)
                    .color(rgba);
            }
        }
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model::init()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.draw(&app);
    draw.to_frame(app, &frame).unwrap();
}
