use std::{fs::File, io::Read, collections::btree_map::Range, cell::Cell};
use std::io::Result;

use crate::cpu::Inst;

use super::{DeviceBusInterface, MASK_ADDRESS_SPACE};
pub struct Bios {
    data: Vec<u8>,
    cache: Vec<Cell::<Inst>>,
}
pub const BIOS_SIZE: u64 = 512 * 1024;
/// Each MIPS instruction is one word (4byte), so there'd be 512KiB/4byte instructions.  
/// 
/// It will take up twice tho.
pub const BIOS_CACHE_SIZE: usize = BIOS_SIZE as usize / 4; 

pub const BIOS_ADDRESS_MASK: u32 = 0x7FFFF;
pub const BIOS_IO_RANGE: std::ops::Range<u32> = 0x1fc00000..0x1fc80000;

impl Bios {
    pub fn new(filepath: &str) -> Result<Bios> {
        let file = File::open(filepath)?;

        let mut data = Vec::new();

        file.take(BIOS_SIZE).read_to_end(&mut data)?;

        let cache = std::iter::repeat(Cell::new(Inst::Unknown))
                        .take(BIOS_CACHE_SIZE)
                        .collect();

        Ok(Bios{ data, cache })
    }

    pub fn empty() -> Bios {
        let data = std::iter::repeat(0u8)
                        .take(BIOS_SIZE as usize)
                        .collect();
        let cache = std::iter::repeat(Cell::new(Inst::Unknown))
                        .take(BIOS_CACHE_SIZE)
                        .collect();
        
    Bios{ data, cache }

    }

    unsafe fn as_ptr<T>(&self) -> *const T {
        return std::mem::transmute(self.data.as_ptr());
    }
    
    fn read<T>(&self, address: u32) -> T {
        unsafe {
            self.as_ptr::<T>()
                .byte_offset((address as _) )
                .read()
        }
    }

    fn fetch(&self, address: u32) -> Inst {
        // cached cell
        let cached = &self.cache[(address << 2) as usize];

        let mut inst = cached.get();

        // if it is unknown, it's not initialized
        if matches!( inst, Inst::Unknown ) {
            inst = Inst::from(self.read::<u32>(address));
            cached.set(inst);
        };

        return inst;
        
    }

}

impl DeviceBusInterface for Bios {
    fn read32(&self, address: u32) -> u32 {
        self.read::<u32>(address)
    }

    fn read16(&self, address: u32) -> u16 {
        self.read::<u16>(address)
    }

    fn read8(&self, address: u32) -> u8 {
        self.read::<u8>(address)
    }

    fn write32(&self, _address: u32, _value: u32) { todo!("Throwind exception at this BS.") }

    fn write16(&self, _address: u32, _value: u16) { todo!("Throwind exception at this BS.") }

    fn write8(&self, _address: u32, _value: u8) { todo!("Throwind exception at this BS.") }

    fn in_range(&self, address: u32) -> bool {
        BIOS_IO_RANGE.contains(&(address & MASK_ADDRESS_SPACE))
    }
}