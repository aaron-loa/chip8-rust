use std::env;
mod cpu;
use cpu::Processor;
mod display_driver;
use sdl2::{self, event::Event, keyboard::Keycode};
mod audio;

fn main() {
    let mut path = r"./roms/".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        path.push_str(&args[1]);
    } else {
        path.push_str("pong.ch8")
    }

    let ctx = sdl2::init().unwrap();
    let mut display_driver = display_driver::Display::new(&ctx);
    let device = audio::make_device(&ctx);
    let mut my_game = Processor::new(ctx.event_pump().unwrap(), device);

    let file = std::fs::read(path);
    for (i, val) in file.unwrap().into_iter().enumerate() {
        my_game.memory[0x200 + i] = val;
    }

    'outer: loop {
        for event in my_game.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'outer;
            };
            if let Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'outer;
            };
        }
        my_game.vram_changed = false;

        //get input like this to avoid borrow errors
        let keys: Vec<Keycode> = my_game
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for key in keys {
            if key == Keycode::Num1 {
                my_game.set_key(0x1);
            }
            if key == Keycode::Num2 {
                my_game.set_key(0x2)
            }
            if key == Keycode::Num3 {
                my_game.set_key(0x3)
            }
            if key == Keycode::Num4 {
                my_game.set_key(0xc)
            }
            if key == Keycode::Q {
                my_game.set_key(0x4)
            }
            if key == Keycode::W {
                my_game.set_key(0x5)
            }
            if key == Keycode::E {
                my_game.set_key(0x6)
            }
            if key == Keycode::R {
                my_game.set_key(0xd)
            }
            if key == Keycode::A {
                my_game.set_key(0x7)
            }
            if key == Keycode::S {
                my_game.set_key(0x8)
            }
            if key == Keycode::D {
                my_game.set_key(0x9)
            }
            if key == Keycode::F {
                my_game.set_key(0xe)
            }
            if key == Keycode::Y {
                my_game.set_key(0xa)
            }
            if key == Keycode::X {
                my_game.set_key(0x0)
            }
            if key == Keycode::C {
                my_game.set_key(0xb)
            }
            if key == Keycode::V {
                my_game.set_key(0xf)
            }
            if key == Keycode::O {
                my_game.dec_sleep_dur()
            }
            if key == Keycode::P {
                my_game.inc_sleep_dur()
            }
        }
        my_game.tick();
        my_game.fetch_op();
        my_game.keypad = [false; 16];
        if my_game.vram_changed {
            display_driver.draw(my_game.vram);
        }
        std::thread::sleep(my_game.sleep_duration);
    }
}
