#[crabtime::function]
fn gen_opcode(pattern!([$(($component_name:expr, $component_op:tt)),*$(,)?]): _) {
    let ops: Vec<(String, String)> = expand!(
        [$((
            crabtime::stringify_if_needed!($component_name).to_string(),
            crabtime::stringify_if_needed!($component_op).to_string(),
        )),*]
    )
    .into_iter()
    .collect();
    let opcodes_enum = ops
        .iter()
        .map(|(name, _opcode)| format!("{name}(u16, u16, u16, u16)"))
        .collect::<Vec<_>>()
        .join(",");
    let opcodes_match_from = ops
        .iter()
        .map(|(name, opcode)| format!("op @ {opcode} => OpCode::{name}(op.0, op.1, op.2, op.3)"))
        .collect::<Vec<_>>()
        .join(",");

    crabtime::output! {
        #[derive(Debug)]
        pub enum OpCode {
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
    }
}

// https://aquova.net/emudev/chip8/8-opcodes.html
gen_opcode!([
    (NOP, (0, 0, 0, 0)),
    (CLS, (0, 0, 0xE, 0)),
    (RET, (0, 0, 0xE, 0xE)),
    (JMP_NNN, (1, _, _, _)),
    (CALL_NNN, (2, _, _, _)),
    (SKIP_VX_EQ_NN, (3, _, _, _)),
    (SKIP_VX_NEQ_NN, (4, _, _, _)),
    (SKIP_VX_EQ_VY, (5, _, _, 0)),
    (SKIP_VX_NEQ_VY, (9, _, _, 0)),
    (SET_VX_NN, (6, _, _, _)),
    (ADD_VX_NN, (7, _, _, _)),
    (SET_VX_VY, (8, _, _, 0)),
    (OR_VX_VY, (8, _, _, 1)),
    (AND_VX_VY, (8, _, _, 2)),
    (XOR_VX_VY, (8, _, _, 3)),
    (ADD_VX_VY, (8, _, _, 4)),
    (SUB_VX_VY, (8, _, _, 5)),
    (RSUB_VX_VY, (8, _, _, 7)),
    (RSHIFT_VX, (8, _, _, 6)),
    (LSHIFT_VX, (8, _, _, 8)),
    (SET_I_NNN, (0xA, _, _, _)),
    (JMP_V0_NNN, (0xB, _, _, _)),
    (RAND_VX_NN, (0xC, _, _, _)),
    (DRAW, (0xD, _, _, _)),
    (SKIP_KEY, (0xE, _, 9, 0xE)),
    (SKIP_NKEY, (0xE, _, 0xA, 1)),
    (SET_VX_DT, (0xF, _, 0, 7)),
    (WAIT_KEY, (0xF, _, 0, 0xA)),
    (SET_DT_VX, (0xF, _, 1, 5)),
    (SET_ST_VX, (0xF, _, 1, 8)),
    (SET_I_VX, (0xF, _, 1, 0xE)),
    (SET_I_FONT, (0xF, _, 2, 9)),
    (BCD_VX, (0xF, _, 3, 3)),
    (STORE_V0_VX, (0xF, _, 5, 5)),
    (LOAD_V0_VX, (0xF, _, 6, 5))
]);
