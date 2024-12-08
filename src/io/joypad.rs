pub(crate) struct Joypad {
    data: u8,
}

impl Joypad {
    pub(crate) fn new() -> Self {
        Self { data: 0x0F }
    }

    pub(crate) fn get(&self) -> u8 {
        self.data | 0xF
    }

    pub(crate) fn set(&mut self, data: u8) {
        self.data = data & 0xF0;
    }
}
