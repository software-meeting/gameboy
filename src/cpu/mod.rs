mod instructions;
mod registers;

use crate::memory::Memory;
use instructions::{Instruction, R16, R8, U3};
use registers::Registers;

#[derive(PartialEq)]
enum ImeState {
    UNSET,
    PENDING,
    SET,
}
pub(crate) struct Cpu {
    registers: Registers,
    ime_state: ImeState,
    low_power_mode: bool,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            registers: Registers::new(),
            ime_state: ImeState::UNSET,
            low_power_mode: false,
        }
    }

    fn execute(&mut self, i: Instruction, memory: &mut Memory) -> u8 {
        let t_cycle = match i {
            Instruction::ADC_A_R8 { r8 } => {
                self.add_a(self.registers.get_r8(&r8), true);
                4
            }
            Instruction::ADC_A_HL => {
                let rhs = memory.read(self.registers.get_hl());
                self.add_a(rhs, true);
                8
            }
            Instruction::ADC_A_N8 { n8 } => {
                self.add_a(n8, true);
                8
            }
            Instruction::ADD_A_R8 { r8 } => {
                self.add_a(self.registers.get_r8(&r8), false);
                4
            }
            Instruction::ADD_A_HL => {
                let rhs = memory.read(self.registers.get_hl());
                self.add_a(rhs, false);
                8
            }
            Instruction::ADD_A_N8 { n8 } => {
                self.add_a(n8, false);
                8
            }
            Instruction::ADD_HL_R16 { r16 } => {
                self.add_hl(self.registers.get_r16(&r16));
                8
            }
            Instruction::ADD_HL_SP => {
                self.add_hl(self.registers.sp);
                8
            }
            Instruction::ADD_SP_E8 { e8 } => {
                self.add_sp(e8);
                16
            }
            Instruction::AND_A_R8 { r8 } => {
                self.and(self.registers.get_r8(&r8));
                4
            }
            Instruction::AND_A_HL => {
                let rhs = memory.read(self.registers.get_hl());
                self.and(rhs);
                8
            }
            Instruction::AND_A_N8 { n8 } => {
                self.and(n8);
                8
            }
            Instruction::BIT_U3_R8 { u3, r8 } => {
                self.bit(u3, self.registers.get_r8(&r8));
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
            Instruction::CP_A_R8 { r8 } => {
                self.cp(self.registers.get_r8(&r8));
                4
            }
            Instruction::CP_A_HL => {
                self.cp(memory.read(self.registers.get_hl()));
                8
            }
            Instruction::CP_A_N8 { n8 } => {
                self.cp(n8);
                8
            }
            Instruction::CPL => {
                self.registers.a = !self.registers.a;
                self.registers.f.zero = false;
                self.registers.f.carry = false;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
                4
            }
            Instruction::DAA => {
                // https://blog.ollien.com/posts/gb-daa/
                let a = self.registers.a;
                // Check & adjust lower nibble
                if (a & 0x0F) > 9 || self.registers.f.half_carry {
                    self.registers.a = a.wrapping_add(0x06);
                    self.registers.f.carry = false;
                }
                // Check & adjust upper nibble
                if (a & 0xF0) > 0x90 || self.registers.f.carry {
                    self.registers.a = a.wrapping_add(0x60);
                    self.registers.f.carry = true;
                }
                self.registers.f.zero = self.registers.a == 0;
                4
            }
            Instruction::DEC_R8 { r8 } => {
                let r8_val = self.registers.get_r8(&r8);
                let r8_val_dec = r8_val - 1;
                self.registers.set_r8(&r8, r8_val_dec);
                self.registers.f.zero = r8_val_dec == 0;
                self.registers.f.subtract = true;
                // Carry from 4th bit required if it flipped
                self.registers.f.half_carry = r8_val_dec & 0x8 != r8_val & 0x8;
                4
            }
            Instruction::DEC_HL => {
                let hl = self.registers.get_hl();
                let hl_val = memory.read(hl);
                let hl_val_dec = hl_val - 1;
                memory.write(hl, hl_val_dec);
                self.registers.f.zero = hl_val_dec == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = hl_val_dec & 0x8 != hl_val & 0x8;
                12
            }
            Instruction::DEC_R16 { r16 } => {
                let r16_val = self.registers.get_r16(&r16);
                let r16_val_dec = r16_val - 1;
                self.registers.set_r16(&r16, r16_val_dec);
                8
            }
            Instruction::DEC_SP => {
                let sp_val = self.registers.sp;
                let sp_val_dec = sp_val - 1;
                self.registers.sp = sp_val_dec;
                8
            }
            Instruction::DI => {
                self.ime_state = ImeState::UNSET;
                4
            }
            Instruction::EI => {
                if self.ime_state != ImeState::PENDING {
                    self.ime_state = ImeState::PENDING;
                }
                4
            }
            Instruction::HALT => {
                if self.ime_state == ImeState::SET {
                    self.low_power_mode = true;
                } else {
                    // Implement interrupt "none pending" and "some pending" branching
                    todo!();
                }
                4
            }
            Instruction::INC_R8 { r8 } => {
                let r8_val = self.registers.get_r8(&r8);
                let r8_val_inc = r8_val + 1;
                self.registers.set_r8(&r8, r8_val_inc);
                self.registers.f.zero = r8_val_inc == 0;
                self.registers.f.subtract = false;
                // Check for 3rd bit overflow
                self.registers.f.half_carry = r8_val_inc & 0x8 != r8_val & 0x8;
                4
            }
            Instruction::INC_HL => {
                let hl = self.registers.get_hl();
                let hl_val = memory.read(hl);
                let hl_val_inc = hl_val + 1;
                memory.write(hl, hl_val_inc);
                self.registers.f.zero = hl_val_inc == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = hl_val_inc & 0x8 != hl_val & 0x8;
                12
            }
            Instruction::INC_R16 { r16 } => {
                let r16_val = self.registers.get_r16(&r16);
                let r16_val_inc = r16_val + 1;
                self.registers.set_r16(&r16, r16_val_inc);
                8
            }
            Instruction::INC_SP => {
                let sp_val = self.registers.sp;
                let sp_val_inc = sp_val + 1;
                self.registers.sp = sp_val_inc;
                8
            }
            Instruction::JP_N16 { n16 } => {
                self.jump(n16);
                16
            }
            Instruction::JP_CC_N16 { condition, n16 } => {
                let proceed = self.check_condition(condition);
                if proceed {
                    self.jump(n16);
                    return 16; // 4 cycles if condition met
                }
                12 // 3 cycles if condition fails
            }
            Instruction::JP_HL => {
                self.jump(self.registers.get_hl());
                4
            }
            Instruction::JR_N16 { offset } => {
                let position = self.position_from_offset(offset);
                self.jump(position);
                12
            }
            Instruction::JR_CC_N16 { condition, offset } => {
                let proceed = self.check_condition(condition);
                if proceed {
                    let position = self.position_from_offset(offset);
                    self.jump(position);
                    return 12; // 3 cycles if condition met
                }
                8 // 2 cycles if condition fails
            }
            Instruction::LD_R8_R8 { dest, src } => {
                let value = self.registers.get_r8(&src);
                self.registers.set_r8(&dest, value);
                4
            }
            Instruction::LD_R8_N8 { dest, n8 } => {
                self.registers.set_r8(&dest, n8);
                8
            }
            Instruction::LD_R16_R16 { dest, src } => {
                let value = self.registers.get_r16(&src);
                self.registers.set_r16(&dest, value);
                12
            }
            Instruction::LD_HL_R8 { r8 } => {
                let value = self.registers.get_r8(&r8);
                memory.write(self.registers.get_hl(), value);
                8
            }
            Instruction::LD_HL_N8 { n8 } => {
                memory.write(self.registers.get_hl(), n8);
                12
            }
            Instruction::LD_R8_HL { r8 } => {
                let value = memory.read(self.registers.get_hl());
                self.registers.set_r8(&r8, value);
                8
            }
            Instruction::LD_R16_A { r16 } => {
                let value = self.registers.a;
                let dest = self.registers.get_r16(&r16);
                memory.write(dest, value);
                8
            }
            Instruction::LD_N16_A { n16 } => {
                let value = self.registers.a;
                memory.write(n16, value);
                16
            }
            Instruction::LDH_N16_A { n16 } => {
                if (0xFF00..0xFFFF).contains(&n16) {
                    let value = memory.read(n16);
                    self.registers.a = value;
                }
                12
            }
            Instruction::LDH_C_A => {
                let value_address = 0xFF00 + self.registers.c as u16;
                let value = memory.read(value_address);
                self.registers.a = value;
                8
            }
            Instruction::LD_A_R16 { r16 } => {
                let value = memory.read(self.registers.get_r16(&r16));
                self.registers.a = value;
                8
            }
            Instruction::LD_A_N16 { n16 } => {
                let value = memory.read(n16);
                self.registers.a = value;
                16
            }
            Instruction::LDH_A_N16 { n16 } => {
                if (0xFF00..0xFFFF).contains(&n16) {
                    let value = memory.read(n16);
                    self.registers.a = value;
                }
                12
            }
            Instruction::LD_A_C => {
                let value_address = 0xFF00 + self.registers.c as u16;
                let value = memory.read(value_address);
                self.registers.a = value;
                8
            }
            Instruction::LD_HLI_A => {
                let value = self.registers.a;
                let hl = self.registers.get_hl();
                memory.write(hl, value);
                self.registers.set_hl(hl.wrapping_add(1));
                8
            }
            Instruction::LD_HLD_A => {
                let value = self.registers.a;
                let hl = self.registers.get_hl();
                memory.write(hl, value);
                self.registers.set_hl(hl.wrapping_sub(1));
                8
            }
            Instruction::LD_A_HLD => {
                let hl = self.registers.get_hl();
                let value = memory.read(hl);
                self.registers.a = value;
                self.registers.set_hl(hl.wrapping_sub(1));
                8
            }
            Instruction::LD_A_HLI => {
                let hl = self.registers.get_hl();
                let value = memory.read(hl);
                self.registers.a = value;
                self.registers.set_hl(hl.wrapping_add(1));
                8
            }
            Instruction::LD_SP_N16 { n16 } => {
                self.registers.sp = n16;
                12
            }
            Instruction::LD_N16_SP { n16 } => {
                memory.write(n16, (self.registers.sp & 0xFF) as u8);
                memory.write(n16.wrapping_add(1), (self.registers.sp >> 8) as u8);
                20
            }
            Instruction::LD_HL_SPE8 { e8 } => {
                let sp = self.registers.sp;
                let value = sp + e8 as i16 as u16;
                self.registers.set_hl(value);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (self.registers.sp & 0xF) + (value & 0xF) > 0xF;
                self.registers.f.carry = (self.registers.sp & 0xFF) + (&0xFF) > 0xFF;
                12
            }
            Instruction::LD_SP_HL => {
                self.registers.sp = self.registers.get_hl();
                8
            }
            Instruction::NOP => 1,
            Instruction::OR_A_R8 { r8 } => {
                self.or_a(self.registers.get_r8(&r8));
                1
            }
            Instruction::OR_A_HL => todo!(),
            Instruction::OR_A_N8 { n8 } => {
                self.or_a(n8);
                2
            }
            Instruction::POP_AF => {
                self.registers.f = registers::Flags::from(memory.read(self.registers.sp));
                self.registers.sp += 1;
                self.registers.a = memory.read(self.registers.sp);
                12
            }
            Instruction::POP_R16 { r16 } => {
                let lower = r16.get_lower();
                let upper = r16.get_upper();
                self.registers
                    .set_r8(&lower, memory.read(self.registers.sp));
                self.registers.sp += 1;
                self.registers
                    .set_r8(&upper, memory.read(self.registers.sp));
                self.registers.sp += 1;
                12
            }
            Instruction::PUSH_AF => todo!(),
            Instruction::PUSH_R16 { r16 } => todo!(),
            Instruction::RES_U3_R8 { u3, r8 } => {
                self.res_r8(instructions::U3::get(&u3), r8);
                2
            }
            Instruction::RES_U3_HL { u3 } => {
                self.res_hl(memory, instructions::U3::get(&u3), self.registers.get_hl());
                4
            }
            Instruction::RET => todo!(),
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
        };
        if self.ime_state == ImeState::PENDING {
            self.ime_state = ImeState::SET;
        }
        t_cycle
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
    fn jump(&mut self, n16: u16) {
        self.registers.pc = n16;
    }
    fn position_from_offset(&mut self, offset: i8) -> u16 {
        let position = self.registers.pc;
        if offset > 0 {
            let cast_offset = offset as u16;
            return position.wrapping_add(cast_offset);
        } else {
            let cast_offset = -offset as u16;
            return position.wrapping_sub(cast_offset);
        }
    }
    fn check_condition(&mut self, condition: instructions::Condition) -> bool {
        match condition {
            instructions::Condition::Z => self.registers.f.zero,
            instructions::Condition::NZ => !self.registers.f.zero,
            instructions::Condition::C => self.registers.f.carry,
            instructions::Condition::NC => !self.registers.f.carry,
        }
    }
    fn cp(&mut self, subtrahend: u8) {
        let result = self.registers.a - subtrahend;
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        // Isolate lower nibble and check if a carry from the 4th bit was required
        self.registers.f.half_carry = self.registers.a & 0xF < subtrahend & 0xF;
        // Check if carry was required for the full operation
        self.registers.f.carry = subtrahend > self.registers.a;
    }
    fn dec_r16(&mut self, r16: R16) {
        let r16_val = self.registers.get_r16(&r16);
        let r16_val_dec = r16_val - 1;
        self.registers.set_r16(&r16, r16_val_dec);
        self.registers.f.zero = r16_val_dec == 0;
        self.registers.f.subtract = true;
        // Carry from 4th bit required if it flipped
        self.registers.f.half_carry = r16_val_dec & 0x8 != r16_val & 0x8;
    }
    fn or_a(&mut self, rhs: u8) {
        self.registers.a = self.registers.a | rhs;
        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }

    fn res_r8(&mut self, bit: u8, r8: R8) {
        let mask = !(1 << bit);
        let value = self.registers.get_r8(&r8) & mask;
        self.registers.set_r8(&r8, value);
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }

    fn res_hl(&mut self, memory: &mut Memory, bit: u8, hl_ref: u16) {
        let mask = !(1 << bit);
        let hl = memory.read(hl_ref);
        memory.write(hl_ref, hl & mask);
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }
}
