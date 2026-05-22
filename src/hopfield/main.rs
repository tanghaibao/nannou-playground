use candle_core::{Device, IndexOp, Tensor};
use nannou::image;
use nannou::prelude::*;
use std::thread::sleep;

/// Cell width and height
const CELL_WH: f32 = 3.0;
/// Number of pixels to update
const UPDATE_COUNT: u32 = 16;

struct Model {
    width: u32,
    height: u32,
    pixels: Vec<f64>,
    weights: Tensor,
}

fn main() {
    sleep(std::time::Duration::from_secs(1));
    // Show the image
    nannou::app(model).update(update).run();
}

impl Model {
    fn new() -> Self {
        let device = Device::Cpu;
        let dtype = candle_core::DType::F64;

        // Read jpg image
        let img = image::open("assets/girl-s.jpg").unwrap().to_rgb8();
        let (width, height) = img.dimensions();
        let n = (width * height) as usize;
        let mut pixels = vec![0.0; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let r = pixel[0] as f64 / 255.0;
                let g = pixel[1] as f64 / 255.0;
                let b = pixel[2] as f64 / 255.0;
                let val = (r + g + b) / 3.0 * 2.0 - 1.0;
                pixels[(y * width + x) as usize] = if val > 0.0 { 1.0 } else { -1.0 };
            }
        }
        println!("Image loaded: {}x{}", width, height);
        // Hebbian learning
        let pixels_rs = Tensor::from_vec(pixels.clone(), n, &device)
            .unwrap()
            .reshape((1, n))
            .unwrap();
        println!("Pixels reshaped: {:?}", pixels_rs.shape());
        println!("{:?}", pixels_rs);
        let eye = Tensor::eye(n, dtype, &device).unwrap();
        let weights = (pixels_rs.t().unwrap().matmul(&pixels_rs).unwrap() - eye).unwrap();
        println!("Weights calculated");
        println!("{:?}", weights);
        // Make a random pattern
        let mut pixels = vec![0.0; n];
        for i in 0..n {
            pixels[i] = if rand::random::<f64>() > 0.5 {
                1.0
            } else {
                -1.0
            };
        }
        Self {
            width,
            height,
            pixels,
            weights,
        }
    }

    fn update(&mut self) {
        // Update the model
        // Pick a random index and update the pixel
        let mut count = 0;
        let mut loop_count = 0;
        while count < UPDATE_COUNT && loop_count < 128 {
            let index = rand::random::<usize>() % (self.width * self.height) as usize;
            let mut new_pixel = 0.0;
            for i in 0..self.height as usize {
                let a = self
                    .weights
                    .i((index, i))
                    .unwrap()
                    .to_scalar::<f64>()
                    .unwrap();
                new_pixel += a * self.pixels[i];
            }
            let np = if new_pixel > 0.0 { 1.0 } else { -1.0 };
            if (np - self.pixels[index]).abs() > 1.0 {
                count += 1;
            }
            self.pixels[index] = np;
            loop_count += 1;
        }
    }

    fn draw(&self, draw: &nannou::draw::Draw) {
        // Draw the model
        let cell_width = CELL_WH;
        let cell_height = CELL_WH;
        let width: f32 = self.width as f32 * cell_width;
        let height = self.height as f32 * cell_height;
        let mut x = 0.0;
        let mut y = height;
        for pixel in self.pixels.iter() {
            // Duotone: map the bipolar pixel onto a deep-navy -> warm-cream ramp.
            let t = (pixel / 2.0 + 0.5) as f32;
            let r = 0.05 + t * (0.98 - 0.05);
            let g = 0.07 + t * (0.91 - 0.07);
            let b = 0.12 + t * (0.74 - 0.12);
            let color = srgb(r, g, b);
            let cell_xy = pt2(
                x - width / 2.0 + cell_width / 2.0,
                y - height / 2.0 + cell_height / 2.0,
            );
            let cell_wh = vec2(cell_width, cell_height);
            draw.rect().xy(cell_xy).wh(cell_wh).color(color);
            x += cell_width;
            if x >= width {
                x = 0.0;
                y -= cell_height;
            }
        }
    }
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(srgb(0.043, 0.047, 0.075));
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
