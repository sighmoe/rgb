pub struct Timer {
    div_counter: u16,   // Internal 16-bit counter (DIV register is upper 8 bits)
    tima: u8,           // Timer counter (0xFF05)
    tma: u8,            // Timer modulo (0xFF06)
    tac: u8,            // Timer control (0xFF07)
    timer_cycles: u16,  // Cycle accumulator for TIMA register
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div_counter: 0,
            tima: 0,
            tma: 0,
            tac: 0,       // Timer disabled on reset
            timer_cycles: 0,
        }
    }
    
    /// Creates a new Timer in the post-boot state for skipping boot sequence
    /// Initializes timer registers to their expected values after boot ROM completion
    pub fn new_post_boot() -> Self {
        Self {
            div_counter: 0xABCC, // DIV register starts with some value after boot
            tima: 0,
            tma: 0,
            tac: 0,       // Timer disabled after boot
            timer_cycles: 0,
        }
    }

    pub fn step(&mut self, cycles: u16) -> bool {
        let mut timer_interrupt = false;

        // Update internal DIV counter - increments every T-cycle
        // Game Boy cycles are passed as T-cycles, so increment directly
        self.div_counter = self.div_counter.wrapping_add(cycles);

        // Update TIMA register if timer is enabled
        if self.is_timer_enabled() {
            self.timer_cycles += cycles;
            let timer_frequency = self.get_timer_frequency();
            
            while self.timer_cycles >= timer_frequency {
                self.timer_cycles -= timer_frequency;
                
                // Increment TIMA
                if self.tima == 0xFF {
                    // TIMA overflow - reload with TMA and request timer interrupt
                    self.tima = self.tma;
                    timer_interrupt = true;
                } else {
                    self.tima = self.tima.wrapping_add(1);
                }
            }
        }

        timer_interrupt
    }

    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => (self.div_counter >> 8) as u8,  // DIV - upper 8 bits of internal counter
            0xFF05 => self.tima,                      // TIMA
            0xFF06 => self.tma,                       // TMA
            0xFF07 => self.tac | 0xF8,                // TAC with unused bits set
            _ => 0xFF,
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        #[cfg(debug_assertions)]
        {
            use log::debug;
            static mut TIMER_WRITE_COUNT: u32 = 0;
            unsafe {
                TIMER_WRITE_COUNT += 1;
                if TIMER_WRITE_COUNT <= 20 {
                    debug!("Timer write: addr=0x{:04X}, value=0x{:02X}", addr, value);
                }
            }
        }
        
        match addr {
            0xFF04 => {
                // Writing to DIV resets the internal counter to 0
                self.div_counter = 0;
            }
            0xFF05 => {
                self.tima = value;
            }
            0xFF06 => {
                self.tma = value;
            }
            0xFF07 => {
                self.tac = value & 0x07; // Only lower 3 bits are used
                // Note: Real Game Boy behavior on TAC write is complex and can affect timer state
                // For now, keep timer_cycles to maintain current timing
            }
            _ => {}
        }
    }

    fn is_timer_enabled(&self) -> bool {
        (self.tac & 0x04) != 0
    }

    fn get_timer_frequency(&self) -> u16 {
        match self.tac & 0x03 {
            0 => 1024, // 4096 Hz (CPU_FREQ / 4096 = ~1024 cycles)
            1 => 16,   // 262144 Hz (CPU_FREQ / 262144 = ~16 cycles)
            2 => 64,   // 65536 Hz (CPU_FREQ / 65536 = ~64 cycles)  
            3 => 256,  // 16384 Hz (CPU_FREQ / 16384 = ~256 cycles)
            _ => unreachable!(),
        }
    }
}