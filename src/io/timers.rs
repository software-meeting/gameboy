pub(crate) struct Timers {
    divider: u8,
    internal_divider: usize,
    timer_counter: u8,
    timer_modulo: u8,
    internal_timer: usize,
}

impl Timers {
    pub(crate) fn new() -> Self {
        Self {
            divider: 0,
            internal_divider: 0,
            timer_counter: 0,
            timer_modulo: 0,
            internal_timer: 0,
        }
    }

    pub(crate) fn tick(&mut self, ticks: usize) {
        self.internal_divider += ticks;
        self.internal_timer += ticks;

        while self.internal_divider >= 256 {
            self.divider.wrapping_add(1);
            self.internal_divider -= 256;
        }
    }

    pub(crate) fn get_divider(&self) -> u8 {
        self.divider
    }
}
