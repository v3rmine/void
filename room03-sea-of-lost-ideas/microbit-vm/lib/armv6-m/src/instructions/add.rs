use nom::{IResult, branch::alt, bits::bits, combinator::map, bits::streaming::{tag, take}, sequence::tuple};
use nom::error::Error as NomError;

use crate::{parse_bits, abi::MemoryMutation};

#[derive(Debug)]
pub enum Add {
    /// Immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADD--immediate->
    ImmediateT1 {
        imm3: u8,
        rn: u8,
        rd: u8,
    },
    ImmediateT2 {
        imm8: u8,
        rdn: u8,
    },
    /// Register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADD--register->
    RegisterT1 {
        rd: u8,
        rn: u8,
        rm: u8,
    },
    RegisterT2 {
        rdn: u8,
        rm: u8,
        dn: u8,
    },
    /// SP plus immediate: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADD--SP-plus-immediate->
    SpPlusImmediateT1 {
        imm8: u8,
        rd: u8,
    },
    SpPlusImmediateT2 {
        imm7: u8,
    },
    /// SP plus register: <https://developer.arm.com/documentation/ddi0419/c/Application-Level-Architecture/Thumb-Instruction-Details/Alphabetical-list-of-ARMv6-M-Thumb-instructions/ADD--SP-plus-register->
    SpPlusRegisterT1 {
        dm: u8,
        rdm: u8,
    },
    SpPlusRegisterT2 {
        rm: u8,
    },
}

pub fn parse_add(i: &[u8]) -> IResult<&'_ [u8], Add> {
    // Immediate
    let parse_immediate_t1 = parse_bits!(
        (0b0001110, 7u8),
        (3u8, 3u8, 3u8),
        (imm3, rn, rd),
        Add::ImmediateT1 { imm3, rn, rd }
    );
    let parse_immediate_t2 = parse_bits!(
        (0b00110, 5u8),
        (3u8, 8u8),
        (rdn, imm8),
        Add::ImmediateT2 { rdn, imm8 }
    );
    // Register
    let parse_register_t1 = parse_bits!(
        (0b0001100, 7u8),
        (3u8, 3u8, 3u8),
        (rm, rn, rd),
        Add::RegisterT1 { rm, rn, rd }
    );
    let parse_register_t2 = parse_bits!(
        (0b01000100, 8u8),
        (1u8, 4u8, 3u8),
        (dn, rm, rdn),
        Add::RegisterT2 { dn, rm, rdn }
    );
    // SP Plus Immediate
    let parse_sp_plus_immediate_t1 = parse_bits!(
        (0b10101, 5u8),
        (3u8, 8u8),
        (rd, imm8),
        Add::SpPlusImmediateT1 { rd, imm8 }
    );
    let parse_sp_plus_immediate_t2 = parse_bits!(
        (0b101100000, 9u16),
        7u8,
        imm7,
        Add::SpPlusImmediateT2 { imm7 }
    );
    // SP Plus Register
    let parse_sp_plus_register_t1 = map(
        tuple((
            tag(0b01000100, 8u8),
            tuple((take(1u8), tag(0b1101, 4u8), take(3u8))),
        )),
        |(_, (dm, _, rdm))| Add::SpPlusRegisterT1 { dm, rdm }
    );
    let parse_sp_plus_register_t2 = map(
        tuple((
            tag(0b010001001, 9u16),
            tuple((take(4u8), tag(0b101, 3u8))),
        )),
        |(_, (rm, _))| Add::SpPlusRegisterT2 { rm }
    );
    
    bits::<_, _, NomError<(&[u8], usize)>, _, _>(
        alt((
            parse_immediate_t1,
            parse_immediate_t2,
            parse_register_t1,
            parse_register_t2,
            parse_sp_plus_immediate_t1,
            parse_sp_plus_immediate_t2,
            parse_sp_plus_register_t1,
            parse_sp_plus_register_t2,
        ))
    )(i)
}

impl MemoryMutation<crate::Armv6M> for Add {
    fn apply(&self, _on: &mut crate::Armv6M) {
        todo!()
    }

    fn rollback(&self, _on: &mut crate::Armv6M) {
        todo!()
    }
}