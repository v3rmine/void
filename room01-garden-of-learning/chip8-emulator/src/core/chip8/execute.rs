use super::*;

#[crabtime::function]
fn gen_opcode(
    pattern!([$(($component_name:expr, $component_op:expr, $component_exec:expr)),*$(,)?]): _,
) {
    let ops: Vec<(String, (u16, u16, u16, u16), String)> = expand!(
        [$((
            crabtime::stringify_if_needed!($component_name).to_string(),
            $component_op,
            crabtime::stringify_if_needed!($component_exec).to_string()
        )),*]
    )
    .into_iter()
    .collect();
    let opcodes_enum = ops
        .iter()
        .map(|(name, _opcode, _exec)| format!("{name}(u16, u16, u16, u16)"))
        .collect::<Vec<_>>()
        .join(",");
    let opcodes_match_from = ops
        .iter()
        .map(|(name, (op1, op2, op3, op4), _exec)| {
            format!("({op1}, {op2}, {op3}, {op4}) => OpCode::{name}({op1}, {op2}, {op3}, {op4})")
        })
        .collect::<Vec<_>>()
        .join(",");
    let opcodes_match_exec = ops
        .iter()
        .map(|(name, _opcode, exec)| format!("OpCode::{name}(_, _, _, _) => {exec}"))
        .collect::<Vec<_>>()
        .join(",");

    crabtime::output! {
        #[derive(Debug)]
        enum OpCode {
            {{opcodes_enum}}
        }

        impl From<(u16, u16, u16, u16)> for OpCode {
            fn from(from: (u16, u16, u16, u16)) -> OpCode {
                match from {
                    {{opcodes_match_from}},
                    _ => unimplemented!("Unimplemented opcode: {:?}", from),
                }
            }
        }

        impl Execute for Chip8Emulator {
            fn execute(&mut self, raw_op: u16) {
                let digit1 = (raw_op & 0xF000) >> 12;
                let digit2 = (raw_op & 0x0F00) >> 8;
                let digit3 = (raw_op & 0x00F0) >> 4;
                let digit4 = raw_op & 0x000F;

                let op = OpCode::from((digit1, digit2, digit3, digit4));
                match op {
                    {{opcodes_match_exec}},
                    _ => unimplemented!("Unimplemented opcode: {:?}", op),
                }
            }
        }
    }
}

pub trait Execute {
    fn execute(&mut self, raw_op: u16);
}

// https://aquova.net/emudev/chip8/8-opcodes.html
gen_opcode!([
    ("NOP", (0, 0, 0, 0), return),
    ("CLS", (0, 0, 0xE, 0), {
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }),
    ("RET", (0, 0, 0xE, 0xE), {
        let ret_addr = self.pop();
        self.program_counter = ret_addr;
    })
]);
