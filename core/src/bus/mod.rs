use std::io::Result;
use crate::cpu::Inst;
pub const MASK_ADDRESS_SPACE: u32 = 0x1FFFFFFF;
pub trait DeviceBusInterface {
    fn read32(&self, address: u32) -> u32;
    fn read16(&self, address: u32) -> u16;
    fn read8(&self, address: u32) -> u8;
    fn write32(&self, address: u32, value: u32);
    fn write16(&self, address: u32, value: u16);
    fn write8(&self, address: u32, value: u8);

    fn try_fetch(&self, address: u32 ) -> Option<Inst>;
    fn in_range(&self, address: u32) -> bool;
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

    pub fn get_device_interface(&self, address: u32) -> Option<&dyn DeviceBusInterface> {
        let masked = address & MASK_ADDRESS_SPACE;
        match masked {
            0x1FC00000..0x1FC80000 => Some( &self.bios ),
            _ => None
        }
    }
}

impl DeviceBusInterface for Bus {
    fn read32(&self, address: u32) -> u32 {
        todo!()
    }

    fn read16(&self, address: u32) -> u16 {
        todo!()
    }

    fn read8(&self, address: u32) -> u8 {
        todo!()
    }

    fn write32(&self, address: u32, value: u32) {
        todo!()
    }

    fn write16(&self, address: u32, value: u16) {
        todo!()
    }

    fn write8(&self, address: u32, value: u8) {
        todo!()
    }

    fn try_fetch(&self, address: u32 ) -> Option<Inst> {
        let device = self.get_device_interface(address)?;
        
        return device.try_fetch(address);
    }

    fn in_range(&self, address: u32) -> bool {
        true
    }
}