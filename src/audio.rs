use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn make_device(_ctx: &sdl2::Sdl) -> AudioDevice<SquareWave> {
    let audio_subsystem = _ctx.audio().unwrap();
    let device = audio_subsystem
        .open_playback(
            None,
            &AudioSpecDesired {
                freq: Some(44100),
                channels: Some(1),
                samples: None,
            },
            |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: (100.0 as f32 * 10.0) / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.001,
                }
            },
        )
        .unwrap();
    device.pause();
    return device;
}
