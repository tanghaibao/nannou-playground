use candle_core::{Device, IndexOp, Tensor};
use nannou::image;
use nannou::prelude::*;

struct Model {
    width: u32,
    height: u32,
    pixels: Vec<f32>,
    weights: Tensor,
}

fn main() {
    // Show the image
    nannou::app(model).update(update).run();
}

impl Model {
    fn new() -> Self {
        let device = Device::Cpu;
        let dtype = candle_core::DType::F32;

        // Read jpg image
        let img = image::open("assets/girl-s.jpg").unwrap().to_rgb8();
        let (width, height) = img.dimensions();
        let n = (width * height) as usize;
        let mut pixels = vec![0.0f32; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let r = pixel[0] as f32 / 255.0;
                let g = pixel[1] as f32 / 255.0;
                let b = pixel[2] as f32 / 255.0;
                // Scale to [-1, 1]
                pixels[(y * width + x) as usize] = (r + g + b) / 3.0 * 2.0 - 1.0;
            }
        }
        println!("Image loaded: {}x{}", width, height);
        // Hebbian learning
        let pixels_rs = Tensor::from_vec(pixels, n, &device)
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
        let pixels = Tensor::rand(-1 as f32, 1 as f32, n, &device)
            .unwrap()
            .to_vec1()
            .unwrap();
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
        let index = rand::random::<usize>() % (self.width * self.height) as usize;
        let mut new_pixel = 0.0;
        for i in 0..self.height as usize {
            let a = self
                .weights
                .i((index, i))
                .unwrap()
                .to_scalar::<f32>()
                .unwrap();
            new_pixel += a * self.pixels[i];
        }
        self.pixels[index] = new_pixel;
    }

    fn draw(&self, draw: &nannou::draw::Draw) {
        // Draw the model
        let width = self.width as f32;
        let height = self.height as f32;
        let mut x = 0.0;
        let mut y = height;
        let cell_width = 1.0;
        let cell_height = 1.0;
        for pixel in self.pixels.iter() {
            let pixel = pixel / 2.0 + 0.5;
            let color = srgba(pixel, pixel, pixel, 1.0);
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
    draw.background().color(LIGHTSLATEGRAY);
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
