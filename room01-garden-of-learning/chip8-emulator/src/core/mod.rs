pub mod chip8;

pub trait EmulatorTick {
    fn tick_cpu(&mut self);
    fn tick_frame(&mut self);
}

pub trait EmulatorReset {
    fn reset(&mut self);
}
