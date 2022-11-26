use std::{ptr::NonNull, pin::Pin};

use super::{mips::mips::Mips, bus::BusDevice};

pub struct Machine {
    pub cpu: Mips,
    _marker: std::marker::PhantomPinned
}

impl Machine {
    pub fn new() -> Pin<Box<Self>> {
        let machine = Machine { 
            cpu: Mips::new(NonNull::dangling()), 
            _marker: Default::default()
        };
        let mut boxed = Box::pin(machine);
        unsafe {
            let ptr = boxed.use_dumb_cheat();
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);

            Pin::get_unchecked_mut(mut_ref).cpu.machine = ptr;
            //Pin::get_unchecked_mut(mut_ref).slice = slice;
        };
        boxed
    }

    unsafe fn use_dumb_cheat(&self) -> NonNull<Self> {
        let fucked = (self as * const Machine) as * mut Machine;
        return NonNull::new(fucked).unwrap();
    }
}

impl BusDevice for Machine {
    fn read<U: super::bus::Unit>(&self, addr: u32 ) -> super::bus::Result<U> {
        Err(super::bus::BusError::CannotRead)
    }

    fn write<U: super::bus::Unit>(&self, addr: u32, val: U ) -> super::bus::Result<()> {
        Err(super::bus::BusError::CannotWrite)
    }

    fn size(&self) -> Option<usize> { None }
}