use crate::io::{Joypad, Timers};

const ROM_BANK_0_START: u16 = 0x0000;
const ROM_BANK_0_END: u16 = 0x3FFF;

const ROM_BANK_N_START: u16 = 0x4000;
const ROM_BANK_N_END: u16 = 0x7FFF;

const VIDEO_RAM_START: u16 = 0x8000;
const VIDEO_RAM_END: u16 = 0x9FFF;

const EXTERNAL_RAM_START: u16 = 0xA000;
const EXTERNAL_RAM_END: u16 = 0xBFFF;

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xDFFF;

const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;

const UNUSABLE_START: u16 = 0xFEA0;
const UNUSABLE_END: u16 = 0xFEFF;

const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF7F;

const HIGH_RAM_START: u16 = 0xFF80;
const HIGH_RAM_END: u16 = 0xFFFE;

const INTERRUPT_ENABLE: u16 = 0xFFFF;

const VIDEO_RAM_SIZE: usize = (VIDEO_RAM_END - VIDEO_RAM_START + 1) as usize;
const EXTERNAL_RAM_SIZE: usize = (EXTERNAL_RAM_END - EXTERNAL_RAM_START + 1) as usize;
const WORK_RAM_SIZE: usize = (WORK_RAM_END - WORK_RAM_START + 1) as usize;
const OAM_SIZE: usize = (OAM_END - OAM_START + 1) as usize;
const HIGH_RAM_SIZE: usize = (HIGH_RAM_END - HIGH_RAM_START + 1) as usize;

pub struct Memory {
    rom: Vec<u8>,
    video_ram: [u8; VIDEO_RAM_SIZE],
    ext_ram: [u8; EXTERNAL_RAM_SIZE],
    work_ram: [u8; WORK_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    high_ram: [u8; HIGH_RAM_SIZE],

    joypad: Joypad,
    timers: Timers,
    interrupt_flag: u8,
    interrupt_enable: u8,
}

impl Memory {
    pub(crate) fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            video_ram: [0; VIDEO_RAM_SIZE],
            ext_ram: [0; EXTERNAL_RAM_SIZE],
            work_ram: [0; WORK_RAM_SIZE],
            oam: [0; OAM_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            joypad: Joypad::new(),
            timers: Timers::new(),
            interrupt_flag: 0,
            interrupt_enable: 0,
        }
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        match address {
            ROM_BANK_0_START..=ROM_BANK_N_END => self.rom[address as usize],
            VIDEO_RAM_START..=VIDEO_RAM_END => self.video_ram[(address - VIDEO_RAM_START) as usize],
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
                self.ext_ram[(address - EXTERNAL_RAM_START) as usize]
            }
            WORK_RAM_START..=WORK_RAM_END => self.work_ram[(address - WORK_RAM_START) as usize],
            ECHO_RAM_START..=ECHO_RAM_END => self.work_ram[(address - ECHO_RAM_START) as usize],
            OAM_START..=OAM_END => self.oam[(address - OAM_START) as usize],
            UNUSABLE_START..=UNUSABLE_END => 0x00, // TODO
            IO_START..=IO_END => self.read_io(address),
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram[(address - HIGH_RAM_START) as usize],
            INTERRUPT_ENABLE => self.interrupt_enable,
        }
    }

    pub(crate) fn write(&mut self, address: u16, byte: u8) {
        match address {
            ROM_BANK_0_START..=ROM_BANK_N_END => self.rom[address as usize] = byte,
            VIDEO_RAM_START..=VIDEO_RAM_END => {
                self.video_ram[(address - VIDEO_RAM_START) as usize] = byte
            }
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => {
                self.ext_ram[(address - EXTERNAL_RAM_START) as usize] = byte
            }
            WORK_RAM_START..=WORK_RAM_END => {
                self.work_ram[(address - WORK_RAM_START) as usize] = byte
            }
            ECHO_RAM_START..=ECHO_RAM_END => {
                self.work_ram[(address - ECHO_RAM_START) as usize] = byte
            }
            OAM_START..=OAM_END => self.oam[(address - OAM_START) as usize] = byte,
            UNUSABLE_START..=UNUSABLE_END => {} // TODO
            IO_START..=IO_END => self.write_io(address, byte),
            HIGH_RAM_START..=HIGH_RAM_END => {
                self.high_ram[(address - HIGH_RAM_START) as usize] = byte
            }
            INTERRUPT_ENABLE => self.interrupt_enable = byte,
        };
    }

    fn read_io(&self, address: u16) -> u8 {
        match address {
            0x0000..0xFF00 | 0xFF80..=0xFFFF => unreachable!(),
            0xFF00 => self.joypad.get(),
            0xFF01 => todo!(),
            0xFF02 => todo!(),
            0xFF03 => unimplemented!(),
            0xFF04 => self.timers.get_divider(),
            0xFF05 => self.timers.get_counter(),
            0xFF06 => self.timers.get_tma(),
            0xFF07 => self.timers.get_tac(),
            0xFF08..=0xFF0E => unimplemented!(),
            0xFF0F => self.interrupt_flag,
            0xFF10..=0xFF3F => todo!(), // Sound
            0xFF4F..=0xFF77 => 0x0,     // CGB only
        }
    }

    fn write_io(&self, address: u16, byte: u8) {
        match address {
            0x0000..0xFF00 | 0xFF80..=0xFFFF => unreachable!(),
            0xFF00 => self.joypad.set(byte),
            0xFF01 => todo!(),
            0xFF02 => todo!(),
            0xFF03 => unimplemented!(),
            0xFF04 => self.timers.reset_div(),
            0xFF05 => self.timers.set_counter(byte),
            0xFF06 => self.timers.set_tma(byte),
            0xFF07 => self.timers.set_tac(byte),
            0xFF08..=0xFF0E => unimplemented!(),
            0xFF0F => self.interrupt_flag = byte,
            0xFF10..=0xFF3F => todo!(), // Sound
            0xFF4F..=0xFF77 => {}       // CGB only
        };
    }
}
