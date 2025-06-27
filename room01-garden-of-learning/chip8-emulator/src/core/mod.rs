pub mod chip8;

pub trait EmulatorTick {
    fn tick_cpu(&mut self);
    fn tick_frame(&mut self);
}

pub trait EmulatorReset {
    fn reset(&mut self);
}

pub trait EmulatorIO {
    fn get_display(&self) -> &[bool];
    fn keypress(&mut self, idx: usize, pressed: bool);
    fn load(&mut self, data: &[u8]);
}

pub trait Emulator: EmulatorIO + EmulatorReset + EmulatorTick {}
