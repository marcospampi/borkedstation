use std::{io::Result, ops::Add};
use crate::cpu::Inst;
pub const MASK_ADDRESS_SPACE: u32 = 0x1FFFFFFF;
pub trait DeviceBusInterface {
    fn read32(&self, address: u32) -> u32;
    fn read16(&self, address: u32) -> u16;
    fn read8(&self, address: u32) -> u8;
    fn write32(&self, address: u32, value: u32);
    fn write16(&self, address: u32, value: u16);
    fn write8(&self, address: u32, value: u8);
    fn in_range(&self, address: u32) -> bool;
}
trait Addressable: Into<u32> + Sized {}
impl Addressable for u32 {}
impl Addressable for u16 {}
impl Addressable for u8 {}



pub trait BusDevice {
    fn read<T: Addressable>(&self, address: u32) -> Result<T>;
    fn write<T: Addressable>(&self, address: u32, val: T) -> Result<()>;
}


mod bios;

pub struct Bus {
    pub bios: bios::Bios
}

impl Bus {
    pub fn new(bios_path: &str) -> Result<Bus> {
        let bios = bios::Bios::new(bios_path)?;

        Ok(Self {
            bios: bios
        })
    }

    pub fn with_empty_bios() -> Bus {
        Self{
            bios: bios::Bios::empty()
        }
    }
/*
  00000000h 80000000h A0000000h  2048K  Main RAM (first 64K reserved for BIOS)
  1F000000h 9F000000h BF000000h  8192K  Expansion Region 1 (ROM/RAM)
  1F800000h 9F800000h    --      1K     Scratchpad (D-Cache used as Fast RAM)
  1F801000h 9F801000h BF801000h  8K     I/O Ports
  1F802000h 9F802000h BF802000h  8K     Expansion Region 2 (I/O Ports)
  1FA00000h 9FA00000h BFA00000h  2048K  Expansion Region 3 (SRAM BIOS region for DTL cards)
  1FC00000h 9FC00000h BFC00000h  512K   BIOS ROM (Kernel) (4096K max)
        FFFE0000h (in KSEG2)     0.5K   Internal CPU control registers (Cache Control) */
    pub fn get_device_interface(&self, address: u32) -> Option<&dyn DeviceBusInterface> {
        let masked = address & MASK_ADDRESS_SPACE;
        match masked {
            0x00000000..0x00200000 => todo!("Main RAM (first 64K reserved for BIOS)"),
            0x1F000000..0x1F800000 => todo!("Expansion Region 1 (ROM/RAM)"),
            0x1F800000..0x1F800400 => todo!("Scratchpad (D-Cache used as Fast RAM)"),
            0x1F801000..0x1F802000 => todo!("I/O Ports"),
            0x1F802000..0x1F803000 => todo!("Expansion Region 2 (I/O Ports)"),
            0x1FA00000..0x1FC00000 => todo!("Expansion Region 3 (SRAM BIOS region for DTL cards)"),
            0x1FC00000..0x1FC80000 => Some( &self.bios ),
            _ => None
        }
    }
}

impl DeviceBusInterface for Bus {
    fn read32(&self, address: u32) -> u32 {
        self.get_device_interface(address).unwrap().read32(address)
    }

    fn read16(&self, address: u32) -> u16 {
        self.get_device_interface(address).unwrap().read16(address)
    }

    fn read8(&self, address: u32) -> u8 {
        self.get_device_interface(address).unwrap().read8(address)
    }

    fn write32(&self, address: u32, value: u32) {
        self.get_device_interface(address).unwrap().write32(address, value)
    }

    fn write16(&self, address: u32, value: u16) {
        self.get_device_interface(address).unwrap().write16(address, value)
    }

    fn write8(&self, address: u32, value: u8) {
        self.get_device_interface(address).unwrap().write8(address, value)
    }

    fn in_range(&self, address: u32) -> bool {
        match self.get_device_interface(address) {
            Some(_) => true,
            _ => false
        }
    }
}