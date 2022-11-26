use std::io::Result;
use std::cell::RefCell;

use crate::{cpu::Cpu, bus::Bus};

pub struct Machine {
    pub cpu: RefCell<Cpu>,
    pub bus: Bus,
    pub cop0: Cop0

}


impl Machine {
    pub fn new(bios_path: &str) -> Result<Self> {
        let bus = Bus::new(bios_path)?;
        Ok(Self {
            cpu: Default::default(),
            bus
        })
    }

    pub fn with_bus(bus: Bus) -> Self {
        Self{
            cpu: Default::default(),
            bus
        }
    }

    pub fn run(&self) -> bool {
        let mut cpu = self.cpu.borrow_mut();
        return cpu.run(self);
    } 
}