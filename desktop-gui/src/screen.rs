use gameboy::io::lcd::{ColoredPixel, Frame};
use sdl2::{VideoSubsystem, render::Canvas, video::{Window, WindowPos}, pixels::Color, rect::Point};

pub struct Screen {
    canvas: Canvas<Window>,
    width: u32,
    height: u32,
}

fn color_from_pixel(pixel: ColoredPixel) -> Color {
    match pixel {
        ColoredPixel::White => Color::RGB(255, 255, 255),
        ColoredPixel::LightGray => Color::RGB(192, 192, 192),
        ColoredPixel::DarkGray => Color::RGB(96, 96, 96),
        ColoredPixel::Black => Color::RGB(0, 0, 0),
    }
}

impl Screen {
    pub fn new(video: &VideoSubsystem, title: &str, width: u32, height: u32, scale: u32, posx_offset: i32)  -> Screen {
        let mut window = video.window(title, width * scale, height * scale)
            .position_centered()
            .build()
            .unwrap();

        let (x, y) = window.position();
        window.set_position(WindowPos::Positioned(x+posx_offset), WindowPos::Positioned(y));

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.set_scale(scale as f32, scale as f32).unwrap();
        canvas.clear();

        Screen { canvas, width, height }
    }

    pub(crate) fn render(&mut self, frame: Frame) {
        self.canvas.clear();

        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let pixel = frame.buffer[x + y * (self.width as usize)];
                self.canvas.set_draw_color(color_from_pixel(pixel));
                let point = Point::new(x as i32, y as i32);
                self.canvas.draw_point(point).unwrap();
            }
        }
                
        self.canvas.present();
    }
}