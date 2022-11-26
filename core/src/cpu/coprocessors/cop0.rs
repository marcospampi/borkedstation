use std::cell::Cell;

use super::Coprocessor;

enum Exception {
    
}

struct Cop0 {
    cause: Cell<u32>,
    
}

impl Cop0 {
    fn get_reg(&self, idx: u8) -> u32 {
        match idx {
            13 => self.cause.get(),
            _ => panic!("should not happen")
        }
    }
    fn set_reg(&self, idx: u8, val: u32 ) {
        match idx {
            13 => self.cause.set(val),
            _ => panic!("should not happen")
        }
    }
}

impl Coprocessor for Cop0 {
    fn execute(&mut self, machine: &crate::Machine, command: u32) {
        todo!()
    }

    fn read(&self, idx: u8) -> u32 {
        self.get_reg(idx)
    }

    fn write(&mut self,  idx: u8, val: u32) {
        self.set_reg(idx, val)
    }
}