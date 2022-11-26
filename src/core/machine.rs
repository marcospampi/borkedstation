use std::{ptr::NonNull, pin::Pin, vec};

use super::{mips::mips::Mips, bus::{BusDevice, BusError, memory::{RomMemory, RamMemory, Memory}}};
const RAM_SIZE: u32 = 2 * 1024 * 1024; // 2MiB
const BIOS_SIZE: u32 = 512 * 1024; // 512KiB
pub const MASK_ADDRESS_SPACE: u32 = 0x1FFFFFFF;

pub struct Machine {
    pub cpu: Mips,
    pub ram: RamMemory,
    pub rom: RomMemory,
    _marker: std::marker::PhantomPinned
}

impl Machine {
    pub fn new() -> Pin<Box<Self>> {
        let machine = Machine { 
            cpu: Mips::new(NonNull::dangling()),
            rom: RomMemory::from(Memory::new(BIOS_SIZE)),
            ram: RamMemory::new(RAM_SIZE),
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

    pub fn new_with_bios(path: &str) -> std::io::Result<Pin<Box<Self>>> {
        let rom = RomMemory::from_file(path, Some(BIOS_SIZE as _))?;
        let machine = Machine { 
            cpu: Mips::new(NonNull::dangling()),
            rom: rom,
            ram: RamMemory::new(RAM_SIZE),
            _marker: Default::default()
        };
        let mut boxed = Box::pin(machine);
        unsafe {
            let ptr = boxed.use_dumb_cheat();
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);

            Pin::get_unchecked_mut(mut_ref).cpu.machine = ptr;
            //Pin::get_unchecked_mut(mut_ref).slice = slice;
        };
        Ok(boxed)
    }

    unsafe fn use_dumb_cheat(&self) -> NonNull<Self> {
        let fucked = (self as * const Machine) as * mut Machine;
        return NonNull::new(fucked).unwrap();
    }

    pub fn run(&self) {
        self.cpu.run()
    }
}

impl BusDevice for Machine {
    fn read<U: super::bus::Unit>(&self, addr: u32 ) -> super::bus::Result<U> {
        // word alignment check
        if addr & 3 != 0 {
            return Err(BusError::BadAddress);
        }

        let addr = addr & MASK_ADDRESS_SPACE;

        match addr {
            0x00000000..0x00200000 => self.ram.read::<U>(addr & 0x1FFFFF),
            0x1F000000..0x1F800000 => Err(BusError::CannotRead),// todo!("Expansion Region 1 (ROM/RAM)"),
            0x1F800000..0x1F800400 => Err(BusError::CannotRead),// todo!("Scratchpad (D-Cache used as Fast RAM)"),
            0x1F801000..0x1F802000 => Err(BusError::CannotRead),// todo!("I/O Ports"),
            0x1F802000..0x1F803000 => Err(BusError::CannotRead),// todo!("Expansion Region 2 (I/O Ports)"),
            0x1FA00000..0x1FC00000 => Err(BusError::CannotRead),// todo!("Expansion Region 3 (SRAM BIOS region for DTL cards)"),
            0x1FC00000..0x1FC80000 => self.rom.read::<U>(addr & 0x7FFFF),
            _ => Err(BusError::BadAddress)
        }
    }

    fn write<U: super::bus::Unit>(&self, addr: u32, val: U ) -> super::bus::Result<()> {
        // word alignment check
        if addr & 3 != 0 {
            return Err(BusError::BadAddress);
        }

        let addr = addr & MASK_ADDRESS_SPACE;

        match addr {
            0x00000000..0x00200000 => self.ram.write::<U>(addr & 0x1FFFFF, val),
            0x1F000000..0x1F800000 => Err(BusError::CannotWrite),//todo!("Expansion Region 1 (ROM/RAM)"),
            0x1F800000..0x1F800400 => Err(BusError::CannotWrite),//todo!("Scratchpad (D-Cache used as Fast RAM)"),
            0x1F801000..0x1F802000 => Err(BusError::CannotWrite),//todo!("I/O Ports"),
            0x1F802000..0x1F803000 => Err(BusError::CannotWrite),//todo!("Expansion Region 2 (I/O Ports)"),
            0x1FA00000..0x1FC00000 => Err(BusError::CannotWrite),//todo!("Expansion Region 3 (SRAM BIOS region for DTL cards)"),
            0x1FC00000..0x1FC80000 => self.rom.write::<U>(addr & 0x7FFFF, val),
            _ => Err(BusError::BadAddress)
        }
    }

    fn size(&self) -> Option<usize> { None }
}