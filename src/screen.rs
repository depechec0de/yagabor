use sdl2::{VideoSubsystem, render::Canvas, video::Window, pixels::Color, rect::Point};

use crate::gameboy::io::lcd::{Pixel, Frame};

pub struct Screen {
    canvas: Canvas<Window>
}

impl Screen {
    fn color(pixel: &Pixel) -> Color {
        match pixel {
            Pixel::White => Color::RGB(255, 255, 255),
            Pixel::LightGray => Color::RGB(192, 192, 192),
            Pixel::DarkGray => Color::RGB(96, 96, 96),
            Pixel::Black => Color::RGB(0, 0, 0),
        }
    }
}

impl Screen {
    pub fn new(video: VideoSubsystem) -> Screen {
        let window = video.window("Game Boy", 160, 144)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        Screen { canvas }
    }

    pub(crate) fn render(&mut self, frame: Frame) {
        self.canvas.clear();

        for (x, row) in frame.iter().enumerate() {
            for (y, pixel) in row.iter().enumerate() {
                self.canvas.set_draw_color(Screen::color(pixel));
                let point = Point::new(x as i32, y as i32);
                self.canvas.draw_point(point).unwrap();
            }
        }
                
        self.canvas.present();
    }
}