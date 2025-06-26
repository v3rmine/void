use nom::Parser;
use opcode::OpCode;

use super::{EmulatorReset, EmulatorTick};

mod opcode;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGISTER_COUNT: usize = 16;
const KEY_COUNT: usize = 16;

const START_ADDRESS: u16 = 0x200;

pub struct Chip8Emulator {
    program_counter: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_registers: [u8; REGISTER_COUNT],
    i_register: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; KEY_COUNT],
    delay_timer: u8,
    sound_timer: u8,
}

impl Default for Chip8Emulator {
    fn default() -> Self {
        let mut emulator = Self {
            program_counter: START_ADDRESS,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_registers: [0; REGISTER_COUNT],
            i_register: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            keys: [false; KEY_COUNT],
            delay_timer: 0,
            sound_timer: 0,
        };

        emulator.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        emulator
    }
}

impl Chip8Emulator {
    fn push(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1;
    }
    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        return self.stack[self.stack_pointer as usize];
    }

    fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO BEEP
            }
            self.sound_timer -= 1;
        }
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.program_counter as usize;
        let next_op = |s| {
            nom::number::be_u16::<_, (_, nom::error::ErrorKind)>()
                .parse(s)
                .map(|(_left, res)| res)
        };
        let op = next_op(&self.ram[pc..(pc + 2)]).unwrap();

        // we fetched 2 bytes
        self.program_counter += 2;

        op
    }

    fn execute(&mut self, raw_op: u16) {
        let digit1 = (raw_op & 0xF000) >> 12;
        let digit2 = (raw_op & 0x0F00) >> 8;
        let digit3 = (raw_op & 0x00F0) >> 4;
        let digit4 = raw_op & 0x000F;
        let op = OpCode::from((digit1, digit2, digit3, digit4));

        match op {
            OpCode::NOP(..) => return,
            OpCode::CLS(..) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            OpCode::RET(..) => {
                let return_address = self.pop();
                self.program_counter = return_address;
            }
            _ => todo!(),
        }
    }
}

impl EmulatorTick for Chip8Emulator {
    fn tick_cpu(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    fn tick_frame(&mut self) {
        self.tick_timers();
    }
}

impl EmulatorReset for Chip8Emulator {
    fn reset(&mut self) {
        *self = Chip8Emulator::default();
    }
}
