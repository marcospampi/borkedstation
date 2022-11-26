pub mod cop0;
pub mod gte;
pub mod mips;
pub trait Coprocessor {
    fn read(&self, reg: u8 ) -> u32;
    fn write(&self, reg: u8, val: u32);
    fn command(&self, command: u32 );
}

pub struct DummyCoprocessor;

impl Coprocessor for DummyCoprocessor {
    fn read(&self, reg: u8 ) -> u32 {
        panic!("Dummy coprocessor access")
    }

    fn write(&self, reg: u8, val: u32) {
        panic!("Dummy coprocessor access")
    }

    fn command(&self, command: u32 ) {
        panic!("Dummy coprocessor access")
    }
}