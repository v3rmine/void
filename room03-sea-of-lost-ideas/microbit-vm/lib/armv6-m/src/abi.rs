use std::rc::Rc;

pub trait MemoryMutation<R: Runtime> {
    fn apply(&self, on: &mut R);
    fn rollback(&self, on: &mut R);
}

pub trait Runtime
where
    Self: Sized,
{
    type Error: std::error::Error;
    type Mutation: MemoryMutation<Self>;
    // type Register;

    fn init() -> Self;

    fn load_file(&mut self, path: &str) -> Result<&mut Self, Self::Error>;
    fn load_hex(&mut self, hex: &str) -> Result<&mut Self, Self::Error>;
    fn load_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, Self::Error>;

    // NOTE: Useless ?
    // fn get_register(&self, register: usize) -> Self::Register;
    // fn set_register(&mut self, register: usize, data: Self::Register) -> Result<&mut Self, Self::Error>;

    fn get_memory(&self) -> Rc<[u8]>;
    fn get_mutations_history(&self) -> Rc<[Self::Mutation]>;

    fn get_memory_at_mutation(&self, idx: usize) -> Vec<u8>;

    fn step(&mut self) -> Self;
    fn run(&mut self) -> Self;
}

pub trait RuntimeExtras
where
    Self: Runtime,
{
    fn apply_mutation(
        &mut self,
        mutation: <Self as Runtime>::Mutation,
    ) -> Result<&mut Self, <Self as Runtime>::Error>;
    fn rollback_last_mutation(&mut self) -> Result<&mut Self, <Self as Runtime>::Error>;
}
