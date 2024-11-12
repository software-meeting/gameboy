use super::{R16, R8};

pub(crate) struct Flags {
    pub(crate) zero: bool,
    pub(crate) subtract: bool,
    pub(crate) half_carry: bool,
    pub(crate) carry: bool,
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            zero: value & 0b10000000 != 0,
            subtract: value & 0b01000000 != 0,
            half_carry: value & 0b00100000 != 0,
            carry: value & 0b00010000 != 0,
        }
    }
}

impl From<&Flags> for u8 {
    fn from(value: &Flags) -> Self {
        ((if value.zero { 1 } else { 0 }) << 7)
            | ((if value.subtract { 1 } else { 0 }) << 6)
            | ((if value.half_carry { 1 } else { 0 }) << 5)
            | ((if value.carry { 1 } else { 0 }) << 4)
    }
}

pub(crate) fn flags_to_u8(flags: &Flags) -> u8 {
    return ((if flags.zero { 1 } else { 0 }) << 7)
        | ((if flags.subtract { 1 } else { 0 }) << 6)
        | ((if flags.half_carry { 1 } else { 0 }) << 5)
        | ((if flags.carry { 1 } else { 0 }) << 4);
}
pub(crate) struct Registers {
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) f: Flags,
    pub(crate) h: u8,
    pub(crate) l: u8,
    pub(crate) sp: u16,
    pub(crate) pc: u16,
}

impl Registers {
    pub(crate) fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: Flags {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) & u8::from(&self.f) as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from((value & 0xFF) as u8);
    }
    pub(crate) fn set_pc(&mut self, lower: u8, upper: u8) {
        self.pc = ((upper << 4) | lower) as u16;
    }

    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) & self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) & self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub(crate) fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) & self.l as u16
    }

    pub(crate) fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub(crate) fn get_r8(&self, register: &R8) -> u8 {
        match register {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
        }
    }

    pub(crate) fn set_r8(&mut self, register: &R8, value: u8) {
        match register {
            R8::A => self.a = value,
            R8::B => self.b = value,
            R8::C => self.c = value,
            R8::D => self.d = value,
            R8::E => self.e = value,
            R8::H => self.h = value,
            R8::L => self.l = value,
        }
    }
    pub(crate) fn set_r16(&mut self, register: &R16, value: u16) {
        match register {
            R16::BC => self.set_bc(value),
            R16::DE => self.set_de(value),
            R16::HL => self.set_hl(value),
        }
    }
    pub(crate) fn get_r16(&self, register: &R16) -> u16 {
        match register {
            R16::BC => self.get_bc(),
            R16::DE => self.get_de(),
            R16::HL => self.get_hl(),
        }
    }
}
