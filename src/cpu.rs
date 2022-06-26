use rand::random;
use sdl2::EventPump;
use std::time;
use std::time::Duration;
#[path = "font.rs"]
mod font;
use font::FONT_SET;

pub struct Instruction {
    nibble_1: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16,
}

impl Instruction {
    pub fn new(one: u8, two: u8) -> Instruction {
        Instruction {
            nibble_1: (one & 0xF0) >> 4,
            x: one & 0x0F,
            y: (two & 0xF0) >> 4,
            n: two & 0x0F,
            nn: two,
            nnn: ((one as u16 & 0x0F) << 8) | two as u16,
        }
    }

    pub fn run(&mut self, processor: &mut Processor) -> () {
        //println!(
        //    "{:#03x} {:#03x} {:#03x} {:#03x}",
        //    self.nibble_1, self.x, self.y, self.n
        //);
        match (self.nibble_1, self.x, self.y, self.n) {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(processor),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(processor),
            (0x01, _, _, _) => self.op_1nnn(processor),
            (0x02, _, _, _) => self.op_2nnn(processor),
            (0x03, _, _, _) => self.op_3xnn(processor),
            (0x04, _, _, _) => self.op_4xnn(processor),
            (0x05, _, _, 0x00) => self.op_5xy0(processor),
            (0x06, _, _, _) => self.op_6xnn(processor),
            (0x07, _, _, _) => self.op_7xnn(processor),
            (0x08, _, _, 0x00) => self.op_8xy0(processor),
            (0x08, _, _, 0x01) => self.op_8xy1(processor),
            (0x08, _, _, 0x02) => self.op_8xy2(processor),
            (0x08, _, _, 0x03) => self.op_8xy3(processor),
            (0x08, _, _, 0x04) => self.op_8xy4(processor),
            (0x08, _, _, 0x05) => self.op_8xy5(processor),
            (0x08, _, _, 0x06) => self.op_8xy6(processor),
            (0x08, _, _, 0x07) => self.op_8xy7(processor),
            (0x08, _, _, 0x0E) => self.op_8xye(processor),
            (0x09, _, _, 0x00) => self.op_9xy0(processor),
            (0x0A, _, _, _) => self.op_annn(processor),
            (0x0B, _, _, _) => self.op_bnnn(processor),
            (0x0C, _, _, _) => self.op_cxnn(processor),
            (0x0D, _, _, _) => self.op_dxyn(processor),
            (0x0E, _, 0x09, 0x0E) => self.op_ex9e(processor),
            (0x0E, _, 0x0A, 0x01) => self.op_exa1(processor),
            (0x0F, _, 0x00, 0x07) => self.op_fx07(processor),
            (0x0F, _, 0x00, 0x0a) => self.op_fx0a(processor),
            (0x0F, _, 0x01, 0x05) => self.op_fx15(processor),
            (0x0F, _, 0x01, 0x08) => self.op_fx18(processor),
            (0x0F, _, 0x01, 0x0e) => self.op_fx1e(processor),
            (0x0F, _, 0x02, 0x09) => self.op_fx29(processor),
            (0x0F, _, 0x03, 0x03) => self.op_fx33(processor),
            (0x0F, _, 0x05, 0x05) => self.op_fx55(processor),
            (0x0F, _, 0x06, 0x05) => self.op_fx65(processor),
            _ => println!("Doesn't exist, or not implemented!"),
        }
    }
    //load in memory
    pub fn op_fx65(&self, processor: &mut Processor) {
        //if self.x == 0 {
        //    processor.v[self.x as usize] = processor.memory[processor.i];
        //    processor.i += 1;
        //    return;
        //}
        for val in 0..(self.x + 1) as usize {
            processor.v[val] = processor.memory[(processor.i + val) as usize];
        }
    }

    //store in memory
    pub fn op_fx55(&self, processor: &mut Processor) {
        //if self.x == 0 {
        //    processor.memory[processor.i as usize] = processor.v[self.x as usize] as u8;
        //    processor.i += 1;
        //    return;
        //}
        for val in 0..(self.x + 1) as usize {
            processor.memory[processor.i + val] = processor.v[val] as u8;
        }
    }
    //binary to decimal
    pub fn op_fx33(&self, processor: &mut Processor) {
        let num = processor.v[self.x as usize];
        let (n1, n2, n3) = (num / 100, (num % 100) / 10, num % 10);
        processor.memory[processor.i as usize] = n1 as u8;
        processor.memory[(processor.i + 1) as usize] = n2 as u8;
        processor.memory[(processor.i + 2) as usize] = n3 as u8;
    }

    //set i to font adress
    pub fn op_fx29(&self, processor: &mut Processor) {
        processor.i = ((processor.v[self.x as usize]) * 5 + 0x050) as usize;
    }
    // wait for input
    pub fn op_fx0a(&self, processor: &mut Processor) {
        for (i, val) in processor.keypad.into_iter().enumerate() {
            if val == true {
                processor.v[self.x as usize] = i as u8;
                return;
            }
        }
        processor.pc -= 2;
    }

    //add to index set v[0xf] to 1 if overflow
    pub fn op_fx1e(&self, processor: &mut Processor) {
        let res = processor.i + processor.v[self.x as usize] as usize;
        processor.v[0x0f] = if processor.i > 0x0F00 { 1 } else { 0 };
        processor.i = res;
    }
    // set delay timer to v[x]
    pub fn op_fx18(&self, processor: &mut Processor) {
        processor.sound_timer = processor.v[self.x as usize];
    }
    // set delay timer to v[x]
    pub fn op_fx15(&self, processor: &mut Processor) {
        processor.delay_timer = processor.v[self.x as usize];
    }
    // set v[x] to delay timer
    pub fn op_fx07(&self, processor: &mut Processor) {
        processor.v[self.x as usize] = processor.delay_timer;
    }
    // skip if v[x] is not pressed
    pub fn op_exa1(&self, processor: &mut Processor) {
        if processor.keypad[processor.v[self.x as usize] as usize] != true {
            processor.pc += 2;
        }
    }
    // skip if v[x] value is pressed
    pub fn op_ex9e(&self, processor: &mut Processor) {
        if processor.keypad[processor.v[self.x as usize] as usize] == true {
            processor.pc += 2;
        }
    }
    //rng
    pub fn op_cxnn(&self, processor: &mut Processor) {
        processor.v[self.x as usize] = random::<u8>() & self.nn as u8;
    }

    //weird jump
    pub fn op_bnnn(&self, processor: &mut Processor) {
        processor.pc = (self.nnn + processor.v[0] as u16) as usize;
    }

    //shr
    pub fn op_8xy6(&self, processor: &mut Processor) {
        processor.v[0x0f] = 0x01 & processor.v[self.x as usize];
        processor.v[self.x as usize] >>= 1;
    }
    //shl
    pub fn op_8xye(&self, processor: &mut Processor) {
        processor.v[0x0f] = if 0x80 & processor.v[self.x as usize] > 0 {
            1
        } else {
            0
        };
        processor.v[self.x as usize] <<= 1;
    }
    // subtract
    pub fn op_8xy7(&self, processor: &mut Processor) {
        let res = processor.v[(self.y) as usize].wrapping_sub(processor.v[self.x as usize]) as u8;
        if processor.v[self.y as usize] >= processor.v[self.x as usize] {
            processor.v[0xF] = 1;
        } else {
            processor.v[0xF] = 0;
        }
        processor.v[self.x as usize] = res;
    }

    pub fn op_8xy5(&self, processor: &mut Processor) {
        let res = processor.v[(self.x) as usize].wrapping_sub(processor.v[self.y as usize]) as u8;
        if processor.v[self.x as usize] >= processor.v[self.y as usize] {
            processor.v[0xF] = 1;
        } else {
            processor.v[0xF] = 0;
        }
        processor.v[self.x as usize] = res;
    }

    // add
    pub fn op_8xy4(&self, processor: &mut Processor) {
        let res = processor.v[(self.x) as usize] as u16 + processor.v[(self.y) as usize] as u16;
        if res > 255 {
            processor.v[0xF] = 1;
            processor.v[self.x as usize] = (res % 256) as u8;
        } else {
            processor.v[0xF] = 0;
            processor.v[self.x as usize] = (res % 256) as u8;
        }
    }
    // binary xor
    pub fn op_8xy3(&self, processor: &mut Processor) {
        processor.v[(self.x) as usize] =
            processor.v[(self.y) as usize] ^ processor.v[self.x as usize];
    }
    // binary and
    pub fn op_8xy2(&self, processor: &mut Processor) {
        processor.v[(self.x) as usize] =
            processor.v[(self.y) as usize] & processor.v[self.x as usize];
    }

    // binary or
    pub fn op_8xy1(&self, processor: &mut Processor) {
        processor.v[(self.x) as usize] =
            processor.v[(self.y) as usize] | processor.v[self.x as usize];
    }

    // set vx to vy
    pub fn op_8xy0(&self, processor: &mut Processor) {
        processor.v[(self.x) as usize] = processor.v[(self.y) as usize];
    }

    //skips
    pub fn op_9xy0(&self, processor: &mut Processor) {
        if processor.v[self.x as usize] != processor.v[self.y as usize] {
            processor.pc += 2;
        }
    }

    pub fn op_5xy0(&self, processor: &mut Processor) {
        if processor.v[self.x as usize] == processor.v[self.y as usize] {
            processor.pc += 2;
        }
    }

    pub fn op_3xnn(&self, processor: &mut Processor) {
        if processor.v[self.x as usize] == self.nn as u8 {
            processor.pc += 2;
        }
    }

    pub fn op_4xnn(&self, processor: &mut Processor) {
        if processor.v[self.x as usize] != self.nn as u8 {
            processor.pc += 2;
        }
    }

    // return from subroutine
    pub fn op_00ee(&self, processor: &mut Processor) {
        processor.pc = match processor.stack.pop() {
            Some(x) => x,
            None => 0x200,
        };
    }

    //start subroutine
    pub fn op_2nnn(&self, processor: &mut Processor) {
        processor.stack.push(processor.pc);
        processor.pc = self.nnn as usize;
    }

    //clear screen 00E0
    pub fn op_00e0(&self, processor: &mut Processor) {
        processor.vram = [[false; 64]; 32];
        processor.vram_changed = true;
    }

    //jump
    pub fn op_1nnn(&self, processor: &mut Processor) {
        processor.pc = self.nnn as usize;
    }
    //set v[x] to nn
    pub fn op_6xnn(&self, processor: &mut Processor) {
        processor.v[(self.x) as usize] = self.nn;
    }

    //add nn to v[x]
    pub fn op_7xnn(&self, processor: &mut Processor) {
        processor.v[(self.x) as usize] += self.nn;
    }
    // set i reg to nnn
    pub fn op_annn(&self, processor: &mut Processor) {
        processor.i = self.nnn as usize;
    }

    /// display / draw
    pub fn op_dxyn(&mut self, processor: &mut Processor) {
        processor.v[0x0f] = 0;
        processor.vram_changed = true;
        for i in 0..self.n {
            let y = ((processor.v[self.y as usize] + i) % 32) as usize;
            let sprite = processor.memory[(processor.i + i as usize)];
            for j in 0..8 {
                let x = ((processor.v[self.x as usize] + j) % 64) as usize;
                if x > 63 {
                    break;
                }
                if (sprite & (0x80 >> j)) > 0 {
                    if processor.vram[y][x] == true {
                        processor.v[0x0f] = 1;
                        processor.vram[y][x] = false;
                    } else {
                        processor.vram[y][x] = true;
                    }
                }
            }
            if y > 32 {
                break;
            }
        }
    }
}

pub struct Processor {
    pub stack: Vec<usize>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: usize,
    pub memory: [u8; 4096],
    pub vram: [[bool; 64]; 32],
    pub sleep_duration: time::Duration,
    pub keypad: [bool; 16],
    pub vram_changed: bool,
    pub v: [u8; 16],
    pub i: usize,
    pub event_pump: EventPump,
    pub instructions_per_second: f64,
}

impl Processor {
    pub fn new(pump: EventPump) -> Self {
        Processor {
            stack: {
                let mut buffer: Vec<usize> = Vec::new();
                buffer.reserve(32);
                buffer
            },
            delay_timer: 0,
            sound_timer: 0,
            memory: {
                let mut buffer = [0; 4096];
                let mut counter = 0x050;
                for i in FONT_SET {
                    for j in i {
                        buffer[counter] = j;
                        counter += 1;
                    }
                }
                buffer
            },
            vram: [[false; 64]; 32],
            sleep_duration: Duration::from_secs_f64(1.0 / 700.0),
            keypad: [false; 16],
            pc: 0x200,
            vram_changed: false,
            v: [0; 16],
            i: 0,
            event_pump: pump,
            instructions_per_second: 700.0,
        }
    }

    pub fn tick(&mut self) -> () {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn decode_op(&mut self, opcode_1: u8, opcode_2: u8) {
        let mut instruction = Instruction::new(opcode_1, opcode_2);
        instruction.run(self);
    }

    pub fn fetch_op(&mut self) {
        let op_1 = self.memory[self.pc];
        let op_2 = self.memory[self.pc + 1];
        self.pc += 2;
        Processor::decode_op(self, op_1, op_2);
    }
    pub fn set_key(&mut self, x: usize) {
        self.keypad = [false; 16];
        self.keypad[x] = true;
    }

    pub fn dec_sleep_dur(&mut self) {
        if self.instructions_per_second > 50.0 {
            self.instructions_per_second -= 1.0;
            self.sleep_duration = Duration::from_secs_f64(1.0 / self.instructions_per_second);
        }
        println!("Instruction per second: {}", self.instructions_per_second);
    }
    pub fn inc_sleep_dur(&mut self) {
        if self.instructions_per_second < 1300.0 {
            self.instructions_per_second += 1.0;
            self.sleep_duration = Duration::from_secs_f64(1.0 / self.instructions_per_second);
        }
        println!("Instruction per second: {}", self.instructions_per_second);
    }
}
