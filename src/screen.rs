use sdl2::{VideoSubsystem, render::Canvas, video::Window, pixels::Color, rect::Point};

use crate::gameboy::io::lcd::{ColoredPixel, Frame, SCREEN_WIDTH, SCREEN_HEIGHT};

pub struct Screen {
    canvas: Canvas<Window>
}

impl Screen {
    pub(crate) fn color(pixel: ColoredPixel) -> Color {
        match pixel {
            ColoredPixel::White => Color::RGB(255, 255, 255),
            ColoredPixel::LightGray => Color::RGB(192, 192, 192),
            ColoredPixel::DarkGray => Color::RGB(96, 96, 96),
            ColoredPixel::Black => Color::RGB(0, 0, 0),
        }
    }
}

impl Screen {
    pub fn new(video: &VideoSubsystem) -> Screen {
        let window = video.window("Game Boy", 160*4, 144*4)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.set_scale(4.0, 4.0).unwrap();
        canvas.clear();

        Screen { canvas }
    }

    pub(crate) fn render(&mut self, frame: Frame) {
        self.canvas.clear();

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let pixel = frame[x + y * SCREEN_WIDTH];
                self.canvas.set_draw_color(Screen::color(pixel));
                let point = Point::new(x as i32, y as i32);
                self.canvas.draw_point(point).unwrap();
            }
        }
                
        self.canvas.present();
    }
}