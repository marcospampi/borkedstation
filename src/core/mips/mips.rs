use std::{cell::Cell, ptr::NonNull};

use crate::core::{machine::Machine, bus::BusDevice};

use super::{cop0::Cop0, gte::Gte, };
pub const REG_SP: usize = 29;
pub const REG_GP: usize = 28;
pub const REG_FP: usize = 30;
pub const REG_RA: usize = 31;
pub const REG_PC_RESET: u32 = 0xbfc00000;

pub struct Mips {
    pub cop0: Cop0,
    pub cop2: Gte,
    gprs: [Cell<u32>; 32],
    hi_lo: (Cell<u32>, Cell<u32>),
    pc: Cell<(u32, u32)>,

    pub machine: NonNull<Machine>
}


impl Mips {

    pub fn new(machine: NonNull<Machine>) -> Self {
        Self {
            cop0: Cop0 {  },
            cop2: Gte {  },
            gprs: Default::default(),
            hi_lo: Default::default(),
            pc: Cell::new((REG_PC_RESET, REG_PC_RESET + 4)),
            machine
        }
    }
    fn get_machine(&self) -> &Machine {
        unsafe { std::mem::transmute(self.machine) }
    }
    fn jump(&self, offset: u32) -> u32 {
        todo!()
    }
    fn step_pc(&self, pc: u32 ) -> u32 {
        todo!()
    }
    fn execute(&self, inst: u32, pc: u32) {
        self.gprs[0].set(0);
        macro_rules! shamt {() => {((inst >> 6) &0x1F) as i16};}
        macro_rules! rd {() => {((inst >> 11) &0x1F) as u8};}
        macro_rules! rt {() => {((inst >> 16) &0x1F) as u8};}
        macro_rules! rs {() => {((inst >> 21) &0x1F) as u8};}
        macro_rules! funct {() => {(inst) &0x3F};}
        macro_rules! opcode {() => {(inst>>26) &0x3F};}
        macro_rules! imm {() => {((((inst)&0xFFFF) as i16) as i32) as u32 };}
        macro_rules! imm26 {() => {((((inst)&0x3ffffff) << 6) as i32 >> 6) as u32 };}
        macro_rules! coproc {() => {(inst>>26) &0x3};}
        macro_rules! coproc_cmd {() => {(inst>>26) &0x1ffffff};}

        macro_rules! reg { 
            ( get $reg: expr) => { self.gprs[$reg as usize].get() };
            ( set $reg: expr , $val: expr ) => { self.gprs[$reg as usize].set($val)}; 
        }
        macro_rules! get {
            ( $reg: expr) => { self.gprs[$reg as usize].get() };
        }
        macro_rules! set {
            ( $reg: expr, $val: expr) => { self.gprs[$reg as usize].set($val) };
        }
        let opcode_funct_pair = (opcode!(), funct!());

        match opcode_funct_pair {
            // sll shift left
            (0b000000, 0b000100) => set!(rd!(), get!(rs!()) << get!(rt!())),

            // slr shift right
            (0b000000, 0b000110) => {set!(rd!(), get!(rs!()) >> get!(rt!()))},

            // slra shift right arithmetic
            (0b000000, 0b000111) => set!(rd!(), (get!(rs!()) as i32 >> get!(rt!()) as i32) as u32 ), 

            // slra shift left with shamt
            (0b000000, 0b000000) => set!(rd!(), get!(rs!()) << shamt!()),

            // slra shift right with shamt
            (0b000000, 0b000010) => set!(rd!(), get!(rs!()) >> shamt!()),

            // slra shift right arithmetic with shamt
            (0b000000, 0b000011) => set!(rd!(), (get!(rs!()) as i32 >> shamt!() as i32) as u32 ),

            // jump register
            (0b000000, 0b001000) => {
                self.jump(get!(rs!()));
            },

            // jump and link register
            (0b000000, 0b001001) => {
                self.jump(get!(rs!()));
                self.gprs[REG_RA].set(pc + 8);
            },
            
            // syscall
            (0b000000, 0b001100) => todo!(),
            
            // break
            (0b000000, 0b001101) => todo!(),
            
            // move from hi
            (0b000000, 0b010000) => { set!(rd!(), self.hi_lo.0.get())},

            // move from lo
            (0b000000, 0b010010) => { set!(rd!(), self.hi_lo.1.get())},
            
            // move to hi
            (0b000000, 0b010001) => { self.hi_lo.0.set(get!(rs!()))},
            
            // move to lo
            (0b000000, 0b010011) => { self.hi_lo.1.set(get!(rs!()))},
            
            // Multiply signed
            (0b000000, 0b011000) => {
                let a = get!(rs!()) as i32 as i64;
                let b = get!(rt!()) as i32 as i64;

                let c = (a * b) as u64;

                self.hi_lo.0.set((c >> 32) as u32);
                self.hi_lo.1.set((c) as u32);

            },

            // Multiply unsigned
            (0b000000, 0b011001) => {
                let a = get!(rs!()) as u64;
                let b = get!(rt!()) as u64;

                let c = a * b;

                self.hi_lo.0.set((c >> 32) as u32);
                self.hi_lo.1.set((c) as u32);
            },

            // Divide signed
            (0b000000, 0b011010) => {
                let a = get!(rs!()) as i32;
                let b = get!(rt!()) as i32;
                
                if b != 0 {
                    let d = (a / b) as u32;
                    let r = (a % b ) as u32;
                    self.hi_lo.0.set(d);
                    self.hi_lo.1.set(r);

                } else {
                    self.hi_lo.0.set(0xDEADBEEF);
                    self.hi_lo.1.set(0xDEADBEEF);
                }
            },

            // Divide unsigned
            (0b000000, 0b011011) => {
                let a = get!(rs!());
                let b = get!(rt!());
                
                if b != 0 {
                    let d = a / b;
                    let r = a % b;
                    self.hi_lo.0.set(d);
                    self.hi_lo.1.set(r);

                } else {
                    self.hi_lo.0.set(0xDEADBEEF);
                    self.hi_lo.1.set(0xDEADBEEF);
                }
            },
            // add
            (0b000000, 0b100000) => {
                let rs = get!(rs!());
                let rt = get!(rt!());
                
                match rs.checked_add(rt) {
                    Some(rd) => set!(rd!(), rd),
                    _ => panic!("No exception till now")
                }
            },
            // addu
            (0b000000, 0b100001) => set!(rd!(), get!(rs!()).wrapping_add(get!(rt!()))),
            // sub
            (0b000000, 0b100010) => {
                let rs = get!(rs!());
                let rt = get!(rt!());
                
                match rs.checked_sub(rt) {
                    Some(rd) => set!(rd!(), rd),
                    _ => panic!("No exception till now")
                }
            },
            // subu
            (0b000000, 0b100011) => set!(rd!(), get!(rs!()).wrapping_sub(get!(rt!()))),
            // and
            (0b000000, 0b100100) => set!(rd!(), get!(rs!()) & get!(rt!())),// Inst::And { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!())},
            // or
            (0b000000, 0b100101) => set!(rd!(), get!(rs!()) | get!(rt!())),// Inst::Or  { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!())},
            // xor
            (0b000000, 0b100110) => set!(rd!(), get!(rs!()) ^ get!(rt!())),// Inst::Xor { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!())},
            // nor
            (0b000000, 0b100111) => set!(rd!(), !(get!(rs!()) | get!(rt!()))),// Inst::Nor { dst: rd!(), src1: rs!(), src2: rt!()},
            // slt
            (0b000000, 0b101010) => set!(rd!(), (get!(rs!()) as i32 > get!(rt!()) as i32) as u32 ),// Inst::SetLessThan { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!()) },
            // sltu
            (0b000000, 0b101011) => set!(rd!(), (get!(rs!()) > get!(rt!())) as u32 ),// Inst::SetLessThanUnsigned { dst: rd!(), src1: rs!(), src2: VariantOperand::Reg(rt!()) },
            // blt, bgtz
            (0b000001, _) => {
                let bits = rt!();
                let should_link = bits & 0b10000 != 0;
                let should_jump: bool = {
                    if bits & 0b00001 != 0 {
                        (get!(rs!()) as i32 ) < 0
                    } else {
                        (get!(rs!()) as i32 ) >= 0
                        
                    }
                };
                
                if should_jump {
                    self.jump((imm!() as i32 as u32) << 2);
                    if should_link {
                        self.gprs[REG_RA].set(pc + 8);
                    }
                }


            },
            // j/jmp
            (0b000010, _) => {
                self.jump(imm26!() as i32 as _);
            },
            // jal/jump and link
            (0b000011, _) => {
                self.jump(imm26!() as i32 as _);
                self.gprs[REG_RA].set(pc + 8);
            },
            // beq
            (0b000100, _) => {
                let cond = get!(rs!()) == get!(rt!());
                if cond {
                    self.jump(imm!() << 2);
                }
            } ,
            // bne
            (0b000101, _) => {
                let cond = get!(rs!()) != get!(rt!());
                if cond {
                    self.jump(imm!() << 2);
                }
            },
            // bltz
            (0b000110, _) => {
                let cond = get!(rs!()) as i32 <= 0;
                if cond {
                    self.jump(imm!() << 2);
                }
            },
            // bgtz
            (0b000111, _) => {
                let cond = get!(rs!()) as i32 > 0;
                if cond {
                    self.jump(imm!() << 2);
                }
            },
            // addi 
            (0b001000, _) => {
                let rs = get!(rs!());
                
                match rs.checked_add(imm!()) {
                    Some(rd) => set!(rd!(), rd),
                    _ => panic!("No exception till now")
                }
            }, 
            // addiu
            (0b001001, _) => set!(rt!(), get!(rs!()).wrapping_add(imm!()) ),
            // slti
            (0b001010, _) => set!(rd!(), (get!(rs!()) as i32 > imm!() as i32) as u32 ), // Inst::SetLessThan { dst: rd!(), src1: rs!(), src2: VariantOperand::Imm(imm!()) },
            // sltui
            (0b001011, _) => set!(rd!(), (get!(rs!()) > imm!()) as u32 ), // Inst::SetLessThanUnsigned { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!()) },
            // andi
            (0b001100, _) => set!(rd!(), get!(rs!()) & imm!() ), // Inst::And { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!())},
            // ori
            (0b001101, _) => set!(rd!(), get!(rs!()) | imm!() ), // Inst::Or  { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!())},
            // xori
            (0b001110, _) => set!(rd!(), get!(rs!()) ^ imm!() ), // Inst::Xor { dst: rt!(), src1: rs!(), src2: VariantOperand::Imm(imm!())},
            // lui
            (0b001111, _) => set!(rd!(), get!(rd!()) | imm!() << 16 ), // Inst::LoadUpperImmediate { dst: rt!(), src: imm!() },
            //0b010000   //Inst::CoprocessorRunCommand { coprocessor: 0, command: coproc_cmd!() },
            //|0b010001  //Inst::CoprocessorRunCommand { coprocessor: 1, command: coproc_cmd!() },
            //|0b010010  //Inst::CoprocessorRunCommand { coprocessor: 2, command: coproc_cmd!() },
            //|0b010011  => todo!(), /*{
            //    let coprocessor = coproc!() as _;
            //    let opt = rs!();
            //    let funct = funct!();
            //    match (opt,funct) {
            //        (0b00100,0b00000) => Inst::MoveToCoprocessorData { coprocessor, src: rd!(), dst: rt!() },
            //        (0b00110,0b00000) => Inst::CopyToCoprocessorControl { coprocessor, src: rd!(), dst: rt!() },
            //        (0b00000,0b00000) => Inst::MoveFromCoprocessorData { coprocessor, dst: rd!(), src: rt!() },
            //        (0b00010,0b00000) => Inst::CopyFromCoprocessorControl { coprocessor,  dst: rd!(), src: rt!()},
            //        _ => Inst::CoprocessorRunCommand { coprocessor, command: coproc_cmd!() }
            //    }*/
//
            //}//Inst::CoprocessorRunCommand { coprocessor: 3, command: coproc_cmd!() },
            (0b100000,_) => {
                let addr = get!(rs!()).wrapping_add(imm!());
                match self.get_machine().read::<u8>(addr) {
                    Ok( val ) => set!(rt!(), ((val as i8) as i32) as u32),
                    Err( err ) => panic!("Bus error! {:?}", err)
                }
            }, //Inst::LoadByte{ dst:rt!(), base:rs!(), offset:imm!(), sign_extend: true},
            (0b100001,_) => {
                let addr = get!(rs!()).wrapping_add(imm!());
                match self.get_machine().read::<u16>(addr) {
                    Ok( val ) => set!(rt!(), ((val as i16) as i32) as u32),
                    Err( err ) => panic!("Bus error! {:?}", err)
                }
            }, //Inst::LoadHalfWord{ dst:rt!(), base:rs!(), offset:imm!(), sign_extend: true},
            (0b100010,_) => {
                todo!("lwl")
            },
            (0b100011,_) => {
                let addr = get!(rs!()).wrapping_add(imm!());
                match self.get_machine().read::<u32>(addr) {
                    Ok( val ) => set!(rt!(),val),
                    Err( err ) => panic!("Bus error! {:?}", err)
                }
            }, // Inst::LoadWord{ dst:rt!(), base:rs!(), offset:imm!() },
            (0b100100,_) => {
                let addr = get!(rs!()).wrapping_add(imm!());
                match self.get_machine().read::<u8>(addr) {
                    Ok( val ) => set!(rt!(),val as u32),
                    Err( err ) => panic!("Bus error! {:?}", err)
                }
            }, // Inst::LoadByte{ dst:rt!(), base:rs!(), offset:imm!() ,sign_extend: false},
            (0b100101,_) => {
                let addr = get!(rs!()).wrapping_add(imm!());
                match self.get_machine().read::<u16>(addr) {
                    Ok( val ) => set!(rt!(),val as u32),
                    Err( err ) => panic!("Bus error! {:?}", err)
                }
            }, // Inst::LoadHalfWord{ dst:rt!(), base:rs!(), offset:imm!() ,sign_extend: false},//self.op_lhu(instruction, debugger, shared),
            (0b100110,_) => todo!(), //self.op_lwr(instruction, debugger, shared),
            (0b101000,_) => {
                let addr = get!(rs!()).wrapping_add(imm!());
                match self.get_machine().write::<u8>(addr, get!(rt!()) as _) {
                    Ok( _ ) => (),
                    Err( err ) => panic!("Bus error! {:?}", err)
                }
            },//Inst::StoreByte { src: rt!(), base: rs!(), offset: imm!() },//self.op_sb(instruction, debugger, shared, renderer),
            (0b101001,_) => {
                let addr = get!(rs!()).wrapping_add(imm!());
                match self.get_machine().write::<u16>(addr, get!(rt!()) as _) {
                    Ok( _ ) => (),
                    Err( err ) => panic!("Bus error! {:?}", err)
                }
            },//Inst::StoreHalfWord { src: rt!(), base: rs!(), offset: imm!() },//self.op_sh(instruction, debugger, shared, renderer),
            (0b101010,_) => todo!(), //self.op_swl(instruction, debugger, shared, renderer),
            (0b101011,_) => {
                let addr = get!(rs!()).wrapping_add(imm!());
                match self.get_machine().write::<u32>(addr, get!(rt!())) {
                    Ok( _ ) => (),
                    Err( err ) => panic!("Bus error! {:?}", err)
                }
            }, //Inst::StoreWord { src: rt!(), base: rs!(), offset: imm!() },
            (0b101110,_) => todo!(), //self.op_swr(instruction, debugger, shared, renderer),
            (0b110000,_) => todo!(), //Inst::LoadWordIntoCoprocessor { coprocessor: 0, dst: rt!(), base: rs!(), offset: imm!() },//self.op_lwc0(instruction),
            (0b110001,_) => todo!(), //Inst::LoadWordIntoCoprocessor { coprocessor: 1, dst: rt!(), base: rs!(), offset: imm!() },//self.op_lwc1(instruction),
            (0b110010,_) => todo!(), //Inst::LoadWordIntoCoprocessor { coprocessor: 2, dst: rt!(), base: rs!(), offset: imm!() },//self.op_lwc2(instruction, debugger, shared),
            (0b110011,_) => todo!(), //Inst::LoadWordIntoCoprocessor { coprocessor: 3, dst: rt!(), base: rs!(), offset: imm!() },//self.op_lwc3(instruction),
            (0b111000,_) => todo!(), //Inst::StoreWordFromCoprocessor { coprocessor: 0, src: rt!(), base: rs!(), offset: imm!() },//self.op_swc0(instruction),
            (0b111001,_) => todo!(), //Inst::StoreWordFromCoprocessor { coprocessor: 1, src: rt!(), base: rs!(), offset: imm!() },//self.op_swc1(instruction),
            (0b111010,_) => todo!(), //Inst::StoreWordFromCoprocessor { coprocessor: 2, src: rt!(), base: rs!(), offset: imm!() },//self.op_swc2(instruction, debugger, shared, renderer),
            (0b111011,_) => todo!(), //Inst::StoreWordFromCoprocessor { coprocessor: 3, src: rt!(), base: rs!(), offset: imm!() },//self.op_swc3(instruction),
            _        => todo!(), //Inst::Invalid,
        }


    }
}
#[cfg(test)]
mod tests {
    use crate::core::machine::Machine;
    use super::*;
    #[test]
    fn test() {
        let machine = Machine::new();
        let n = 1024;
        let expected = n * (n + 1) / 2;
        machine.cpu.gprs[4].set(n);
        let insts = [
            0x24820001, // addiu	r2,r4,1
            0x00440018, // mult	r2,r4
            0x00001812, // mflo	v1
            0x00031042, // srl	r2,r3,0x1f
            0x00431021  // addu	r2,r2,r3
        ];
        //for (i,inst) in insts.iter().enumerate() {
        //    machine.cpu.execute(*inst, (i*4) as u32);
        //}
        {
            machine.cpu.execute(0x24820001, 0 ); // addiu	r2,r4,1 
            machine.cpu.execute(0x00440018, 0 ); // mult	r2,r4 
            machine.cpu.execute(0x00001812, 0 ); // mflo	r3 
            machine.cpu.execute(0x00031082, 0 ); // srl	r2,r3,0x1f 
            machine.cpu.execute(0x00431021, 0 ); // addu	r2,r2,r3 
        }
        assert_eq!(machine.cpu.gprs[2].get(), expected);
    }
}
