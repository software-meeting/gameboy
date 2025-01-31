mod instructions;
mod registers;

use crate::memory::Memory;
use instructions::{Instruction, R16, R8, U3};
use registers::{Flags, Registers};

#[derive(PartialEq)]
enum ImeState {
    UNSET,
    PENDING_NEW_INSTRUCTION,
    PENDING_INSTRUCTION_COMPLETION,
    SET,
}
pub(crate) struct Cpu {
    registers: Registers,
    ime_state: ImeState,
    low_power_mode: bool,
    very_low_power_mode: bool,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            registers: Registers::new(),
            ime_state: ImeState::UNSET,
            low_power_mode: false,      // HALT
            very_low_power_mode: false, // STOP
        }
    }

    fn execute(&mut self, i: Instruction, memory: &mut Memory) -> u8 {
        // Update IME (if we are halfway thru di or ei instruction)
        // Check IME || if halted
        //

        let t_cycle = match i {
            /*
             * Add the value in r8 plus the carry flag to A.
             */
            Instruction::ADC_A_R8 { r8 } => {
                self.add_a(self.registers.get_r8(&r8), true);
                4
            }
            /*
             * Add the byte pointed to by HL plus the carry flag to A.
             */
            Instruction::ADC_A_HL_PNTR => {
                let rhs = memory.read(self.registers.get_hl());
                self.add_a(rhs, true);
                8
            }
            /*
             *  Add the value n8 plus the carry flag to A.
             */
            Instruction::ADC_A_N8 { n8 } => {
                self.add_a(n8, true);
                8
            }
            /*
             * Add the value in r8 to A.
             */
            Instruction::ADD_A_R8 { r8 } => {
                self.add_a(self.registers.get_r8(&r8), false);
                4
            }
            /*
             * Add the byte pointed to by HL to A.
             */
            Instruction::ADD_A_HL_PNTR => {
                let rhs = memory.read(self.registers.get_hl());
                self.add_a(rhs, false);
                8
            }
            /*
             * Add the value n8 to A.
             */
            Instruction::ADD_A_N8 { n8 } => {
                self.add_a(n8, false);
                8
            }
            /*
             * Add the value in r16 to HL.
             */
            Instruction::ADD_HL_R16 { r16 } => {
                self.add_hl(self.registers.get_r16(&r16));
                8
            }
            /*
             * Add the value in SP to HL.
             */
            Instruction::ADD_HL_SP => {
                self.add_hl(self.registers.sp);
                8
            }
            /*
             * Add the signed value e8 to SP.
             */
            Instruction::ADD_SP_E8 { e8 } => {
                self.add_sp(e8);
                16
            }
            /*
             * Bitwise AND between the value in r8 and A.
             */
            Instruction::AND_A_R8 { r8 } => {
                self.and(self.registers.get_r8(&r8));
                4
            }
            /*
             * Bitwise AND between the byte pointed to by HL and A.
             */
            Instruction::AND_A_HL_PNTR => {
                let rhs = memory.read(self.registers.get_hl());
                self.and(rhs);
                8
            }
            /*
             * Bitwise AND between the value in n8 and A.
             */
            Instruction::AND_A_N8 { n8 } => {
                self.and(n8);
                8
            }
            /*
             * Test bit u3 in register r8, set the zero flag if bit not set.
             */
            Instruction::BIT_U3_R8 { u3, r8 } => {
                self.bit(u3, self.registers.get_r8(&r8));
                8
            }
            /*
             * Test bit u3 in the byte pointed by HL, set the zero flag if bit not set.
             */
            Instruction::BIT_U3_HL_PNTR { u3 } => {
                let byte = memory.read(self.registers.get_hl());
                self.bit(u3, byte);
                12
            }
            /*
             * Push address of the next instruction on the stack, such that RET can pop it later.
             * Subsequently executes a JP n16; effectively storing n16 to PC.
             */
            Instruction::CALL_N16 { n16 } => {
                self.call(memory, n16);
                24
            }
            /*
             * If condition is met, execute a CALL instruction. Refer above for details.
             */
            Instruction::CALL_CC_N16 { condition, n16 } => {
                let proceed = self.check_condition(condition);
                if proceed {
                    self.call(memory, n16);
                    return 12;
                }
                24
            }
            /*
             * Complement Carry Flag; C = -C.
             */
            Instruction::CCF => {
                self.registers.f.carry = !self.registers.f.carry;
                4
            }
            /*
             * Subtract the value in r8 from A and set flags accordingly, but don't store the result.
             * Useful for comparing values.
             */
            Instruction::CP_A_R8 { r8 } => {
                self.cp(self.registers.get_r8(&r8));
                4
            }
            /*
             * Subtract the byte pointed to by HL from A and set flags accordingly, but don't store the result.
             */
            Instruction::CP_A_HL_PNTR => {
                self.cp(memory.read(self.registers.get_hl()));
                8
            }
            /*
             * Subtract the value n8 from A and set flags accordingly, but don't store the result.
             */
            Instruction::CP_A_N8 { n8 } => {
                self.cp(n8);
                8
            }
            /*
             * Complement accumulator; A = -A.
             */
            Instruction::CPL => {
                self.registers.a = !self.registers.a;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
                4
            }
            /*
             * Adjust the accumulator value if BCD representation after an arithmetic instruction.
             * Reference https://blog.ollien.com/posts/gb-daa/ for details.
             */
            Instruction::DAA => {
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
            /*
             *  Decrement value in register r8 by 1.
             */
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
            /*
             * Decrement the byte pointed to by HL by 1.
             */
            Instruction::DEC_HL_PNTR => {
                let hl = self.registers.get_hl();
                let hl_val = memory.read(hl);
                let hl_val_dec = hl_val - 1;
                memory.write(hl, hl_val_dec);
                self.registers.f.zero = hl_val_dec == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = hl_val_dec & 0x8 != hl_val & 0x8;
                12
            }
            /*
             * Decrement value in register r16 by 1.
             */
            Instruction::DEC_R16 { r16 } => {
                let r16_val = self.registers.get_r16(&r16);
                let r16_val_dec = r16_val - 1;
                self.registers.set_r16(&r16, r16_val_dec);
                8
            }
            /*
             * Decrement the stack pointer by 1.
             */
            Instruction::DEC_SP => {
                let sp_val = self.registers.sp;
                let sp_val_dec = sp_val - 1;
                self.registers.sp = sp_val_dec;
                8
            }
            /*
             * Disable Interrupts; clears the IME flag state.
             */
            Instruction::DI => {
                self.ime_state = ImeState::UNSET;
                4
            }
            /*
             * Enable Interrupts; sets the IME flag state to wait for another instruction, such to be set subequent to its execution.
             */
            Instruction::EI => {
                self.set_ime();
                4
            }
            /*
             * Enter low power mode until an interrupt occurs.
             * This behaviour depends on the state of the ime flag.
             */
            Instruction::HALT => {
                if self.ime_state == ImeState::SET {
                    self.low_power_mode = true;
                } else {
                    // Implement interrupt branching
                    todo!();
                }
                4
            }
            /*
             * Increment value in register r8 by 1.
             */
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
            /*
             * Increment the byte pointed to by HL by 1.
             */
            Instruction::INC_HL_PNTR => {
                let hl = self.registers.get_hl();
                let hl_val = memory.read(hl);
                let hl_val_inc = hl_val + 1;
                memory.write(hl, hl_val_inc);
                self.registers.f.zero = hl_val_inc == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = hl_val_inc & 0x8 != hl_val & 0x8;
                12
            }
            /*
             * Increment value in register r16 by 1.
             */
            Instruction::INC_R16 { r16 } => {
                let r16_val = self.registers.get_r16(&r16);
                let r16_val_inc = r16_val + 1;
                self.registers.set_r16(&r16, r16_val_inc);
                8
            }
            /*
             * Increment value in register SP by 1.
             */
            Instruction::INC_SP => {
                let sp_val = self.registers.sp;
                let sp_val_inc = sp_val + 1;
                self.registers.sp = sp_val_inc;
                8
            }
            /*
             * Jump to address n16; effectively, store n16 into PC.
             */
            Instruction::JP_N16 { n16 } => {
                self.jump(n16);
                16
            }
            /*
             * If condition is met, jump to address n16.
             */
            Instruction::JP_CC_N16 { condition, n16 } => {
                let proceed = self.check_condition(condition);
                if proceed {
                    self.jump(n16);
                    return 16; // 4 cycles if condition met
                }
                12 // 3 cycles if condition fails
            }
            /*
             * Jump to the address within HL.
             */
            Instruction::JP_HL => {
                self.jump(self.registers.get_hl());
                4
            }
            /*
             * Relative jump a certain signed offset away.
             */
            Instruction::JR_N16 { offset } => {
                // TODO: I'm fairly sure this is completely incorrect. Re-implementation is required.
                let position = self.position_from_offset(offset);
                self.jump(position);
                12
            }
            /*
             * Relative jump if the condition is met.
             */
            Instruction::JR_CC_N16 { condition, offset } => {
                let proceed = self.check_condition(condition);
                if proceed {
                    let position = self.position_from_offset(offset);
                    self.jump(position);
                    return 12; // 3 cycles if condition met
                }
                8 // 2 cycles if condition fails
            }
            /*
             * Load (copy) the value in register src to the register dest.
             */
            Instruction::LD_R8_R8 { dest, src } => {
                let value = self.registers.get_r8(&src);
                self.registers.set_r8(&dest, value);
                4
            }
            /*
             * Load value n8 into a 8-bit register.
             */
            Instruction::LD_R8_N8 { dest, n8 } => {
                self.registers.set_r8(&dest, n8);
                8
            }
            /*
             * Load value n16 into a 16-bit register.
             */
            Instruction::LD_R16_R16 { dest, src } => {
                let value = self.registers.get_r16(&src);
                self.registers.set_r16(&dest, value);
                12
            }
            /*
             * Store value in register r8 into the byte pointed to by register HL.
             */
            Instruction::LD_HL_PNTR_R8 { r8 } => {
                let value = self.registers.get_r8(&r8);
                memory.write(self.registers.get_hl(), value);
                8
            }
            /*
             * Store value n8 into the byte pointed to by register HL.
             */
            Instruction::LD_HL_PNTR_N8 { n8 } => {
                memory.write(self.registers.get_hl(), n8);
                12
            }
            /*
             * Load value into register r8 from the byte pointed to by register HL.
             */
            Instruction::LD_R8_HL_PNTR { r8 } => {
                let value = memory.read(self.registers.get_hl());
                self.registers.set_r8(&r8, value);
                8
            }
            /*
             * Store value in register A into the byte pointed to by register r16.
             */
            Instruction::LD_R16_PNTR_A { r16 } => {
                let value = self.registers.a;
                let dest = self.registers.get_r16(&r16);
                memory.write(dest, value);
                8
            }
            /*
             * Store value in register A into the byte at address n16.
             */
            Instruction::LD_N16_PNTR_A { n16 } => {
                let value = self.registers.a;
                memory.write(n16, value);
                16
            }
            /*
             * Store value in register A into the byte at address n16, provided n16 is between FF00 & FFFF.
             */
            Instruction::LDH_N16_PNTR_A { n16 } => {
                if (0xFF00..0xFFFF).contains(&n16) {
                    let value = memory.read(n16);
                    self.registers.a = value;
                }
                12
            }
            /*
             * Store value in register A into the byte at address $FF00+C; effectively, use (+ve) offsets rather than absolute addresses.
             */
            Instruction::LDH_C_PNTR_A => {
                let value_address = 0xFF00 + self.registers.c as u16;
                let value = memory.read(value_address);
                self.registers.a = value;
                8
            }
            /*
             * Load value in register A from the byte pointed to by register r16.
             */
            Instruction::LD_A_R16_PNTR { r16 } => {
                let value = memory.read(self.registers.get_r16(&r16));
                self.registers.a = value;
                8
            }
            /*
             * Load value in register A from the byte at address n16.
             */
            Instruction::LD_A_N16_PNTR { n16 } => {
                let value = memory.read(n16);
                self.registers.a = value;
                16
            }
            /*
             * Load value in register A from the byte at address n16, provided the address is between $FF00 and $FFFF.
             */
            Instruction::LDH_A_N16_PNTR { n16 } => {
                if (0xFF00..0xFFFF).contains(&n16) {
                    let value = memory.read(n16);
                    self.registers.a = value;
                }
                12
            }
            /*
             * Load value in register A from the byte at address $FF00+c.
             */
            Instruction::LDH_A_C_PNTR => {
                let value_address = 0xFF00 + self.registers.c as u16;
                let value = memory.read(value_address);
                self.registers.a = value;
                8
            }
            /*
             * Store value in register A into the byte pointed by HL and increment HL afterwards.
             */
            Instruction::LD_HLI_PNTR_A => {
                let value = self.registers.a;
                let hl = self.registers.get_hl();
                memory.write(hl, value);
                self.registers.set_hl(hl.wrapping_add(1));
                8
            }
            /*
             * Store value in register A into the byte pointed by HL and decrement HL afterwards.
             */
            Instruction::LD_HLD_PNTR_A => {
                let value = self.registers.a;
                let hl = self.registers.get_hl();
                memory.write(hl, value);
                self.registers.set_hl(hl.wrapping_sub(1));
                8
            }
            /*
             * Load value into register A from the byte pointed by HL and decrement HL afterwards.
             */
            Instruction::LD_A_HLD_PNTR => {
                let hl = self.registers.get_hl();
                let value = memory.read(hl);
                self.registers.a = value;
                self.registers.set_hl(hl.wrapping_sub(1));
                8
            }
            /*
             * Load value into register A from the byte pointed by HL and increment HL afterwards.
             */
            Instruction::LD_A_HLI_PNTR => {
                let hl = self.registers.get_hl();
                let value = memory.read(hl);
                self.registers.a = value;
                self.registers.set_hl(hl.wrapping_add(1));
                8
            }
            /*
             * Load value n16 into register SP.
             */
            Instruction::LD_SP_N16 { n16 } => {
                self.registers.sp = n16;
                12
            }
            /*
             * Store SP & $FF at address n16 and SP >> 8 at address n16 + 1.
             */
            Instruction::LD_N16_PNTR_SP { n16 } => {
                memory.write(n16, (self.registers.sp & 0xFF) as u8);
                memory.write(n16.wrapping_add(1), (self.registers.sp >> 8) as u8);
                20
            }
            /*
             * Add the signed value e8 to SP and store the result in HL.
             */
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
            /*
             * Load register HL into register SP.
             */
            Instruction::LD_SP_HL => {
                self.registers.sp = self.registers.get_hl();
                8
            }
            /*
             * No Operation; do nothing.
             */
            Instruction::NOP => 1,
            /*
             * Store into A the bitwise OR of the value in r8 and A.
             */
            Instruction::OR_A_R8 { r8 } => {
                self.or_a(self.registers.get_r8(&r8));
                4
            }
            /*
             * Store into A the bitwise OR of the byte pointed to by HL and A.
             */
            Instruction::OR_A_HL_PNTR => {
                self.or_a(memory.read(self.registers.get_hl()));
                8
            }
            /*
             * Store into A the bitwise OR of n8 and A.
             */
            Instruction::OR_A_N8 { n8 } => {
                self.or_a(n8);
                8
            }
            /*
             * Pop register AF from the stack.
             * This is equivalent to these following processes:
             * 1. load the value from the stack pointer into the F (flag) register; increment sp
             * 2. load the value from the stack pointer into the A (accumulator) register; increment sp
             */
            Instruction::POP_AF => {
                self.registers.f = registers::Flags::from(memory.read(self.registers.sp));
                self.registers.sp += 1;
                self.registers.a = memory.read(self.registers.sp);
                self.registers.sp += 1;
                12
            }
            /*
             * Pop 16-bit register from the stack.
             * This is equivalent to these following processes:
             * 1. load the value from the stack pointer into the lower nibble of the register; increment sp
             * 2. load the value from the stack pointer into the upper nibble of the register; increment sp
             */
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
            /*
             * Push register AF from the stack.
             * This is equivalent to these following processes:
             * 1. decrement sp; load the value from the a (accumulator) register to the stack
             * 2. decrement sp; load the value from the f (flag) register to the stack
             */
            Instruction::PUSH_AF => {
                self.registers.sp -= 1;
                let value = self.registers.a;
                self.load_u8_into_stack(memory, value);
                self.registers.sp -= 1;
                let value: u8 = registers::flags_to_u8(&self.registers.f);
                self.load_u8_into_stack(memory, value);
                16
            }
            /*
             * Push 16-bit register from the stack.
             * This is equivalent to these following processes:
             * 1. decrement sp; load the value from the lower nibble of the register to the stack
             * 2. decrement sp; load the value from the upper nibble of the register to the stack
             */
            Instruction::PUSH_R16 { r16 } => {
                self.registers.sp -= 1;
                let lower = self.registers.get_r8(&r16.get_lower());
                let upper = self.registers.get_r8(&r16.get_upper());
                self.load_u8_into_stack(memory, upper);
                self.registers.sp -= 1;
                self.load_u8_into_stack(memory, lower);
                16
            }
            /*
             * Set bit u3 in register r8 to 0. Bit 0 is the rightmost one, bit 7 the leftmost one.
             */
            Instruction::RES_U3_R8 { u3, r8 } => {
                self.res_r8(instructions::U3::get(&u3), r8);
                2
            }
            /*
             * Set bit u3 in the byte pointed by HL to 0. Bit 0 is the rightmost one, bit 7 the leftmost one.
             */
            Instruction::RES_U3_HL_PNTR { u3 } => {
                self.res_hl(memory, instructions::U3::get(&u3), self.registers.get_hl());
                4
            }
            /*
             * Return from subroutine; effectively, a POP PC instruction.
             */
            Instruction::RET => {
                self.ret(memory);
                12
            }
            /*
             * Return from subroutine if condition is met.
             */
            Instruction::RET_CC { condition } => {
                let proceed = self.check_condition(condition);
                if proceed {
                    self.ret(memory);
                    return 20;
                }
                8
            }
            /*
             * Return & enable interrupts; effectively, the same as executing EI then calling RET.
             * Note: IME is set immediately set subsequent to this instruction.
             */
            Instruction::RETI => {
                self.ime_state = ImeState::PENDING_INSTRUCTION_COMPLETION;
                self.ret(memory);
                16
            }
            /*
             * Rotate bits in register r8 left, through the carry flag.
             */
            Instruction::RL_R8 { r8 } => {
                let value = self.rotate_left_through_carry(self.registers.get_r8(&r8));
                self.registers.set_r8(&r8, value);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                8
            }
            /*
             * Rotate the byte pointed to by HL left, through the carry flag.
             */
            Instruction::RL_HL_PNTR => {
                let value = self.rotate_left_through_carry(memory.read(self.registers.get_hl()));
                memory.write(self.registers.get_hl(), value);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                16
            }
            /*
             * Rotate register A left, through the carry flag.
             */
            Instruction::RLA => {
                self.registers.a = self.rotate_left_through_carry(self.registers.a);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                4
            }
            /*
             * Rotate register r8 left.
             */
            Instruction::RLC_R8 { r8 } => {
                let value = self.rotate_left(self.registers.get_r8(&r8));
                self.registers.set_r8(&r8, value);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                8
            }
            /*
             * Rotate the byte pointed to by HL left.
             */
            Instruction::RLC_HL_PNTR => {
                let value = self.rotate_left(memory.read(self.registers.get_hl()));
                memory.write(self.registers.get_hl(), value);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                16
            }
            /*
             * Rotate register A left.
             */
            Instruction::RLCA => {
                let value = self.rotate_left(self.registers.a);
                self.registers.a = value;
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                4
            }
            /*
             * Rotate register r8 right, through the carry flag.
             */
            Instruction::RR_R8 { r8 } => {
                let value = self.rotate_right_through_carry(self.registers.get_r8(&r8));
                self.registers.set_r8(&r8, value);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                8
            }
            /*
             * Rotate the byte pointed to by HL right, through the carry flag.
             */
            Instruction::RR_HL_PNTR => {
                let value = self.rotate_right_through_carry(memory.read(self.registers.get_hl()));
                memory.write(self.registers.get_hl(), value);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                16
            }
            /*
             * Rotate register A right, through the carry flag.
             */
            Instruction::RRA => {
                self.registers.a = self.rotate_right_through_carry(self.registers.a);
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                4
            }
            /*
             * Rotate register r8 right.
             */
            Instruction::RRC_R8 { r8 } => {
                let value = self.rotate_right(self.registers.get_r8(&r8));
                self.registers.set_r8(&r8, value);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                8
            }
            /*
             * Rotate the byte pointed to by HL right.
             */
            Instruction::RRC_HL_PNTR => {
                let value = self.rotate_right(memory.read(self.registers.get_hl()));
                memory.write(self.registers.get_hl(), value);
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                16
            }
            /*
             * Rotate register A right.
             */
            Instruction::RRCA => {
                let value = self.rotate_right(self.registers.a);
                self.registers.a = value;
                self.registers.f.zero = value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                4
            }
            /*
             * Call address vec. This is a shorter and faster equivalent to CALL for suitable values of vec.
             */
            Instruction::RST { vec } => {
                self.call(memory, vec.get());
                16
            }
            /*
             * Subtract the value in r8 and the carry flag from A.
             */
            Instruction::SBC_A_R8 { r8 } => {
                let c: u8 = if self.registers.f.carry { 1 } else { 0 };
                let subtrahend = self.registers.get_r8(&r8) + c;
                let previous = self.registers.a;
                self.registers.a -= subtrahend;
                self.registers.f.carry = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (previous & 0xF) < (subtrahend & 0xF);
                self.registers.f.carry = previous < subtrahend;
                4
            }
            /*
             * Subtract the byte pointed to by HL and the carry flag from A.
             */
            Instruction::SBC_A_HL_PNTR => {
                let previous = self.registers.a;
                let c: u8 = if self.registers.f.carry { 1 } else { 0 };
                let hl = memory.read(self.registers.get_hl());
                let subtrahend = hl + c;
                self.registers.a -= subtrahend;
                self.registers.f.carry = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (previous & 0xF) < (subtrahend & 0xF);
                self.registers.f.carry = previous < subtrahend;
                8
            }
            /*
             * Subtract the value n8 and the carry flag from A.
             */
            Instruction::SBC_A_N8 { n8 } => {
                let previous = self.registers.a;
                let c: u8 = if self.registers.f.carry { 1 } else { 0 };
                let subtrahend = n8 + c;
                self.registers.a -= subtrahend;
                self.registers.f.carry = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (previous & 0xF) < (subtrahend & 0xF);
                self.registers.f.carry = previous < subtrahend;
                8
            }
            /*
             * Set Carry Flag.
             */
            Instruction::SCF {} => {
                self.registers.f.carry = true;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                4
            }
            /*
             * Set bit u3 in register r8 to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
             */
            Instruction::SET_U3_R8 { u3, r8 } => {
                let mask = 1 << (u3.get() - 1);
                self.registers
                    .set_r8(&r8, self.registers.get_r8(&r8) | mask);
                8
            }
            /*
             * Set bit u3 in the byte pointed by HL to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
             */
            Instruction::SET_U3_HL_PNTR { u3 } => {
                let mask = 1 << (u3.get() - 1);
                self.registers.set_hl(self.registers.get_hl() | mask);
                8
            }
            /*
             * Shift Left Arithmetically register r8.
             */
            Instruction::SLA_R8 { r8 } => {
                let result = self.rotate_arithmetic_left(self.registers.get_r8(&r8));
                self.registers.set_r8(&r8, result);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                8
            }
            /*
             * Shift Left Arithmetically the byte pointed to by HL.
             */
            Instruction::SLA_HL_PNTR => {
                let result = self.rotate_arithmetic_left(memory.read(self.registers.get_hl()));
                memory.write(self.registers.get_hl(), result);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                16
            }
            /*
             * Shift Right Arithmetically register r8 (bit 7 of r8 is unchanged).
             */
            Instruction::SRA_R8 { r8 } => {
                let result = self.rotate_arithmetic_right(self.registers.get_r8(&r8));
                self.registers.set_r8(&r8, result);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                8
            }
            /*
             * Shift Right Arithmetically the byte pointed to by HL (bit 7 of the byte pointed to by HL is unchanged).
             */
            Instruction::SRA_HL_PNTR => {
                let result = self.rotate_arithmetic_right(memory.read(self.registers.get_hl()));
                memory.write(self.registers.get_hl(), result);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                16
            }
            /*
             * Shift Right Logically register r8.
             */
            Instruction::SRL_R8 { r8 } => {
                let result = self.rotate_logical_right(self.registers.get_r8(&r8));
                self.registers.set_r8(&r8, result);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                8
            }
            /*
             * Shift Right Logically the byte pointed to by HL.
             */
            Instruction::SRL_HL_PNTR => {
                let result = self.rotate_logical_right(memory.read(self.registers.get_hl()));
                memory.write(self.registers.get_hl(), result);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                8
            }
            /*
             * Enter CPU very low power mode.
             */
            Instruction::STOP => {
                self.very_low_power_mode = true;
                4
            }
            /*
             * Subtract the value in r8 from A.
             */
            Instruction::SUB_A_R8 { r8 } => {
                let value = self.registers.get_r8(&r8);
                let previous = self.registers.a;
                self.registers.a -= value;
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (value & 0x10) > (previous & 0x10);
                self.registers.f.carry = value > previous;
                4
            }
            /*
             * Subtract the byte pointed to by HL from A.
             */
            Instruction::SUB_A_HL_PNTR => {
                let value = memory.read(self.registers.get_hl());
                let previous = self.registers.a;
                self.registers.a -= value;
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (value & 0x10) > (previous & 0x10);
                self.registers.f.carry = value > previous;
                4
            }
            /*
             * Subtract the value n8 from A.
             */
            Instruction::SUB_A_N8 { n8 } => {
                let previous = self.registers.a;
                self.registers.a -= n8;
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (n8 & 0x10) > (previous & 0x10);
                self.registers.f.carry = n8 > previous;
                4
            }
            /*
             * Swap the upper 4 bits in register r8 and the lower 4 ones.
             */
            Instruction::SWAP_R8 { r8 } => {
                let result = self.swap(self.registers.get_r8(&r8));
                self.registers.set_r8(&r8, result);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
                8
            }
            /*
             * Swap the upper 4 bits in the byte pointed by HL and the lower 4 ones.
             */
            Instruction::SWAP_HL_PNTR => {
                let result = self.swap(memory.read(self.registers.get_hl()));
                memory.write(self.registers.get_hl(), result);
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
                16
            }
            /*
             * Bitwise XOR between the value in r8 and A.
             */
            Instruction::XOR_A_R8 { r8 } => {
                self.registers.a = self.registers.a ^ self.registers.get_r8(&r8);
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
                4
            }
            /*
             * Bitwise XOR between the byte pointed to by HL and A.
             */
            Instruction::XOR_A_HL_PNTR => {
                self.registers.a = self.registers.a ^ memory.read(self.registers.get_hl());
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
                8
            }
            /*
             * Bitwise XOR between the value in n8 and A.
             */
            Instruction::XOR_A_N8 { n8 } => {
                self.registers.a = self.registers.a ^ n8;
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
                8
            }
        };
        if self.ime_state == ImeState::PENDING_INSTRUCTION_COMPLETION {
            // We know that we have completed the instruction proceeding the EI call.
            self.ime_state = ImeState::SET;
        } else if self.ime_state == ImeState::PENDING_NEW_INSTRUCTION {
            // EI is called but we must wait another instruction to be called (and also executed).
            self.ime_state = ImeState::PENDING_INSTRUCTION_COMPLETION;
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
    fn set_dec_r16(&mut self, r16: R16) {
        let r16_val = self.registers.get_r16(&r16);
        let r16_val_dec = r16_val - 1;
        self.registers.set_r16(&r16, r16_val_dec);
        self.registers.f.zero = r16_val_dec == 0;
        self.registers.f.subtract = true;
        // Carry from 4th bit required if it flipped
        self.registers.f.half_carry = r16_val_dec & 0x8 != r16_val & 0x8;
    }
    // todo: rewrite LD instructions to use helper functions
    fn load_u8_into_R8(&mut self, value: u8, register: R8) {
        self.registers.set_r8(&register, value);
    }
    fn load_u8_into_stack(&mut self, memory: &mut Memory, value: u8) {
        let address = self.registers.sp;
        memory.write(address, value);
    }
    fn load_address_into_reg(&mut self, memory: &mut Memory, address: u16, register: R8) {
        let value = memory.read(address);
        self.registers.set_r8(&register, value);
    }
    fn call(&mut self, memory: &mut Memory, n16: u16) {
        // push program counter to stack
        self.registers.sp -= 1;
        let lower: u8 = (self.registers.pc & 0xF) as u8;
        let upper: u8 = (self.registers.pc >> 4) as u8;
        self.load_u8_into_stack(memory, lower);
        self.registers.sp -= 1;
        self.load_u8_into_stack(memory, upper);
        self.registers.pc = n16;
    }
    fn ret(&mut self, memory: &mut Memory) {
        let lower: u8 = memory.read(self.registers.pc);
        self.registers.pc += 1;
        let upper: u8 = memory.read(self.registers.pc);
        self.registers.pc += 1;
        self.registers.set_pc(lower, upper);
    }
    fn set_ime(&mut self) {
        if self.ime_state != ImeState::PENDING_NEW_INSTRUCTION
            || self.ime_state != ImeState::PENDING_INSTRUCTION_COMPLETION
        {
            self.ime_state = ImeState::PENDING_NEW_INSTRUCTION;
        }
    }
    fn rotate_arithmetic_left(&mut self, value: u8) -> u8 {
        let b7: bool = value & 0x80 == 0;
        self.registers.f.carry = b7;
        return value << 1;
    }
    fn rotate_arithmetic_right(&mut self, value: u8) -> u8 {
        let b7: u8 = value & 0x80;
        self.registers.f.carry = value & 1 == 0;
        return (value >> 1) | b7;
    }
    fn rotate_logical_right(&mut self, value: u8) -> u8 {
        let b7: u8 = value & 0x80;
        self.registers.f.carry = value & 1 == 0;
        return (value >> 1) | b7;
    }
    fn rotate_left_through_carry(&mut self, value: u8) -> u8 {
        self.registers.f.carry = value & 0x80 == 0;
        let mut rot: u8 = value << 1;
        if self.registers.f.carry {
            rot = rot | 1;
        }
        rot
    }
    fn rotate_left(&mut self, value: u8) -> u8 {
        let b7: bool = value & 0x80 == 0;
        self.registers.f.carry = b7;
        let mut rot: u8 = value << 1;
        if b7 {
            rot = rot | 1;
        }
        rot
    }
    fn rotate_right_through_carry(&mut self, value: u8) -> u8 {
        self.registers.f.carry = value & 1 == 0;
        let mut rot: u8 = value >> 1;
        if self.registers.f.carry {
            rot = rot | 0x80;
        }
        rot
    }
    fn rotate_right(&mut self, value: u8) -> u8 {
        let b0: bool = value & 1 == 0;
        self.registers.f.carry = b0;
        let mut rot: u8 = value >> 1;
        if b0 {
            rot = rot | 0x80;
        }
        rot
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
    fn swap(&mut self, value: u8) -> u8 {
        let upper = (value >> 4) & 0xF;
        let lower = value & 0xF;
        return (lower << 4) | upper;
    }
}
