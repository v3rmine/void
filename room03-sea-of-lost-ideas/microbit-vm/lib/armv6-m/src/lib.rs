use abi::Runtime;

pub mod abi;
pub mod error;
pub mod instructions;
pub mod structure;

mod macros;

struct Armv6M {
    memory: [u8; structure::MEMORY_MAX_ADDRESSABLE_ADDRESS],

    // NOTE: Generic ARM Registers
    registers: [u32; 13],

    // Point to SP_main or to SP_process
    sp: usize,
    // Link Register
    lr: u32,
    // Program counter
    pc: u32,

    // NOTE: Specific Armv6-M SP Registers
    // SP_main or MSP
    sp_main: usize,
    // SP_process or PSP
    sp_process: usize,
    control: u32,
}

macro_rules! get_register_generator {
    ($(($i:ident, $idx:expr)),*) => {
        $(
            fn $i(&self) -> u32 {
                self.registers[$idx]
            }
        )*
    };
}

macro_rules! set_register_generator {
    ($(($i:ident, $idx:expr)),*) => {
        $(
            fn $i(&mut self, value: u32) {
                self.registers[$idx] = value;
            }
        )*
    };
}

impl Armv6M {
    get_register_generator!(
        (get_r0, 0),
        (get_r1, 1),
        (get_r2, 2),
        (get_r3, 3),
        (get_r4, 4),
        (get_r5, 5),
        (get_r6, 6),
        (get_r7, 7),
        (get_r8, 8),
        (get_r9, 9),
        (get_r10, 10),
        (get_r11, 11),
        (get_r12, 12)
    );
    set_register_generator!(
        (set_r0, 0),
        (set_r1, 1),
        (set_r2, 2),
        (set_r3, 3),
        (set_r4, 4),
        (set_r5, 5),
        (set_r6, 6),
        (set_r7, 7),
        (set_r8, 8),
        (set_r9, 9),
        (set_r10, 10),
        (set_r11, 11),
        (set_r12, 12)
    );
}

impl Runtime for Armv6M {
    type Error = error::Error;

    type Mutation = instructions::add::Add;

    fn init() -> Self {
        todo!()
    }

    fn load_file(&mut self, path: &str) -> Result<&mut Self, Self::Error> {
        todo!()
    }

    fn load_hex(&mut self, hex: &str) -> Result<&mut Self, Self::Error> {
        todo!()
    }

    fn load_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, Self::Error> {
        todo!()
    }

    fn get_memory(&self) -> std::rc::Rc<[u8]> {
        todo!()
    }

    fn get_mutations_history(&self) -> std::rc::Rc<[Self::Mutation]> {
        todo!()
    }

    fn get_memory_at_mutation(&self, idx: usize) -> Vec<u8> {
        todo!()
    }

    fn step(&mut self) -> Self {
        todo!()
    }

    fn run(&mut self) -> Self {
        todo!()
    }
}
