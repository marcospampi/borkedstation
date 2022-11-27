pub mod memory;
pub mod mmio;
pub mod io;
use mmio::U8U16U32;
pub trait Unit: Sized + Into<u32> + Copy + Default + std::fmt::Display + 'static {
    const SIZE: u32 = std::mem::size_of::<Self>() as _;
    fn from_u8u16u32(from: U8U16U32) -> Self;
}
impl Unit for u32 {
    const SIZE: u32 = std::mem::size_of::<u32>() as _;

    fn from_u8u16u32(from: U8U16U32) -> Self {
        match from {
            U8U16U32::U32(val) => val,
            _ => panic!("Mismateched data type")
        }
    }
}
impl Unit for u16 {
    const SIZE: u32 = std::mem::size_of::<u16>() as _;
    fn from_u8u16u32(from: U8U16U32) -> Self {
        match from {
            U8U16U32::U16(val) => val,
            _ => panic!("Mismateched data type")
        }
    }
}
impl Unit for u8 {
    const SIZE: u32 = std::mem::size_of::<u8>() as _;

    fn from_u8u16u32(from: U8U16U32) -> Self {
        match from {
            U8U16U32::U8(val) => val,
            _ => panic!("Mismateched data type")
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub enum BusError {
    BadAddress,
    CannotWrite,
    CannotRead
}
pub type Result<T> = core::result::Result<T, BusError>;

pub trait BusDevice {
    fn read<U: Unit>(&self, addr: u32 ) -> Result<U> {
        Err(BusError::CannotRead)
    }
    fn write<U: Unit>(&self, addr: u32, val: U ) -> Result<()> {
        Err(BusError::CannotWrite)
    }
    fn size(&self) -> Option<usize> { None }
}



#[derive(Default)]
pub struct DummyDevice;
impl BusDevice for DummyDevice {
    fn read<U: Unit>(&self, addr: u32 ) -> Result<U> {
        println!("Dummy device read at {:}", addr);
        Ok(U::default())
    }
    fn write<U: Unit>(&self, addr: u32, val: U ) -> Result<()> {
        println!("Dummy device write at {:}, value is {}", addr, val);

        Ok(())
    }
    fn size(&self) -> Option<usize> { None }
}
