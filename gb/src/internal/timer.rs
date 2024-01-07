pub struct Timer {
    pub sysclock: u16,
    pub tma: u8,
    pub tma_previous: Option<u8>,
    pub tima: u8,
    pub tac: u8,
    pub tima_irq: usize,

    sysclock_cycles: usize,
    current_freq: u16
}

impl Timer {
    pub fn update(&mut self) {
        self.sysclock_cycles += 4;

        if (self.tac >> 2 & 0x1) == 1 {
            let bit_set_prev = self.current_freq;

            self.current_freq = match self.tac & 0x3 {
                0 => (self.sysclock >> 9) & 0x1, // 1024 (default)
                1 => (self.sysclock >> 3) & 0x1, // 16
                2 => (self.sysclock >> 5) & 0x1, // 64
                3 => (self.sysclock >> 7) & 0x1, // 256
                _ => panic!()
            };

            if bit_set_prev == 1 && self.current_freq == 0 {
                let result = self.tima.overflowing_add(1);
                if result.1 {
                    self.tima = if self.tma_previous.is_none() { self.tma } else { self.tma_previous.unwrap() };
                    self.tima_irq = 2;
                } else {
                    self.tima = result.0;
                }
            }
        }

        if self.sysclock_cycles > 0xFF {
            self.sysclock += 1;
            self.sysclock_cycles = 0;
        }
        self.tma_previous = None;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self { 
            sysclock: 0x0,
            tma: 0,
            tma_previous: None,
            tima: 0xFF,
            tac: 0x0,
            tima_irq: 0,

            sysclock_cycles: 0,
            current_freq: 0
        }
    }
}