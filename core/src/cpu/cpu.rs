use crate::{machine::Machine, bus::DeviceBusInterface};

use super::{instructions::*};

pub const REG_SP: usize = 29;
pub const REG_GP: usize = 28;
pub const REG_FP: usize = 30;
pub const REG_RA: usize = 31;
pub const REG_PC_RESET: u32 = 0xbfc00000;
pub struct Cpu {
    gprs: [u32;32],
    hi_lo: (u32,u32),
    pc: (u32,u32),
    
}

impl Default for Cpu {
    fn default() -> Self {
        Self { 
            gprs: Default::default(), 
            hi_lo: Default::default(), 
            pc: (REG_PC_RESET,REG_PC_RESET+4) 
        }
    }
}

impl Cpu {
    pub fn get_gpr(&self, gpr: u8) -> u32 {
        self.gprs[gpr as usize]
    }
    pub fn set_gpr(&mut self, gpr: u8, value: u32 ) {
        self.gprs[gpr as usize] = value;
    }
    
    pub fn set_pc(&mut self, pc: u32) {
        self.pc = (pc, pc + 4);
    }
    pub fn run(&mut self, machine: &Machine) -> bool{
        while self.step(machine) {}
        return false;
    }

    fn step_pc(&mut self, jump: Option<u32>) -> u32 {
        let current_pc = self.pc.0;
        if let Some(jump) = jump {
            self.pc = ( self.pc.0, jump);
        }
        else {
            self.pc = (self.pc.1, self.pc.1 + 4);
        }
        return current_pc;
    }
    fn raise_exception(&mut self, machine: &Machine, pc: u32 ) {
        todo!("TODO: Exceptions");
    }

    fn step(&mut self, machine: &Machine ) -> bool {
        let pc = self.step_pc(None);
        let inst = Inst::from(machine.bus.read32(pc));
        self.execute(machine, inst, pc)
    }
    fn execute(&mut self, machine: &Machine, inst: Inst, pc: u32) -> bool {

        macro_rules! reg { ($id:ident) => { self.gprs[$id as usize] };}
        macro_rules! var { ($id:ident) => { 
            match $id {
                VariantOperand::Reg( reg ) => reg!(reg) as _,
                VariantOperand::Imm( imm ) => (imm as i32) as u32
            }    
        };}

        match inst {
            Inst::Unknown => todo!(),
            Inst::Invalid => self.raise_exception(machine, pc),
            Inst::Nop => (),
            Inst::Move { dst, src } => {reg!(dst) = var!(src)},
            Inst::Add { dst, src1, src2, checked } => {
                if checked {
                    match reg!(src1).checked_add(var!(src2)) {
                        Some( result ) => reg!(dst) = result,
                        _ => self.raise_exception(machine, pc)
                    }
                }
                else {
                    reg!(dst) = reg!(src1).wrapping_add(var!(src2));
                }
            },
            Inst::Sub { dst, src1, src2, checked } => {
                if checked {
                    match reg!(src1).checked_sub(reg!(src2)) {
                        Some( result ) => reg!(dst) = result,
                        _ => self.raise_exception(machine, pc)
                    }
                }
                else {
                    reg!(dst) = reg!(src1).wrapping_sub(reg!(src2));
                }
            },
            Inst::And { dst, src1, src2 } => {
                reg!(dst) = reg!(src1) & var!(src2);
            },
            Inst::Or { dst, src1, src2 } => {
                reg!(dst) = reg!(src1) | var!(src2);
            },
            Inst::Xor { dst, src1, src2 } => {
                reg!(dst) = reg!(src1) ^ var!(src2);
            },
            Inst::Nor { dst, src1, src2 } => {
                reg!(dst) = !(reg!(src1) & reg!(src2));
            },
            Inst::SetLessThan { dst, src1, src2 } => {
                reg!(dst) = ((reg!(src1) as i32) < ( var!(src2) as i32 )) as u32;
            },
            Inst::SetLessThanUnsigned { dst, src1, src2 } => {
                reg!(dst) = ((reg!(src1) as u32) < ( var!(src2) as u32 )) as u32;
            },
            Inst::ShiftLeft { dst, src1, src2 } => {
                reg!(dst) = reg!(src1).wrapping_shl((var!(src2) & 0x1f) as u32);
            },
            Inst::ShiftRight { dst, src1, src2 } => {
                reg!(dst) = reg!(src1).wrapping_shr((var!(src2) & 0x1f) as u32);
            },
            Inst::ShiftRightArithmetic { dst, src1, src2 } => {
                reg!(dst) = (reg!(src1) as i32).wrapping_shr((var!(src2) & 0x1f) ) as u32;
            },
            Inst::LoadUpperImmediate { dst, src } => {
                reg!(dst) = (src as u32) << 16;
            },
            Inst::MultiplySigned { src1, src2 } => {

                let result: i64 = reg!(src1) as i64 * reg!(src2) as i64;
                
                self.hi_lo = ((result >> 32) as u32, result as u32);

            },
            Inst::MultiplyUnsigned { src1, src2 } => {
                let result: u64 = reg!(src1) as u64 * reg!(src2) as u64;

                self.hi_lo = ((result >> 32) as u32, result as u32);

            },
            Inst::DivideSigned { src1, src2 } => {
                if src2 == 0 {
                    self.hi_lo = ( 0, 0 );
                }
                else {
                    self.hi_lo = ( 
                        (reg!(src1) as i32 / reg!(src2) as i32) as u32,
                        (reg!(src1) as i32 % reg!(src2) as i32) as u32 
                    );
                }

            },
            Inst::DivideUnsigned { src1, src2 } => {
                if src2 == 0 {
                    self.hi_lo = ( 0, 0 );
                }
                else {
                    self.hi_lo = ( 
                        (reg!(src1) / reg!(src2)),
                        (reg!(src1) % reg!(src2)) 
                    );
                }

            },
            Inst::MoveFromHiLo { dst, src } => {
                match src {
                    HiLoRegs::Hi => reg!(dst) = self.hi_lo.0,
                    HiLoRegs::Lo => reg!(dst) = self.hi_lo.1
                };
            },
            Inst::MoveToHiLo { dst, src } => {
                match dst {
                    HiLoRegs::Hi => self.hi_lo.0 = reg!(src),
                    HiLoRegs::Lo => self.hi_lo.1 = reg!(src)
                };
            },
            Inst::Jump { dst, link } => {
                self.step_pc(Some((pc & 0xF0000000) + dst * 4));
                if link {
                    self.gprs[REG_RA] = pc + 8;
                }
            },
            Inst::JumpRegister { dst, link } => {
                let dst_pc = reg!(dst);
                self.step_pc(Some(dst_pc));
                if link {
                    
                    self.gprs[REG_RA] = pc + 8;
                }
            },
            Inst::CompareAndBranch { cond, lhs, rhs, dst, link } => {
                let lhs = reg!(lhs);
                let jumps = match cond {
                    Cond::Equal => lhs == reg!(rhs),
                    Cond::NotEqual => lhs != reg!(rhs),
                    Cond::LessThanZero => (lhs as i32) < 0,
                    Cond::GreaterEqualZero => (lhs as i32) >= 0,
                    Cond::GreaterThanZero => lhs > 0,
                    Cond::LessThanEqualZero => lhs <= 0
                };
                if jumps {
                    self.step_pc(Some(pc + (dst as i32 * 4) as u32));
                    if link {
                        self.gprs[REG_RA] = pc + 8;
                    }
                }
            },
            Inst::Syscall { .. } => self.raise_exception(machine, pc),
            Inst::Break { .. } => self.raise_exception(machine, pc),
            Inst::MoveFromCoprocessorData { coprocessor, src, dst } => {
                todo!()
                /*if let Some(coprocessor) = self.get_coprocessor(coprocessor as _) {
                    reg!(dst) = coprocessor.read( src);
                }
                else {
                    self.raise_exception(machine, pc)
                }*/
            },
            Inst::MoveToCoprocessorData { coprocessor, src, dst } => {
                /*let value = reg!(src);
                if let Some(coprocessor) = self.get_coprocessor_mut(coprocessor as _) {
                    coprocessor.write(dst, value);
                }
                else {
                    self.raise_exception(machine, pc)
                }*/
                todo!();
            },
            Inst::CopyFromCoprocessorControl { coprocessor, src, dst } => {
                todo!();
            },
            Inst::CopyToCoprocessorControl { coprocessor, src, dst } => {
                todo!();
            },
            Inst::CoprocessorRunCommand { coprocessor, command } => todo!(),
            Inst::LoadWordIntoCoprocessor { coprocessor, dst, base, offset } => todo!(),
            Inst::StoreWordFromCoprocessor { coprocessor, src, base, offset } => todo!(),
            Inst::LoadWord { dst, base, offset } => todo!(),
            Inst::LoadHalfWord { dst, base, offset, sign_extend } => todo!(),
            Inst::LoadByte { dst, base, offset, sign_extend } => todo!(),
            Inst::StoreWord { src, base, offset } => {
                machine.bus.write32(reg!(base).wrapping_add(offset as i32 as _), reg!(src));
            },
            Inst::StoreHalfWord { src, base, offset } => todo!(),
            Inst::StoreByte { src, base, offset } => todo!(),
        }

        return true;
    }

}

#[cfg(test)]
mod tests {
    use std::borrow::{BorrowMut, Borrow};

    use crate::bus;

    use super::*;
    #[test]
    fn run_some_instructions() {
        let machine = Machine::with_bus(bus::Bus::with_empty_bios());
        let mut cpu = machine.cpu.borrow_mut();
        cpu.execute(&machine, Inst::Move { dst: 1, src: VariantOperand::Imm(4) }, 0);
        cpu.execute(&machine, Inst::Move { dst: 2, src: VariantOperand::Imm(4) }, 0);
        cpu.execute(&machine, Inst::Add { dst: 1, src1: 1, src2: VariantOperand::Reg(2), checked: false}, 0);

        assert_eq!(cpu.get_gpr(1), 8);
    }
    #[test]
    fn run_sum_1_to_n() {
        /*
            sum(1,n) = n * ( n + 1) / 2
         */
        let machine = Machine::with_bus(bus::Bus::with_empty_bios());
        let mut cpu = machine.cpu.borrow_mut();

        let big_number = 32;

        let code = [
            Inst::Move { dst: 1, src: VariantOperand::Imm(big_number) },
            Inst::Move { dst: 2, src: VariantOperand::Imm(2)},
            Inst::Add { dst: 3, src1: 1, src2: VariantOperand::Imm(1), checked: false },
            Inst::MultiplyUnsigned { src1: 1, src2: 3 },
            Inst::MoveFromHiLo { dst: 3, src: HiLoRegs::Lo },
            Inst::DivideUnsigned { src1: 3, src2: 2 },
            Inst::MoveFromHiLo { dst: 1, src: HiLoRegs::Hi }
        ];
        for i in code.iter() {
            cpu.execute(&machine, Inst::from(*i), 0);
        }
        assert_eq!(cpu.borrow().get_gpr(1), (big_number * (big_number+1)/2) as u32);
    }
}
