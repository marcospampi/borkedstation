type Reg = u8;

#[derive(Debug,Copy,Clone)]
pub enum VariantOperand {
    Imm(i16),
    Reg(Reg),
}

#[derive(Debug,Copy,Clone)]
pub enum HiLoRegs {
    Hi,
    Lo,
}

#[derive(Debug,Copy,Clone)]
pub enum Cond {
    Equal, NotEqual,
    LessThanZero,
    GreaterEqualZero,
    GreaterThanZero,
    LessThanEqualZero
}


#[derive(Debug,Copy,Clone)]
pub enum Inst {
    Unknown,
    Invalid,
    Nop,
    Move {
        dst: Reg, src: VariantOperand
    },
    Add {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
        checked: bool
    },
    Sub {
        dst: Reg,
        src1: Reg,
        src2: Reg,
        checked: bool
    },
    And {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
    },
    Or {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
    },
    Xor {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
    },
    Nor {
        dst: Reg,
        src1: Reg,
        src2: Reg,
    },
    SetLessThan {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
    },
    SetLessThanUnsigned {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
    },
    ShiftLeft {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
    },
    ShiftRight {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
    },
    ShiftRightArithmetic {
        dst: Reg,
        src1: Reg,
        src2: VariantOperand,
    },
    LoadUpperImmediate {
        dst: Reg,
        src: i16,
    },
    MultiplySigned {
        src1: Reg,
        src2: Reg,
    },
    MultiplyUnsigned {
        src1: Reg,
        src2: Reg,
    },
    DivideSigned {
        src1: Reg,
        src2: Reg,
    },
    DivideUnsigned {
        src1: Reg,
        src2: Reg,
    },
    MoveFromHiLo {
        dst: Reg,
        src: HiLoRegs,
    },
    MoveToHiLo {
        dst: HiLoRegs,
        src: Reg,
    },
    Jump {
        dst: u32,
        link: bool,
    },
    JumpRegister {
        dst: Reg,
        link: bool,
    },
    CompareAndBranch {
        cond: Cond,
        lhs: Reg, rhs: Reg,
        dst: i16, link: bool
    },
    Syscall {
        comment: u32
    },
    Break {
        comment: u32
    },
    MoveFromCoprocessorData {
        coprocessor: u8,
        src: Reg, dst: Reg
    },
    MoveToCoprocessorData {
        coprocessor: u8,
        src: Reg, dst: Reg
    },
    CopyFromCoprocessorControl {
        coprocessor: u8,
        src: Reg, dst: Reg
    },
    CopyToCoprocessorControl {
        coprocessor: u8,
        src: Reg, dst: Reg
    },
    CoprocessorRunCommand {
        coprocessor: u8,
        command: u32
    },
    LoadWordIntoCoprocessor {
        coprocessor: u8,
        dst: Reg, base: Reg, offset: i16
    },
    StoreWordFromCoprocessor {
        coprocessor: u8,
        src: Reg, base: Reg, offset: i16
    },
    LoadWord {
        dst: Reg, base: Reg, offset: i16, 
    },
    LoadHalfWord {
        dst: Reg, base: Reg, offset: i16, sign_extend: bool,
    },
    LoadByte {
        dst: Reg, base: Reg, offset: i16, sign_extend: bool
    },
    StoreWord {
        src: Reg, base: Reg, offset: i16
    },
    StoreHalfWord {
        src: Reg, base: Reg, offset: i16
    },
    StoreByte {
        src: Reg, base: Reg, offset: i16
    }

}

impl Default for Inst {
    fn default() -> Self {
        Inst::Unknown
    }
}

impl From<u32> for Inst {
    fn from(word: u32) -> Self {
        macro_rules! shamt {() => {((word >> 6) &0x1F) as i16};}
        macro_rules! rd {() => {((word >> 11) &0x1F) as u8};}
        macro_rules! rt {() => {((word >> 16) &0x1F) as u8};}
        macro_rules! rs {() => {((word >> 21) &0x1F) as u8};}
        macro_rules! funct {() => {(word) &0x3F};}
        macro_rules! opcode {() => {(word>>26) &0x3F};}
        macro_rules! imm {() => {(((word)&0xFFFF) as i16) };}
        macro_rules! imm26 {() => {((((word)&0x3ffffff) << 6) as i32 >> 6) as u32 };}
        macro_rules! coproc {() => {(word>>26) &0x3};}
        macro_rules! coproc_cmd {() => {(word>>26) &0x1ffffff};}
        let first_match = match opcode!() {
            0b000000 => match funct!() {
                0b000100 => Inst::ShiftLeft { dst: rd!(), src1: rt!(), src2: VariantOperand::Reg(rs!()) },
                0b000110 => Inst::ShiftRight { dst: rd!(), src1: rt!(), src2: VariantOperand::Reg(rs!()) },
                0b000111 => Inst::ShiftRightArithmetic { dst: rd!(), src1: rt!(), src2: VariantOperand::Reg(rs!()) },
                0b000000 => Inst::ShiftLeft { dst: rd!(), src1: rt!(), src2: VariantOperand::Imm(shamt!()) },
                0b000010 => Inst::ShiftRight { dst: rd!(), src1: rt!(), src2: VariantOperand::Imm(shamt!()) },
                0b000011 => Inst::ShiftRightArithmetic { dst: rd!(), src1: rt!(), src2: VariantOperand::Imm(shamt!()) },
                0b001000 => Inst::JumpRegister { dst: rs!(), link: false },
                0b001001 => Inst::JumpRegister { dst: rs!(), link: true },
                0b001100 => Inst::Syscall { comment: 0 },
                0b001101 => Inst::Break { comment: 0 },
                0b010000 => Inst::MoveFromHiLo { dst: rd!(), src: HiLoRegs::Hi },
                0b010001 => Inst::MoveToHiLo { dst: HiLoRegs::Hi, src: rs!() },
                0b010010 => Inst::MoveFromHiLo { dst: rd!(), src: HiLoRegs::Lo },
                0b010011 => Inst::MoveToHiLo { dst: HiLoRegs::Lo, src: rs!() },
                0b011000 => Inst::MultiplySigned { src1: rs!(), src2: rt!() },
                0b011001 => Inst::MultiplyUnsigned { src1: rs!(), src2: rt!() },
                0b011010 => Inst::DivideSigned { src1: rs!(), src2: rt!() },
                0b011011 => Inst::DivideUnsigned { src1: rs!(), src2: rt!() },
                0b100000 => Inst::Add { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!()) , checked: true},
                0b100001 => Inst::Add { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!()) , checked: false} ,
                0b100010 => Inst::Sub { dst: rd!(), src1: rs!(), src2: rt!() , checked: true},
                0b100011 => Inst::Sub { dst: rd!(), src1: rs!(), src2: rt!() , checked: false} ,
                0b100100 => Inst::And { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!())},
                0b100101 => Inst::Or  { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!())},
                0b100110 => Inst::Xor { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!())},
                0b100111 => Inst::Nor { dst: rd!(), src1: rs!(), src2: rt!()},
                0b101010 => Inst::SetLessThan { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!()) },
                0b101011 => Inst::SetLessThanUnsigned { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!()) },
                _        => Inst::Invalid,
            },
            0b000001 => {
                let rt = rt!();
                let link = rt & 0b10000 != 0;
                let cond = {
                    if rt & 0b00001 != 0 {
                        Cond::LessThanZero
                    } else {
                        Cond::GreaterEqualZero
                    }
                };

                Inst::CompareAndBranch { cond: cond, lhs: rs!(), rhs: 0, dst: imm!(), link: link }

            },
            0b000010 => Inst::Jump { dst: imm26!(), link: false },
            0b000011 => Inst::Jump { dst: imm26!(), link: true },
            0b000100 => Inst::CompareAndBranch { cond: Cond::Equal , lhs: rs!(), rhs: rt!(), dst: imm!(), link: false },
            0b000101 => Inst::CompareAndBranch { cond: Cond::NotEqual , lhs: rs!(), rhs: rt!(), dst: imm!(), link: false },
            0b000110 => Inst::CompareAndBranch { cond: Cond::LessThanEqualZero, lhs: rs!(), rhs: 0, dst: imm!(), link: false },
            0b000111 => Inst::CompareAndBranch { cond: Cond::GreaterThanZero, lhs: rs!(), rhs: 0, dst: imm!(), link: false },
            0b001000 => Inst::Add { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!()), checked: true },
            0b001001 => Inst::Add { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!()), checked: false },
            0b001010 => Inst::SetLessThan { dst: rd!(), src1: rs!(), src2: VariantOperand::Imm(imm!()) },
            0b001011 => Inst::SetLessThanUnsigned { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!()) },
            0b001100 => Inst::And { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!())},
            0b001101 => Inst::Or  { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!())},
            0b001110 => Inst::Xor { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!())},
            0b001111 => Inst::LoadUpperImmediate { dst: rt!(), src: imm!() },
            0b010000   //Inst::CoprocessorRunCommand { coprocessor: 0, command: coproc_cmd!() },
            |0b010001  //Inst::CoprocessorRunCommand { coprocessor: 1, command: coproc_cmd!() },
            |0b010010  //Inst::CoprocessorRunCommand { coprocessor: 2, command: coproc_cmd!() },
            |0b010011  => {
                /*  31..26 |25..21|20..16|15..11|10..6 |  5..0  |
                     6bit  | 5bit | 5bit | 5bit | 5bit |  6bit  |
                    -------+------+------+------+------+--------+------------
                    0100nn |0|0000| rt   | rd   | N/A  | 000000 | MFCn rt,rd_dat  ;rt = dat
                    0100nn |0|0010| rt   | rd   | N/A  | 000000 | CFCn rt,rd_cnt  ;rt = cnt
                    0100nn |0|0100| rt   | rd   | N/A  | 000000 | MTCn rt,rd_dat  ;dat = rt
                    0100nn |0|0110| rt   | rd   | N/A  | 000000 | CTCn rt,rd_cnt  ;cnt = rt
                    0100nn |0|1000|00000 | <--immediate16bit--> | BCnF target ;jump if false
                    0100nn |0|1000|00001 | <--immediate16bit--> | BCnT target ;jump if true
                    0100nn |1| <--------immediate25bit--------> | COPn imm25
                    010000 |1|0000| N/A  | N/A  | N/A  | 000001 | COP0 01h  ;=TLBR
                    010000 |1|0000| N/A  | N/A  | N/A  | 000010 | COP0 02h  ;=TLBWI
                    010000 |1|0000| N/A  | N/A  | N/A  | 000110 | COP0 06h  ;=TLBWR
                    010000 |1|0000| N/A  | N/A  | N/A  | 001000 | COP0 08h  ;=TLBP
                    010000 |1|0000| N/A  | N/A  | N/A  | 010000 | COP0 10h  ;=RFE
                    1100nn | rs   | rt   | <--immediate16bit--> | LWCn rt_dat,[rs+imm]
                    1110nn | rs   | rt   | <--immediate16bit--> | SWCn rt_dat,[rs+imm]
                 */
                let coprocessor = coproc!() as _;
                let opt = rs!();
                let funct = funct!();
                match (opt,funct) {
                    (0b00100,0b00000) => Inst::MoveToCoprocessorData { coprocessor, src: rd!(), dst: rt!() },
                    (0b00110,0b00000) => Inst::CopyToCoprocessorControl { coprocessor, src: rd!(), dst: rt!() },
                    (0b00000,0b00000) => Inst::MoveFromCoprocessorData { coprocessor, dst: rd!(), src: rt!() },
                    (0b00010,0b00000) => Inst::CopyFromCoprocessorControl { coprocessor,  dst: rd!(), src: rt!()},
                    _ => Inst::CoprocessorRunCommand { coprocessor, command: coproc_cmd!() }
                }

            }//Inst::CoprocessorRunCommand { coprocessor: 3, command: coproc_cmd!() },
            0b100000 => Inst::LoadByte{ dst:rt!(), base:rs!(), offset:imm!(), sign_extend: true},
            0b100001 => Inst::LoadHalfWord{ dst:rt!(), base:rs!(), offset:imm!(), sign_extend: true},
            //0b100010 => Inst::Load{ dst:rt!(), base:rs!(), offset:imm!() },//self.op_lwl(instruction, debugger, shared),
            0b100011 => Inst::LoadWord{ dst:rt!(), base:rs!(), offset:imm!() },
            0b100100 => Inst::LoadByte{ dst:rt!(), base:rs!(), offset:imm!() ,sign_extend: false},
            0b100101 => Inst::LoadHalfWord{ dst:rt!(), base:rs!(), offset:imm!() ,sign_extend: false},//self.op_lhu(instruction, debugger, shared),
            //0b100110 => self.op_lwr(instruction, debugger, shared),
            0b101000 => Inst::StoreByte { src: rt!(), base: rs!(), offset: imm!() },//self.op_sb(instruction, debugger, shared, renderer),
            0b101001 => Inst::StoreHalfWord { src: rt!(), base: rs!(), offset: imm!() },//self.op_sh(instruction, debugger, shared, renderer),
            //0b101010 => self.op_swl(instruction, debugger, shared, renderer),
            0b101011 => Inst::StoreWord { src: rt!(), base: rs!(), offset: imm!() },
            //0b101110 => self.op_swr(instruction, debugger, shared, renderer),
            0b110000 => Inst::LoadWordIntoCoprocessor { coprocessor: 0, dst: rt!(), base: rs!(), offset: imm!() },//self.op_lwc0(instruction),
            0b110001 => Inst::LoadWordIntoCoprocessor { coprocessor: 1, dst: rt!(), base: rs!(), offset: imm!() },//self.op_lwc1(instruction),
            0b110010 => Inst::LoadWordIntoCoprocessor { coprocessor: 2, dst: rt!(), base: rs!(), offset: imm!() },//self.op_lwc2(instruction, debugger, shared),
            0b110011 => Inst::LoadWordIntoCoprocessor { coprocessor: 3, dst: rt!(), base: rs!(), offset: imm!() },//self.op_lwc3(instruction),
            0b111000 => Inst::StoreWordFromCoprocessor { coprocessor: 0, src: rt!(), base: rs!(), offset: imm!() },//self.op_swc0(instruction),
            0b111001 => Inst::StoreWordFromCoprocessor { coprocessor: 1, src: rt!(), base: rs!(), offset: imm!() },//self.op_swc1(instruction),
            0b111010 => Inst::StoreWordFromCoprocessor { coprocessor: 2, src: rt!(), base: rs!(), offset: imm!() },//self.op_swc2(instruction, debugger, shared, renderer),
            0b111011 => Inst::StoreWordFromCoprocessor { coprocessor: 3, src: rt!(), base: rs!(), offset: imm!() },//self.op_swc3(instruction),
            _        => Inst::Invalid,
        };
        match first_match {
            Inst::ShiftLeft { dst: 0,..}
            |Inst::ShiftRight { dst: 0,..}
            |Inst::ShiftRightArithmetic { dst: 0,..}
            |Inst::Add { dst: 0,..} 
            |Inst::SetLessThan { dst: 0,..}
            |Inst::SetLessThanUnsigned { dst: 0,..}
            |Inst::And { dst: 0,..}
            |Inst::Or { dst: 0,..}
            |Inst::Xor { dst: 0,..}
            |Inst::Nor { dst: 0,..}
            |Inst::LoadUpperImmediate { dst: 0,..}
            |Inst::LoadWord { dst: 0,..}
            |Inst::LoadByte { dst: 0,..}
            |Inst::MoveFromHiLo { dst: 0,..}
            |Inst::MoveFromCoprocessorData { dst: 0,..}
            |Inst::LoadHalfWord { dst: 0,..} => Inst::Nop,
            _ => first_match
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_must_be_8byte() {
        assert!(
            std::mem::size_of::<Inst>() <= 8,
            "Instruction size must be less than or equal than 8, current {}",
            std::mem::size_of::<Inst>()
        );
    }

    #[test]
    fn nop_test() {
        let insts = [0x2020000a,0x34000000];
        for inst in insts.iter().map(|e| Inst::from(*e)) {
            let valid = match inst {
                Nop => true,
                _ => false
            };
            assert!(valid, "Should be Nop, got {:?}", inst );
        }
        assert!(
            std::mem::size_of::<Inst>() <= 8,
            "Instruction size must be less than or equal than 8, current {}",
            std::mem::size_of::<Inst>()
        );
    }

    #[test]
    fn instruction_test () {
        let insts = [
            0x2408001e, // ; |input:12| addiu $t0, $zero, 30
            0x24090014, // ; |input:14| addiu $t1, $zero, 20
            0x01092020, // ; |input:18| add $a0, $t0, $t1
            0x24020001, // ; |input:22| addiu $v0, $zero, 1
            0x0000000c, // ; |input:26| syscall
            0x240400c0, // ; |input:33| addiu $a0, $zero, str[hi]
            0x00042400, // ; |input:34| sll $a0, $a0, 16
            0x24840000, // ; |input:35| addiu $a0, $a0, str[lo]
            0x24020004, // ; |input:38| addiu $v0, $zero, 4
            0x0000000c, // ; |input:42| syscall
            0x24020000, // ; |input:45| addiu $v0, $zero, 0
            0x03e00008 // ; |input:46| jr $ra
        ];

        for inst in insts.iter().map(|e| Inst::from(*e)) {

        }
    }
}
