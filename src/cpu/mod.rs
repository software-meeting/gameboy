mod instructions;
mod registers;

use crate::memory::Memory;
use instructions::{Instruction, R16, R8, U3};
use registers::Registers;

pub(crate) struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    fn execute(&mut self, i: Instruction, memory: &mut Memory) -> u8 {
        match i {
            Instruction::ADC_A_r8 { r8 } => {
                self.add_a(self.registers.get_r8(r8), true);
                4
            }
            Instruction::ADC_A_HL => {
                let rhs = memory.read(self.registers.get_hl());
                self.add_a(rhs, true);
                8
            }
            Instruction::ADC_A_n8 { n8 } => {
                self.add_a(n8, true);
                8
            }
            Instruction::ADD_A_r8 { r8 } => {
                self.add_a(self.registers.get_r8(r8), false);
                4
            }
            Instruction::ADD_A_HL => {
                let rhs = memory.read(self.registers.get_hl());
                self.add_a(rhs, false);
                8
            }
            Instruction::ADD_A_n8 { n8 } => {
                self.add_a(n8, false);
                8
            }
            Instruction::ADD_HL_r16 { r16 } => {
                self.add_hl(self.registers.get_r16(r16));
                8
            }
            Instruction::ADD_HL_SP => {
                self.add_hl(self.registers.sp);
                8
            }
            Instruction::ADD_SP_e8 { e8 } => {
                self.add_sp(e8);
                16
            }
            Instruction::AND_A_r8 { r8 } => {
                self.and(self.registers.get_r8(r8));
                4
            }
            Instruction::AND_A_HL => {
                let rhs = memory.read(self.registers.get_hl());
                self.and(rhs);
                8
            }
            Instruction::AND_A_n8 { n8 } => {
                self.and(n8);
                8
            }
            Instruction::BIT_U3_R8 { u3, r8 } => {
                self.bit(u3, self.registers.get_r8(r8));
                8
            }
            Instruction::BIT_U3_HL { u3 } => {
                let byte = memory.read(self.registers.get_hl());
                self.bit(u3, byte);
                12
            }
            Instruction::CALL_N16 { n16 } => todo!(),
            Instruction::CALL_CC_N16 { condition, n16 } => todo!(),
            Instruction::CCF => {
                self.registers.f.carry = !self.registers.f.carry;
                4
            }
            Instruction::CP_A_R8 { r8 } => todo!(),
            Instruction::CP_A_HL => todo!(),
            Instruction::CPL => todo!(),
            Instruction::DAA => todo!(),
            Instruction::DEC_R8 { r8 } => todo!(),
            Instruction::DEC_HL => todo!(),
            Instruction::DEC_R16 { r16 } => todo!(),
            Instruction::DEC_SP => todo!(),
            Instruction::DI => todo!(),
            Instruction::EI => todo!(),
            Instruction::HALT => todo!(),
            Instruction::INC_R8 { r8 } => todo!(),
            Instruction::INC_HL => todo!(),
            Instruction::INC_R16 { r16 } => todo!(),
            Instruction::INC_SP => todo!(),
            Instruction::JP_N16 { n16 } => todo!(),
            Instruction::JP_CC_N16 { condition, n16 } => todo!(),
            Instruction::JP_HL => todo!(),
            Instruction::JR_N16 { n16 } => todo!(),
            Instruction::JR_CC_N16 { condition, n16 } => todo!(),
            Instruction::LD_R8_R8 { dest, src } => todo!(),
            Instruction::LD_R8_N8 { dest, n8 } => todo!(),
            Instruction::LD_R16_R16 { dest, src } => todo!(),
            Instruction::LD_HL_R8 { r8 } => todo!(),
            Instruction::LD_R8_HL { r8 } => todo!(),
            Instruction::LD_R16_A { r16 } => todo!(),
            Instruction::LD_N16_A { n16 } => todo!(),
            Instruction::LDH_N16_A { n16 } => todo!(),
            Instruction::LDH_C_A => todo!(),
            Instruction::LD_A_R16 { r16 } => todo!(),
            Instruction::LD_A_N16 { n16 } => todo!(),
            Instruction::LDH_A_R16 { r16 } => todo!(),
            Instruction::LDH_A_N16 { n16 } => todo!(),
            Instruction::LD_A_C => todo!(),
            Instruction::LD_HLI_A => todo!(),
            Instruction::LD_HLD_A => todo!(),
            Instruction::LD_A_HLI => todo!(),
            Instruction::LD_SP_N16 { n16 } => todo!(),
            Instruction::LD_N16_SP { n16 } => todo!(),
            Instruction::LD_HL_SPE8 {} => todo!(),
            Instruction::LD_HL_SPE => todo!(),
            Instruction::LD_SP_HL => todo!(),
            Instruction::NOP => todo!(),
            Instruction::OR_A_R8 { r8 } => todo!(),
            Instruction::OR_A_HL => todo!(),
            Instruction::OR_A_N8 { n8 } => todo!(),
            Instruction::POP_AF => todo!(),
            Instruction::POP_R16 { r16 } => todo!(),
            Instruction::PUSH_AF => todo!(),
            Instruction::PUSH_R16 { r16 } => todo!(),
            Instruction::RES_U3_R8 { u3, r8 } => todo!(),
            Instruction::RES_U3_HL { u3 } => todo!(),
            Instruction::RET_CC { condition } => todo!(),
            Instruction::RETI => todo!(),
            Instruction::RL_R8 { r8 } => todo!(),
            Instruction::RL_HL => todo!(),
            Instruction::RLA => todo!(),
            Instruction::RLC_R8 { r8 } => todo!(),
            Instruction::RLC_HL => todo!(),
            Instruction::RLCA => todo!(),
            Instruction::RR_R8 { r8 } => todo!(),
            Instruction::RR_HL => todo!(),
            Instruction::RRA => todo!(),
            Instruction::RRC_R8 { r8 } => todo!(),
            Instruction::RRC_HL => todo!(),
            Instruction::RRCA => todo!(),
            Instruction::RST { vec } => todo!(),
            Instruction::SBC_A_R8 { r8 } => todo!(),
            Instruction::SBC_A_HL => todo!(),
            Instruction::SBC_A_N8 { n8 } => todo!(),
            Instruction::SCF => todo!(),
            Instruction::SET_U3_R8 { u3, r8 } => todo!(),
            Instruction::SET_U3_HL { u3 } => todo!(),
            Instruction::SLA_R8 { r8 } => todo!(),
            Instruction::SLA_HL => todo!(),
            Instruction::SRA_R8 { r8 } => todo!(),
            Instruction::SRA_HL => todo!(),
            Instruction::SRL_R8 { r8 } => todo!(),
            Instruction::SRL_HL => todo!(),
            Instruction::STOP => todo!(),
            Instruction::SUB_A_R8 { r8 } => todo!(),
            Instruction::SUB_A_HL => todo!(),
            Instruction::SUB_A_N8 { n8 } => todo!(),
            Instruction::SWAP_R8 { r8 } => todo!(),
            Instruction::SWAP_HL => todo!(),
            Instruction::XOR_A_R8 { r8 } => todo!(),
            Instruction::XOR_A_HL => todo!(),
            Instruction::XOR_A_N8 { n8 } => todo!(),
        }
    }

    fn add_a(&mut self, rhs: u8, carry: bool) {
        let (mut sum, mut did_overflow) = self.registers.a.overflowing_add(rhs);
        if carry {
            let (carry_sum, carry_overflow) = sum.overflowing_add(1);
            sum = carry_sum;
            did_overflow = did_overflow || carry_overflow;
        }

        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (rhs & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;

        self.registers.a = sum;
    }

    fn add_hl(&mut self, rhs: u16) {
        let (sum, did_overflow) = self.registers.get_hl().overflowing_add(rhs);

        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFFF) + (rhs & 0xFFF) > 0xFFF;
        self.registers.f.carry = did_overflow;

        self.registers.set_hl(sum);
    }

    fn add_sp(&mut self, rhs: i8) {
        let rhs = rhs as i16 as u16;
        let sum = self.registers.sp.wrapping_add(rhs);

        self.registers.f.half_carry = (self.registers.sp & 0xF) + (rhs & 0xF) > 0xF;
        self.registers.f.carry = (self.registers.sp & 0xFF) + (rhs & 0xFF) > 0xFF;

        self.registers.sp = sum;
    }

    fn and(&mut self, rhs: u8) {
        self.registers.a &= rhs;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
    }

    fn bit(&mut self, u3: U3, byte: u8) {
        let bit = match u3 {
            U3::Zero => byte & 0b1,
            U3::One => byte & 0b10,
            U3::Two => byte & 0b100,
            U3::Three => byte & 0b1000,
            U3::Four => byte & 0b10000,
            U3::Five => byte & 0b100000,
            U3::Six => byte & 0b1000000,
            U3::Seven => byte & 0b10000000,
        };
        self.registers.f.zero = bit != 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }
}
