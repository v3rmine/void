use nom::{IResult, bits};
use nom::error::Error as NomError;

use crate::abi::MemoryMutation;
use crate::{parse_bits, Armv6M};

// Source: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/POP>
#[derive(Debug)]
pub struct Pop {
    pub p: u8,
    pub register_list: u8
}

pub fn parse_pop(i: &[u8]) -> IResult<&'_ [u8], Pop> {
    bits::<_, _, NomError<(&[u8], usize)>, _, _>(
        parse_bits!(
            (0b1011110, 7u8),
            (1u8, 8u8),
            (p, register_list),
            Pop { p, register_list }
        )
    )(i)
}

impl MemoryMutation<Armv6M> for Pop {
    fn apply(&self, _on: &mut Armv6M) {
        todo!()
    }

    fn rollback(&self, _on: &mut Armv6M) {
        todo!()
    }
}
