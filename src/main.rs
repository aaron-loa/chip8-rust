use ggez::conf::NumSamples;
use ggez::conf::*;
use ggez::event;
use ggez::ContextBuilder;
use std::env;
mod cpu;
use cpu::Processor;

mod window_conf;
use window_conf::WINDOW_MODE_CONF;

fn main() {
    let window_setup_conf: ggez::conf::WindowSetup = WindowSetup {
        /// The window title.
        title: "CHIP-8 EMULATOR".to_string(),
        /// Number of samples to use for multisample anti-aliasing.
        samples: NumSamples::One,
        /// Whether or not to enable vsync.
        vsync: false,
        /// A file path to the window's icon.
        /// It takes a path rooted in the `resources` directory (see the [`filesystem`](../filesystem/index.html)
        /// module for details), and an empty string results in a blank/default icon.
        icon: String::new(),
        /// Whether or not to enable sRGB (gamma corrected color)
        /// handling on the display.
        srgb: false,
    };
    let (ctx, event_loop) = ContextBuilder::new("my_game", "I made this work")
        .window_mode(WINDOW_MODE_CONF)
        .window_setup(window_setup_conf)
        .build()
        .expect("aieee, could not create ggez context!");
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = Processor::new();
    let mut path = r"C:\Users\petri\Desktop\pythonok\rust\chip\src\".to_string();
    let args: Vec<String> = env::args().collect();
    path.push_str(&args[1]);
    let file = std::fs::read(path);
    for (i, val) in file.unwrap().into_iter().enumerate() {
        my_game.memory[0x200 + i] = val;
    }
    // Run!
    event::run(ctx, event_loop, my_game);
}
