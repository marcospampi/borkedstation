pub mod memory;
pub mod mmio;
use mmio::U8U16U32;
pub trait Unit: Sized + Into<u32> + Copy + 'static {
    fn from_u8u16u32(from: U8U16U32) -> Self;
}
impl Unit for u32 {
    fn from_u8u16u32(from: U8U16U32) -> Self {
        match from {
            U8U16U32::U32(val) => val,
            _ => panic!("Mismateched data type")
        }
    }
}
impl Unit for u16 {
    fn from_u8u16u32(from: U8U16U32) -> Self {
        match from {
            U8U16U32::U16(val) => val,
            _ => panic!("Mismateched data type")
        }
    }
}
impl Unit for u8 {
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

