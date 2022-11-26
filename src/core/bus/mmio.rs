use std::any::TypeId;

use super::{BusDevice, BusError, Result, Unit};

pub enum MMIOCommand {
    ReadU8(u32),
    ReadU16(u32),
    ReadU32(u32),
    WriteU8(u32, u8),
    WriteU16(u32, u16),
    WriteU32(u32, u32),
}

pub enum U8U16U32 {
    U8(u8),
    U16(u16),
    U32(u32),
}

pub trait Mmio {
    fn interpreter(&self, cmd: MMIOCommand) -> Result<Option<U8U16U32>>;

    fn read<U: Unit>(&self, addr: u32) -> Result<U> {
        if TypeId::of::<U>() == TypeId::of::<u8>() {
            return match self.interpreter(MMIOCommand::ReadU8(addr)) {
                Ok(Some(res)) => Ok(U::from_u8u16u32(res)),
                Err(err) => Err(err),
                _ => Err(BusError::CannotRead),
            };
        } else if TypeId::of::<U>() == TypeId::of::<u16>() {
            return match self.interpreter(MMIOCommand::ReadU16(addr)) {
                Ok(Some(res)) => Ok(U::from_u8u16u32(res)),
                Err(err) => Err(err),
                _ => Err(BusError::CannotRead),
            };
        } else if TypeId::of::<U>() == TypeId::of::<u32>() {
            return match self.interpreter(MMIOCommand::ReadU32(addr)) {
                Ok(Some(res)) => Ok(U::from_u8u16u32(res)),
                Err(err) => Err(err),
                _ => Err(BusError::CannotRead),
            };
        }
        unreachable!()
    }
    fn write<U: Unit>(&self, addr: u32, val: U) -> Result<()> {
        if TypeId::of::<U>() == TypeId::of::<u8>() {
            let val = val.into() as _;
            return match self.interpreter(MMIOCommand::WriteU8(addr, val)) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            };
        } else if TypeId::of::<U>() == TypeId::of::<u16>() {
            let val = val.into() as _;

            return match self.interpreter(MMIOCommand::WriteU16(addr, val)) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            };
        } else if TypeId::of::<U>() == TypeId::of::<u32>() {
            let val = val.into() as _;

            return match self.interpreter(MMIOCommand::WriteU32(addr, val)) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            };
        }
        unreachable!()
    }
}

pub struct MmioToBusAdapter<M: Mmio>(M);
impl<M: Mmio> BusDevice for MmioToBusAdapter<M> {
    fn read<U: Unit>(&self, addr: u32 ) -> Result<U> {
        self.0.read(addr)
    }

    fn write<U: Unit>(&self, addr: u32, val: U ) -> Result<()> {
        self.0.write(addr,val)
    }

    fn size(&self) -> Option<usize> { None }
}

#[cfg(test)]
mod test {
    use std::cell::Cell;
    use crate::core::bus::{BusError, mmio::MmioToBusAdapter, BusDevice};
    use super::{MMIOCommand, Mmio};
    struct DummyMmio(Cell<u32>);

    impl super::Mmio for DummyMmio {
        fn interpreter(
            &self,
            cmd: super::MMIOCommand,
        ) -> crate::core::bus::Result<Option<super::U8U16U32>> {
            match cmd {
                MMIOCommand::ReadU32(0) => Ok(Some(super::U8U16U32::U32(self.0.get()))),
                MMIOCommand::WriteU32(0, val) => {
                    self.0.set(val);
                    Ok(None)
                },
                _ => Err(BusError::BadAddress)
            }
        }
    }
    #[test]
    fn test_mmio() {
        let dummy = MmioToBusAdapter(DummyMmio(Cell::new(0)));
        let val: u32 = 1;
        dummy.write::<u32>(0, val).expect("Error: didn't write!");
        assert_eq!(dummy.read::<u32>(0).expect("Error: Didn't read"), val);


    }
}
