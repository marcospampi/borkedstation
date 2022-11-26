pub mod memcontrol;
use super::{BusDevice, mmio::Mmio};


#[derive(Default)]
pub struct IOMap {
    memcontrol: memcontrol::MemControl
}

impl BusDevice for IOMap {
    fn read<U: super::Unit>(&self, addr: u32 ) -> super::Result<U> {
        match addr {
            0x1F801000..0x1F801020 => self.memcontrol.read::<U>(addr),
            0x1F801060 |0x1ffe0130 => self.memcontrol.read::<U>(addr),
            
            _ => Err( super::BusError::BadAddress )
        }
    }

    fn write<U: super::Unit>(&self, addr: u32, val: U ) -> super::Result<()> {
        match addr {
            0x1F801000..0x1F801020 => self.memcontrol.write::<U>(addr, val),
            0x1F801060 |0x1ffe0130 => self.memcontrol.write::<U>(addr, val),

            _ => Err( super::BusError::BadAddress )
        }
    }

    fn size(&self) -> Option<usize> { None }
}