use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;
pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(_ctx: &sdl2::Sdl) -> Display {
        Display {
            canvas: {
                let window = sdl2::VideoSubsystem::window(
                    &_ctx.video().unwrap(),
                    "CHIP8 EMULATOR",
                    640,
                    320,
                )
                .opengl()
                .resizable()
                .position_centered()
                .build()
                .unwrap();
                window.into_canvas().build().unwrap()
            },
        }
    }
    pub fn draw(&mut self, x: [[bool; 64]; 32]) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        let mut rect = sdl2::rect::Rect::new(0, 0, 10, 10);
        for i in 0..32 as usize {
            for j in 0..64 as usize {
                rect.x = (j * 10) as i32;
                rect.y = (i * 10) as i32;
                self.canvas.set_draw_color(match x[i][j] {
                    true => pixels::Color::RGB(255, 255, 255),
                    false => pixels::Color::RGB(0, 0, 0),
                });
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.canvas.present();
    }
}
