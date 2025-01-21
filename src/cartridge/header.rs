enum Licensee {
    None,
}

enum CartridgeType {
    ROMOnly,
    MBC1,
    MBC1RAM,
    MBC1RAMBattery,
    MBC2,
    MBC2Battery,
    ROMRAM,
    ROMRAMBattery,
    MMM01,
    MMM01RAM,
    MMM01RAMBattery,
    MBC3TimerBattery,
    MBC3TimerRAMBattery,
    MBC3,
    MBC3RAM,
    MBC3RAMBattery,
    MBC5,
    MBC5RAM,
    MBC5RAMBattery,
    MBC5Rumble,
    MBC5RumbleRAM,
    MBC5RumbleRAMBattery,
    MBC6,
    MBC7SensorRumbleRAMBattery,
    PocketCamera,
    BandaiTAMA5,
    HuC3,
    HuC1RAMBattery,
}

enum DestinationCode {
    Japanese,
    Overseas,
}

struct header {
    title: [u8; 16],
    manufacturer_code: [u8; 4],
    licensee: Licensee,
    cgb: bool,
    sgb: bool,
    cartridge_type: CartridgeType,
    rom_size: usize,
    num_rom_banks: usize,
    ram_size: usize,
    destination_code: DestinationCode,
    version_number: u8,
}

impl header {
    fn new(rom: &[u8; 0x150]) -> Self {
        if rom[0x14D] != checksum(rom) as u8 {
            panic!("Header checksum failed");
        }

        let mut title = [0u8; 16];
        title.copy_from_slice(&rom[0x134..0x143]);
        let mut manufacturer_code = [0u8; 4];
        manufacturer_code.copy_from_slice(&rom[0x13F..0x142]);

        let cartridge_type = match rom[0x147] {
            0x00 => CartridgeType::ROMOnly,
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1RAM,
            0x03 => CartridgeType::MBC1RAMBattery,
            0x05 => CartridgeType::MBC2,
            0x06 => CartridgeType::MBC2Battery,
            0x08 => CartridgeType::ROMRAM,
            0x09 => CartridgeType::ROMRAMBattery,
            0x0B => CartridgeType::MMM01,
            0x0C => CartridgeType::MMM01RAM,
            0x0D => CartridgeType::MMM01RAMBattery,
            0x0F => CartridgeType::MBC3TimerBattery,
            0x10 => CartridgeType::MBC3TimerRAMBattery,
            0x11 => CartridgeType::MBC3,
            0x12 => CartridgeType::MBC3RAM,
            0x13 => CartridgeType::MBC3RAMBattery,
            0x19 => CartridgeType::MBC5,
            0x1A => CartridgeType::MBC5RAM,
            0x1B => CartridgeType::MBC5RAMBattery,
            0x1C => CartridgeType::MBC5Rumble,
            0x1D => CartridgeType::MBC5RumbleRAM,
            0x1E => CartridgeType::MBC5RumbleRAMBattery,
            0x20 => CartridgeType::MBC6,
            0x22 => CartridgeType::MBC7SensorRumbleRAMBattery,
            0xFC => CartridgeType::PocketCamera,
            0xFD => CartridgeType::BandaiTAMA5,
            0xFE => CartridgeType::HuC3,
            0xFF => CartridgeType::HuC1RAMBattery,
            _ => unimplemented!(),
        };

        let rom_size = match rom[0x148] {
            0x00 => 32 * 1024,   // 32KiB
            0x01 => 64 * 1024,   // 64KiB
            0x02 => 128 * 1024,  // 128KiB
            0x03 => 256 * 1024,  // 256KiB
            0x04 => 512 * 1024,  // 512KiB
            0x05 => 1024 * 1024, // 1MiB
            0x06 => 2048 * 1024, // 2MiB
            0x07 => 4096 * 1024, // 4MiB
            0x52 => 1152 * 1024, // 1.1MiB
            0x53 => 1280 * 1024, // 1.2MiB
            0x54 => 1536 * 1024, // 1.5MiB
            _ => unimplemented!(),
        };

        let num_rom_banks = match rom[0x148] {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => unimplemented!(),
        };

        let ram_size = match rom[0x149] {
            0x00 => 0,          // None
            0x02 => 8 * 1024,   // 8KiB
            0x03 => 32 * 1024,  // 32KiB
            0x04 => 128 * 1024, // 128KiB
            0x05 => 64 * 1024,  // 64KiB
            _ => unimplemented!(),
        };

        let destination_code = match rom[0x14A] {
            0x00 => DestinationCode::Japanese,
            0x01 => DestinationCode::Overseas,
            _ => unimplemented!(),
        };

        Self {
            title,
            manufacturer_code,
            licensee: Licensee::None, // TODO
            cgb: rom[0x143] == 0x80 || rom[0x143] == 0xC0,
            sgb: rom[0x146] == 0x03,
            cartridge_type,
            rom_size,
            num_rom_banks,
            ram_size,
            destination_code,
            version_number: rom[0x14C],
        }
    }
}

fn checksum(rom: &[u8; 0x150]) -> u16 {
    let mut sum: u16 = 0;
    for byte in 0x134..0x14D {
        sum = sum.wrapping_sub(rom[byte] as u16).wrapping_sub(1);
    }
    sum
}
