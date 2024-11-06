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

pub(crate) struct Registers {
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    d: u8,
    e: u8,
    pub(crate) f: Flags,
    h: u8,
    l: u8,
    pub(crate) sp: u16,
    pc: u16,
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

    pub(crate) fn get_r8(&self, register: R8) -> u8 {
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

    pub(crate) fn get_r16(&self, register: R16) -> u16 {
        match register {
            R16::BC => self.get_bc(),
            R16::DE => self.get_de(),
            R16::HL => self.get_hl(),
        }
    }
}
