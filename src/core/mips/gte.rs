use super::Coprocessor;

#[derive(Default)]
pub struct Gte {

}

impl Coprocessor for Gte {
    fn read(&self, reg: u8 ) -> u32 {
        todo!()
    }

    fn write(&self, reg: u8, val: u32) {
        todo!()
    }

    fn command(&self, command: u32 ) {
        todo!()
    }
}