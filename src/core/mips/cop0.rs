use std::cell::Cell;

use super::Coprocessor;

const PROCESSOR_ID: u32  = 0x00000002;

/*
00h INT     Interrupt
01h MOD     Tlb modification (none such in PSX)
02h TLBL    Tlb load         (none such in PSX)
03h TLBS    Tlb store        (none such in PSX)
04h AdEL    Address error, Data load or Instruction fetch
05h AdES    Address error, Data store
            The address errors occur when attempting to read
            outside of KUseg in user mode and when the address
            is misaligned. (See also: BadVaddr register)
06h IBE     Bus error on Instruction fetch
07h DBE     Bus error on Data load/store
08h Syscall Generated unconditionally by syscall instruction
09h BP      Breakpoint - break instruction
0Ah RI      Reserved instruction
0Bh CpU     Coprocessor unusable
0Ch Ov      Arithmetic overflow
0Dh-1Fh     Not used */
enum ExceptionsCodes {
    Interrupt = 0x0,
    AddressReadError = 0x4,
    AddressWriteError = 0x5,
    FetchError = 0x6,
    DataError = 0x7,
    Breakpoint = 0x09,
    ReservedInstruction = 0x0A,
    InvalidCoprocessor = 0x0B,
    ArithmeticOverflow = 0x0C,
}
/*
cop0r0-r2   - N/A
cop0r3      - BPC - Breakpoint on execute (R/W)
cop0r4      - N/A
cop0r5      - BDA - Breakpoint on data access (R/W)
cop0r6      - JUMPDEST - Randomly memorized jump address (R)
cop0r7      - DCIC - Breakpoint control (R/W)
cop0r8      - BadVaddr - Bad Virtual Address (R)
cop0r9      - BDAM - Data Access breakpoint mask (R/W)
cop0r10     - N/A
cop0r11     - BPCM - Execute breakpoint mask (R/W)
cop0r12     - SR - System status register (R/W)
cop0r13     - CAUSE - (R)  Describes the most recently recognised exception
cop0r14     - EPC - Return Address from Trap (R)
cop0r15     - PRID - Processor ID (R)
cop0r16-r31 - Garbage
cop0r32-r63 - N/A - None such (Control regs) */
#[derive(Default)]
pub struct Cop0 {
    /// cop0r3      - BPC - Breakpoint on execute (R/W)
    pub breakpoint_on_execute: Cell<u32>,
    /// cop0r5      - BDA - Breakpoint on data access (R/W)
    pub breakpoint_on_data_access: Cell<u32>,
    /// cop0r6      - JUMPDEST - Randomly memorized jump address (R)
    pub jumpdest: Cell<u32>,
    /// cop0r7      - DCIC - Breakpoint control (R/W)
    pub breakpoint_control: Cell<u32>,
    /// cop0r8      - BadVaddr - Bad Virtual Address (R)
    pub bad_virtual_address: Cell<u32>,
    /// cop0r9      - BDAM - Data Access breakpoint mask (R/W)
    pub data_access_breakpoint_mask: Cell<u32>,
    /// cop0r11     - BPCM - Execute breakpoint mask (R/W)
    pub execute_breakpoint_mask: Cell<u32>,
    /// cop0r12     - SR - System status register (R/W)
    pub system_status: Cell<u32>,
    /// cop0r13     - CAUSE - (R)  Describes the most recently recognised exception
    pub exception_cause: Cell<u32>,
    /// cop0r14     - EPC - Return Address from Trap (R)
    pub return_address_from_trap: Cell<u32>,

}

impl Coprocessor for Cop0 {
    fn read(&self, reg: u8) -> u32 {
        match reg {
            3 => self.breakpoint_on_execute.get(),
            5 => self.breakpoint_on_data_access.get(),
            6 => self.jumpdest.get(),
            7 => self.breakpoint_control.get(),
            8 => self.bad_virtual_address.get(),
            9 => self.data_access_breakpoint_mask.get(),
            11 => self.execute_breakpoint_mask.get(), 
            12 => self.system_status.get(),
            13 => self.exception_cause.get(),
            14 => self.return_address_from_trap.get(), 
            15 => PROCESSOR_ID, 
            _ => panic!("Panico"),
        }
    }

    fn write(&self, reg: u8, val: u32) {
        match reg {
            3 => self.breakpoint_on_execute.set(val),
            5 => self.breakpoint_on_data_access.set(val),
            6 => self.jumpdest.set(val),
            7 => self.breakpoint_control.set(val),
            8 => self.bad_virtual_address.set(val),
            9 => self.data_access_breakpoint_mask.set(val),
            11 => self.execute_breakpoint_mask.set(val), 
            12 => self.system_status.set(val),
            13 => self.exception_cause.set(val),
            14 => self.return_address_from_trap.set(val), 
            15 => (), 
            _ => panic!("Panico"),
        }
    }

    fn command(&self, command: u32) {
        todo!()
    }
}

impl Cop0 {
    pub fn caches_isolated(&self) -> bool {
        self.system_status.get() & 0x10009 != 0
    }
}