use crate::machine::Machine;
use super::Cpu;

pub trait Coprocessor {
    fn execute(&mut self, machine: &Machine, command: u32);
    fn read(&self, register: u8) -> u32;
    fn write(&mut self,  register: u8, value: u32);
}