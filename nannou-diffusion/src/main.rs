use nannou::prelude::*;

const GRID_SIZE: usize = 161;
const SQUARE_SIZE: f32 = 5.0;
const DA: f32 = 0.5; // diffusion rate for A
const DB: f32 = 0.25; // diffusion rate for B

const FEEDA: f32 = 0.04; // feed rate for A
const KILLB: f32 = 0.1; // kill rate for B
const REACTION: f32 = 1.0; // reaction rate

struct Model {
    ma: Vec<Vec<f32>>,
    mb: Vec<Vec<f32>>,
}

impl Model {
    fn init() -> Self {
        let mut ma = vec![vec![1.0; GRID_SIZE]; GRID_SIZE];
        let mut mb = vec![vec![0.0; GRID_SIZE]; GRID_SIZE];
        // Place a center square of B's
        let center = GRID_SIZE / 2;
        let width = GRID_SIZE / 20;
        // Central square of B's expand outward within a uniform of A particles
        for i in center - width..center + width {
            for j in center - width..center + width {
                ma[i][j] = 0.0;
                mb[i][j] = 1.0;
            }
        }
        Self { ma, mb }
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
                let a = self.ma[i][j] / (self.ma[i][j] + self.mb[i][j]);
                let rgba = if a > 0.5 {
                    srgba(
                        FIREBRICK.red,
                        FIREBRICK.green,
                        FIREBRICK.blue,
                        (2.0 * (a - 0.5) * 255.0) as u8,
                    )
                } else {
                    srgba(
                        STEELBLUE.red,
                        STEELBLUE.green,
                        STEELBLUE.blue,
                        (2.0 * (0.5 - a) * 255.0) as u8,
                    )
                };
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
