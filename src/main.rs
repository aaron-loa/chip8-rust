use std::env;
mod cpu;
use cpu::Processor;
mod display_driver;
use sdl2::{self, event::Event, keyboard::Keycode};

fn main() {
    let mut path = r"C:\Users\petri\Desktop\pythonok\rust\chip\src\".to_string();
    let args: Vec<String> = env::args().collect();
    let ctx = sdl2::init().unwrap();
    let mut display_driver = display_driver::Display::new(&ctx);
    let mut my_game = Processor::new(ctx.event_pump().unwrap());

    path.push_str(&args[1]);
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
            match key {
                Keycode::Num1 => my_game.set_key(0x1),
                Keycode::Num2 => my_game.set_key(0x2),
                Keycode::Num3 => my_game.set_key(0x3),
                Keycode::Num4 => my_game.set_key(0xc),
                Keycode::Q => my_game.set_key(0x4),
                Keycode::W => my_game.set_key(0x5),
                Keycode::E => my_game.set_key(0x6),
                Keycode::R => my_game.set_key(0xd),
                Keycode::A => my_game.set_key(0x7),
                Keycode::S => my_game.set_key(0x8),
                Keycode::D => my_game.set_key(0x9),
                Keycode::F => my_game.set_key(0xe),
                Keycode::Y => my_game.set_key(0xa),
                Keycode::X => my_game.set_key(0x0),
                Keycode::C => my_game.set_key(0xb),
                Keycode::V => my_game.set_key(0xf),
                Keycode::O => my_game.dec_sleep_dur(),
                Keycode::P => my_game.inc_sleep_dur(),
                _ => (),
            }
        }
        my_game.tick(); // sound not implemented yet
        my_game.fetch_op();
        if my_game.vram_changed {
            display_driver.draw(my_game.vram);
        }
        std::thread::sleep(my_game.sleep_duration);
    }
}
