use super::Coprocessor;
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

pub struct Cop0 {}

impl Coprocessor for Cop0 {
    fn read(&self, reg: u8) -> u32 {
        todo!()
    }

    fn write(&self, reg: u8, val: u32) {
        todo!()
    }

    fn command(&self, command: u32) {
        todo!()
    }
}
