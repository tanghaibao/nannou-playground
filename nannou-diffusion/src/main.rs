use nannou::prelude::*;

const GRID_SIZE: usize = 151;
const SQUARE_SIZE: f32 = 5.0;
const DA: f32 = 0.2; // diffusion rate for A
const DB: f32 = 0.1; // diffusion rate for B

struct Model {
    ma: Vec<Vec<f32>>,
    mb: Vec<Vec<f32>>,
}

impl Model {
    fn init() -> Self {
        Self {
            ma: vec![vec![1.0; GRID_SIZE]; GRID_SIZE],
            mb: vec![vec![0.0; GRID_SIZE]; GRID_SIZE],
        }
    }

    /// Diffuse the given matrix by the given rate
    fn diffuse(m: &Vec<Vec<f32>>, rate: f32) -> Vec<Vec<f32>> {
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
                next[i][j] = (1.0 - rate) * m[i][j] + rate / 5.0 * nei + rate / 20.0 * diag;
            }
        }
        next
    }

    fn update(&mut self) {
        // Diffuse A and B
        let next_a = Self::diffuse(&self.ma, DA);
        let mut next_b = Self::diffuse(&self.mb, DB);
        // Place a center square of B's
        let center = GRID_SIZE / 2;
        let width = GRID_SIZE / 10;
        // Central square of B's expand outward within a uniform of A particles
        for i in center - width..center + width {
            for j in center - width..center + width {
                next_b[i][j] += 1.0;
            }
        }
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
