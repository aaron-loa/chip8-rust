use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;
pub struct Display {
    canvas: Canvas<Window>,
    black: pixels::Color,
    white: pixels::Color,
    rectangle: sdl2::rect::Rect,
}

impl Display {
    pub fn new(_ctx: &sdl2::Sdl) -> Display {
        Display {
            canvas: {
                let window =
                    sdl2::VideoSubsystem::window(&_ctx.video().unwrap(), "chip8", 640, 320)
                        .opengl()
                        .resizable()
                        .position_centered()
                        .build()
                        .unwrap();
                window.into_canvas().build().unwrap()
            },
            white: pixels::Color::RGB(255, 255, 255),
            black: pixels::Color::RGB(0, 0, 0),
            rectangle: sdl2::rect::Rect::new(0, 0, 10, 10),
        }
    }
    pub fn draw(&mut self, x: [[bool; 64]; 32]) {
        for i in 0..32 as usize {
            for j in 0..64 as usize {
                self.rectangle.x = (j * 10) as i32;
                self.rectangle.y = (i * 10) as i32;
                self.canvas.set_draw_color(match x[i][j] {
                    true => self.white,
                    false => self.black,
                });
                self.canvas.fill_rect(self.rectangle).unwrap();
            }
        }
        self.canvas.present();
    }
}
