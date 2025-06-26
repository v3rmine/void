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

#[derive(Debug)]
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

    #[tracing::instrument]
    fn fetch(&mut self) -> OpCode {
        let pc = self.program_counter as usize;
        let mut op_parser = nom::combinator::map(
            nom::number::be_u16::<_, (_, nom::error::ErrorKind)>(),
            |raw_op| {
                OpCode::from((
                    // The & operation applies a bitmask.  The >> operation is a right bit shift.
                    // 0xF000 masks the first nibble (4 bits) of the opcode.  >> 12 shifts it to the right by 12 bits, placing it in the least significant nibble.
                    (raw_op & 0xF000) >> 12,
                    // 0x0F00 masks the second nibble of the opcode. >> 8 shifts it to the right by 8 bits, placing it in the least significant nibble.
                    (raw_op & 0x0F00) >> 8,
                    // 0x00F0 masks the third nibble of the opcode. >> 4 shifts it to the right by 4 bits, placing it in the least significant nibble.
                    (raw_op & 0x00F0) >> 4,
                    // 0x000F masks the fourth nibble of the opcode. No shift is needed as it's already in the least significant nibble.
                    raw_op & 0x000F,
                ))
            },
        );
        // We only take the next 2 bytes in the RAM
        let (_left, op) = op_parser.parse(&self.ram[pc..(pc + 2)]).unwrap();

        // We fetched 2 bytes so we increase the PC
        self.program_counter += 2;

        op
    }

    #[tracing::instrument]
    fn execute(&mut self, op: OpCode) {
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
