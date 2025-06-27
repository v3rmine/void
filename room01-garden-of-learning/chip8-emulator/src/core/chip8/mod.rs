use nom::Parser;
use opcode::OpCode;

use super::{EmulatorIO, EmulatorReset, EmulatorTick};

use constants::*;
pub use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

mod constants;
mod opcode;

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
        self.stack[self.stack_pointer as usize]
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

    // Source for the implementation https://aquova.net/emudev/chip8/5-instr.html
    fn execute(&mut self, op: OpCode) {
        match op {
            OpCode::NOP(..) => (),
            OpCode::CLS(..) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            OpCode::RET(..) => {
                let return_address = self.pop();
                self.program_counter = return_address;
            }
            OpCode::JMP_NNN(_, n1, n2, n3) => {
                // Combine the three nibbles n1, n2, and n3 into a 12-bit address nnn.
                // n1 is shifted left by 8 bits, n2 is shifted left by 4 bits, and n3 remains as is.
                // The bitwise OR operator (|) combines these shifted values to form the final 12-bit address.
                let nnn = (n1 << 8) | (n2 << 4) | n3;
                self.program_counter = nnn;
            }
            OpCode::CALL_NNN(_, n1, n2, n3) => {
                let nnn = (n1 << 8) | (n2 << 4) | n3;
                self.push(self.program_counter);
                self.program_counter = nnn;
            }
            OpCode::SKIP_VX_EQ_NN(_, x, n1, n2) => {
                let x = x as usize;
                let nn = ((n1 << 4) | n2) as u8;
                if self.v_registers[x] == nn {
                    self.program_counter += 2;
                }
            }
            OpCode::SKIP_VX_NEQ_NN(_, x, n1, n2) => {
                let x = x as usize;
                let nn = ((n1 << 4) | n2) as u8;
                if self.v_registers[x] != nn {
                    self.program_counter += 2;
                }
            }
            OpCode::SKIP_VX_EQ_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                if self.v_registers[x] == self.v_registers[y] {
                    self.program_counter += 2;
                }
            }
            OpCode::SKIP_VX_NEQ_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                if self.v_registers[x] != self.v_registers[y] {
                    self.program_counter += 2;
                }
            }
            OpCode::SET_VX_NN(_, x, n1, n2) => {
                let x = x as usize;
                let nn = ((n1 << 4) | n2) as u8;
                self.v_registers[x] = nn;
            }
            OpCode::ADD_VX_NN(_, x, n1, n2) => {
                let x = x as usize;
                let nn = ((n1 << 4) | n2) as u8;
                self.v_registers[x] = self.v_registers[x].wrapping_add(nn);
            }
            OpCode::SET_VX_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                self.v_registers[x] = self.v_registers[y];
            }
            OpCode::OR_VX_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                self.v_registers[x] |= self.v_registers[y];
            }
            OpCode::AND_VX_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                self.v_registers[x] &= self.v_registers[y];
            }
            OpCode::XOR_VX_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                self.v_registers[x] ^= self.v_registers[y];
            }
            OpCode::ADD_VX_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                let (new_vx, carry) = self.v_registers[x].overflowing_add(self.v_registers[y]);
                self.v_registers[x] = new_vx;
                self.v_registers[0xF] = carry as u8;
            }
            OpCode::SUB_VX_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                let (new_vx, borrow) = self.v_registers[x].overflowing_sub(self.v_registers[y]);
                self.v_registers[x] = new_vx;
                self.v_registers[0xF] = !borrow as u8;
            }
            OpCode::RSUB_VX_VY(_, x, y, _) => {
                let x = x as usize;
                let y = y as usize;
                let (new_vx, borrow) = self.v_registers[y].overflowing_sub(self.v_registers[x]);
                self.v_registers[x] = new_vx;
                self.v_registers[0xF] = !borrow as u8;
            }
            OpCode::RSHIFT_VX(_, x, _, _) => {
                let x = x as usize;
                let lsb = self.v_registers[x] & 1;
                self.v_registers[x] >>= 1;
                self.v_registers[0xF] = lsb;
            }
            OpCode::LSHIFT_VX(_, x, _, _) => {
                let x = x as usize;
                let msb = (self.v_registers[x] >> 7) & 1;
                self.v_registers[x] <<= 1;
                self.v_registers[0xF] = msb;
            }
            OpCode::SET_I_NNN(_, n1, n2, n3) => {
                let nnn = (n1 << 8) | (n2 << 4) | n3;
                self.i_register = nnn;
            }
            OpCode::JMP_V0_NNN(_, n1, n2, n3) => {
                let nnn = (n1 << 8) | (n2 << 4) | n3;
                self.program_counter = (self.v_registers[0] as u16) + nnn;
            }
            OpCode::RAND_VX_NN(_, x, n1, n2) => {
                let x = x as usize;
                let nn = ((n1 << 4) | n2) as u8;
                let rng: u8 = rand::random();
                self.v_registers[x] = rng & nn;
            }
            OpCode::DRAW(_, x, y, num_rows) => {
                // Get the (x, y) coords for our sprite
                let x_coord = self.v_registers[x as usize] as u16;
                let y_coord = self.v_registers[y as usize] as u16;
                // The last digit determines how many rows high our sprite is
                let num_rows = num_rows as usize;
                // Keep track if any pixels were flipped
                let mut flipped = false;
                // Iterate over each row of our sprite
                for y_line in 0..num_rows {
                    // Determine which memory address our row's data is stored
                    let addr = self.i_register + y_line as u16;
                    let pixels = self.ram[addr as usize];
                    // Iterate over each column in our row
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // Sprites should wrap around screen, so apply modulo
                            let x = (x_coord as usize + x_line) % SCREEN_WIDTH;
                            let y = (y_coord as usize + y_line) % SCREEN_HEIGHT;
                            // Get our pixel's index for our 1D screen array
                            let idx = x + SCREEN_WIDTH * y;
                            // Check if we're about to flip the pixel and set
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }
                // Populate VF register
                self.v_registers[0xF] = flipped as u8;
            }
            OpCode::SKIP_KEY(_, x, _, _) => {
                let x = x as usize;
                let vx = self.v_registers[x] as usize;
                let key = self.keys[vx];
                if key {
                    self.program_counter += 2;
                }
            }
            OpCode::SKIP_NKEY(_, x, _, _) => {
                let x = x as usize;
                let vx = self.v_registers[x] as usize;
                let key = self.keys[vx];
                if !key {
                    self.program_counter += 2;
                }
            }
            OpCode::WAIT_KEY(_, x, _, _) => {
                let x = x as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    // Redo opcode
                    self.program_counter -= 2;
                }
            }
            OpCode::SET_VX_DT(_, x, _, _) => {
                let x = x as usize;
                self.v_registers[x] = self.delay_timer;
            }
            OpCode::SET_DT_VX(_, x, _, _) => {
                let x = x as usize;
                self.delay_timer = self.v_registers[x];
            }
            OpCode::SET_ST_VX(_, x, _, _) => {
                let x = x as usize;
                self.sound_timer = self.v_registers[x];
            }
            OpCode::ADD_I_VX(_, x, _, _) => {
                let x = x as usize;
                let vx = self.v_registers[x] as u16;
                self.i_register = self.i_register.wrapping_add(vx);
            }
            OpCode::SET_I_FONT(_, x, _, _) => {
                let x = x as usize;
                let c = self.v_registers[x] as u16;
                self.i_register = c * 5;
            }
            OpCode::BCD_VX(_, x, _, _) => {
                let x = x as usize;
                let vx = self.v_registers[x] as f32;
                // Fetch the hundreds digit by dividing by 100 and tossing the decimal
                let hundreds = (vx / 100.0).floor() as u8;
                // Fetch the tens digit by dividing by 10, tossing the ones digit and the decimal
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                // Fetch the ones digit by tossing the hundreds and the tens
                let ones = (vx % 10.0) as u8;
                self.ram[self.i_register as usize] = hundreds;
                self.ram[(self.i_register + 1) as usize] = tens;
                self.ram[(self.i_register + 2) as usize] = ones;
            }
            OpCode::STORE_V0_VX(_, x, _, _) => {
                let x = x as usize;
                let i = self.i_register as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_registers[idx];
                }
            }
            OpCode::LOAD_V0_VX(_, x, _, _) => {
                let x = x as usize;
                let i = self.i_register as usize;
                for idx in 0..=x {
                    self.v_registers[idx] = self.ram[i + idx];
                }
            }
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

impl EmulatorIO for Chip8Emulator {
    fn get_display(&self) -> &[bool] {
        &self.screen
    }

    fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    fn load(&mut self, data: &[u8]) {
        let start = START_ADDRESS as usize;
        let end = start + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
}
