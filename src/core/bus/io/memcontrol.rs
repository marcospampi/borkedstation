use std::cell::Cell;

use crate::core::bus::mmio;

use mmio::U8U16U32 as UU;
#[derive(Default)]
pub struct MemControl {
    exp1_base: Cell<u32>,
    exp2_base: Cell<u32>,
    exp1_size: Cell<u32>,
    exp3_size: Cell<u32>,
    bios_rom: Cell<u32>,
    spu_delay: Cell<u32>,
    cdrom_delay: Cell<u32>,
    exp2_size: Cell<u32>,
    com_delay: Cell<u32>,
    ram_size: Cell<u32>,
    cache_control: Cell<u32>
}

/*
1F801000h 4    Expansion 1 Base Address (usually 1F000000h)
1F801004h 4    Expansion 2 Base Address (usually 1F802000h)
1F801008h 4    Expansion 1 Delay/Size (usually 0013243Fh; 512Kbytes 8bit-bus)
1F80100Ch 4    Expansion 3 Delay/Size (usually 00003022h; 1 byte)
1F801010h 4    BIOS ROM    Delay/Size (usually 0013243Fh; 512Kbytes 8bit-bus)
1F801014h 4    SPU_DELAY   Delay/Size (usually 200931E1h)
1F801018h 4    CDROM_DELAY Delay/Size (usually 00020843h or 00020943h)
1F80101Ch 4    Expansion 2 Delay/Size (usually 00070777h; 128-bytes 8bit-bus)
1F801020h 4    COM_DELAY / COMMON_DELAY (00031125h or 0000132Ch or 00001325h)
1F801060h 4/2  RAM_SIZE (usually 00000B88h; 2MB RAM mirrored in first 8MB)
FFFE0130h 4        Cache Control
*/
impl mmio::Mmio for MemControl {
    fn interpreter(
        &self,
        cmd: mmio::MMIOCommand,
    ) -> crate::core::bus::Result<Option<mmio::U8U16U32>> {
        match cmd {
            mmio::MMIOCommand::ReadU32(0x1F801000) => Ok(Some(UU::U32(self.exp1_base.get()))),
            mmio::MMIOCommand::WriteU32(0x1F801000, val) => {
                self.exp1_base.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F801004) => Ok(Some(UU::U32(self.exp2_base.get()))),
            mmio::MMIOCommand::WriteU32(0x1F801004, val) => {
                self.exp2_base.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F801008) => Ok(Some(UU::U32(self.exp1_size.get()))),
            mmio::MMIOCommand::WriteU32(0x1F801008, val) => {
                self.exp1_size.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F80100C) => Ok(Some(UU::U32(self.exp3_size.get()))),
            mmio::MMIOCommand::WriteU32(0x1F80100C, val) => {
                self.exp3_size.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F801010) => Ok(Some(UU::U32(self.bios_rom.get()))),
            mmio::MMIOCommand::WriteU32(0x1F801010, val) => {
                self.bios_rom.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F801014) => Ok(Some(UU::U32(self.spu_delay.get()))),
            mmio::MMIOCommand::WriteU32(0x1F801014, val) => {
                self.spu_delay.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F801018) => Ok(Some(UU::U32(self.cdrom_delay.get()))),
            mmio::MMIOCommand::WriteU32(0x1F801018, val) => {
                self.cdrom_delay.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F80101C) => Ok(Some(UU::U32(self.exp2_size.get()))),
            mmio::MMIOCommand::WriteU32(0x1F80101C, val) => {
                self.exp2_size.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F801020) => Ok(Some(UU::U32(self.com_delay.get()))),
            mmio::MMIOCommand::WriteU32(0x1F801020, val) => {
                self.com_delay.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1F801060) => Ok(Some(UU::U32(self.ram_size.get()))),
            mmio::MMIOCommand::WriteU32(0x1F801060, val) => {
                self.ram_size.set(val);
                Ok(None)
            },
            mmio::MMIOCommand::ReadU32(0x1ffe0130) => Ok(Some(UU::U32(self.cache_control.get()))),
            mmio::MMIOCommand::WriteU32(0x1ffe0130, val) => {
                self.cache_control.set(val);
                Ok(None)
            },
            _ => Err(crate::core::bus::BusError::BadAddress)
        }
    }
}
