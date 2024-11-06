const ROM_BANK_0_START: usize = 0x0000;
const ROM_BANK_0_END: usize = 0x3FFF;

const ROM_BANK_N_START: usize = 0x4000;
const ROM_BANK_N_END: usize = 0x7FFF;

const VIDEO_RAM_START: usize = 0x8000;
const VIDEO_RAM_END: usize = 0x9FFF;

const EXTERNAL_RAM_START: usize = 0xA000;
const EXTERNAL_RAM_END: usize = 0xBFFF;

const WORK_RAM_START: usize = 0xC000;
const WORK_RAM_END: usize = 0xDFFF;

const ECHO_RAM_START: usize = 0xE000;
const ECHO_RAM_END: usize = 0xFDFF;

const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFE00;

const UNUSABLE_START: usize = 0xFEA0;
const UNUSABLE_END: usize = 0xFEFF;

const IO_START: usize = 0xFF00;
const IO_END: usize = 0xFF7F;

const HIGH_RAM_START: usize = 0xFF80;
const HIGH_RAM_END: usize = 0xFFFE;

const INTERUPT_ENABLE: usize = 0xFFFF;

pub struct Memory {
    data: [u8; INTERUPT_ENABLE],
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self {
            data: [0; INTERUPT_ENABLE],
        }
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub(crate) fn write(&mut self, address: u16, byte: u8) {
        self.data[address as usize] = byte;
    }
}
