pub(crate) struct Timers {
    divider: u8,
    internal_divider: usize,
    timer_counter: u8,
    timer_modulo: u8,
    internal_timer: usize,
    timer_control: u8,
}

impl Timers {
    pub(crate) fn new() -> Self {
        Self {
            divider: 0,
            internal_divider: 0,
            timer_counter: 0,
            timer_modulo: 0,
            internal_timer: 0,
            timer_control: 0,
        }
    }

    pub(crate) fn tick(&mut self, ticks: usize) -> bool {
        let mut interrupt = false;
        self.internal_divider += ticks;

        while self.internal_divider >= 256 {
            self.divider.wrapping_add(1);
            self.internal_divider -= 256;
        }

        if self.timer_control & 0b100 == 0b100 {
            self.internal_timer += ticks;

            let timer_period = match self.timer_control & 0b11 {
                0b00 => 1024,
                0b01 => 16,
                0b10 => 64,
                0b11 => 256,
                _ => unreachable!(),
            };
            while self.internal_timer >= timer_period {
                self.timer_counter = self.timer_counter.wrapping_add(1);
                self.internal_timer -= timer_period;

                if self.timer_counter == 0 {
                    self.timer_counter = self.timer_modulo;
                    interrupt = true;
                }
            }
        }

        interrupt
    }

    pub(crate) fn reset_div(&mut self) {
        self.divider = 0;
    }

    pub(crate) fn get_divider(&self) -> u8 {
        self.divider
    }

    pub(crate) fn set_counter(&mut self, byte: u8) {
        self.timer_counter = byte;
    }

    pub(crate) fn get_counter(&self) -> u8 {
        self.timer_counter
    }

    pub(crate) fn set_tma(&mut self, byte: u8) {
        self.timer_modulo = byte;
    }

    pub(crate) fn get_tma(&self) -> u8 {
        self.timer_modulo
    }

    pub(crate) fn set_tac(&mut self, byte: u8) {
        self.timer_control = byte;
    }

    pub(crate) fn get_tac(&self) -> u8 {
        self.timer_control
    }
}
