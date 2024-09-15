use nannou::image;
use nannou::prelude::*;

struct Model {
    width: u32,
    height: u32,
    pixels: Vec<i32>,
}

fn main() {
    // Show the image
    nannou::app(model).update(update).run();
}

impl Model {
    fn new() -> Self {
        // Read jpg image
        let img = image::open("assets/girl.jpg").unwrap().to_rgb8();
        let (width, height) = img.dimensions();
        let mut pixels = vec![0; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let r = pixel[0] as i32;
                let g = pixel[1] as i32;
                let b = pixel[2] as i32;
                pixels[(y * width + x) as usize] = (r + g + b) / 3;
            }
        }
        println!("Image loaded: {}x{}", width, height);
        Self {
            width,
            height,
            pixels,
        }
    }

    fn update(&mut self) {
        // Update the model
    }

    fn draw(&self, draw: &nannou::draw::Draw) {
        // Draw the model
        let width = self.width as f32;
        let height = self.height as f32;
        let mut x = 0.0;
        let mut y = 0.0;
        let cell_width = 1.0;
        let cell_height = 1.0;
        for &pixel in self.pixels.iter() {
            let color = srgba(
                pixel as f32 / 255.0,
                pixel as f32 / 255.0,
                pixel as f32 / 255.0,
                1.0,
            );
            let cell_xy = pt2(
                x - width / 2.0 + cell_width / 2.0,
                y - height / 2.0 + cell_height / 2.0,
            );
            let cell_wh = vec2(cell_width, cell_height);
            draw.rect().xy(cell_xy).wh(cell_wh).color(color);
            x += cell_width;
            if x >= width {
                x = 0.0;
                y += cell_height;
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
