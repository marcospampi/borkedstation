use std::{io::Read, vec};

use super::{BusDevice, Unit, BusError};

/// A R/W Memory device
pub struct Memory {
    data: Vec<u8>
}

impl Memory {
    pub fn new(size: u32) -> Self {
        Self {
            data: std::iter::repeat(0).take(size as _).collect()
        }
    }

    unsafe fn as_ptr<U: Unit>(&self) -> * const U {
        return  std::mem::transmute(self.data.as_ptr());
    }
    unsafe fn as_ptr_mut<U: Unit>(&self) -> * mut U {
        return  std::mem::transmute(self.data.as_ptr());
    }
}

impl From<Vec<u8>> for Memory {
    fn from(value: Vec<u8>) -> Self {
        Self {
            data: value
        }
    }
}

impl BusDevice for Memory {
    fn read<U: Unit>(&self, addr: u32 ) -> super::Result<U> {
        if ( addr < self.data.len() as _) {
            unsafe {
                Ok(*self.as_ptr::<U>().byte_offset(addr as _))
            }
        }
        else {
            Err(BusError::BadAddress)
        }
    }

    fn write<U: Unit>(&self, addr: u32, val: U ) -> super::Result<()> {
        if ( addr < self.data.len() as _) {
            unsafe {
                *self.as_ptr_mut::<U>().byte_offset(addr as _) = val;
                Ok(())
            }
        }
        else {
            Err(BusError::BadAddress)
        }
    }

    fn size(&self) -> Option<usize> {
        Some(self.data.len())
    }
}

pub struct RomMemory(Memory);

impl RomMemory {
    pub fn from_file(path: &str, size: Option<u64>) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut data = vec![];
        if let Some(size) = size {
            file.take(size).read_to_end(&mut data)?;
        }
        else {
            file.read_to_end(&mut data)?;

        }

        Ok(Self(Memory::from(data)))
    }
}

impl From<Memory> for RomMemory {
    fn from(value: Memory) -> Self {
        Self (value)
    }
}

impl BusDevice for RomMemory {
    fn read<U: Unit>(&self, addr: u32 ) -> super::Result<U> {
        self.0.read(addr)
    }
    fn size(&self) -> Option<usize> {
        self.0.size()
    }
}

pub struct RamMemory(Memory);
impl RamMemory {
    pub fn new(size: u32) -> Self {
        Self(Memory::new(size))
    }
}
impl BusDevice for RamMemory {
    fn read<U: Unit>(&self, addr: u32 ) -> super::Result<U> {
        self.0.read(addr)
    }

    fn write<U: Unit>(&self, addr: u32, val: U ) -> super::Result<()> {
        self.0.write(addr, val)
    }

    fn size(&self) -> Option<usize> { 
        self.0.size()
    }
}