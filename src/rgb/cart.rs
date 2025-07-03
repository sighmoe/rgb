use std::fs;
use std::path::Path;
#[cfg(debug_assertions)]
use log::debug;

#[derive(Debug, Clone, Copy)]
pub enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
}

impl CartridgeType {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(CartridgeType::RomOnly),
            0x01 => Some(CartridgeType::Mbc1),
            0x02 => Some(CartridgeType::Mbc1Ram),
            0x03 => Some(CartridgeType::Mbc1RamBattery),
            0x0F => Some(CartridgeType::Mbc3TimerBattery),
            0x10 => Some(CartridgeType::Mbc3TimerRamBattery),
            0x11 => Some(CartridgeType::Mbc3),
            0x12 => Some(CartridgeType::Mbc3Ram),
            0x13 => Some(CartridgeType::Mbc3RamBattery),
            _ => None,
        }
    }

    pub fn has_ram(&self) -> bool {
        matches!(self, 
            CartridgeType::Mbc1Ram | CartridgeType::Mbc1RamBattery |
            CartridgeType::Mbc3TimerRamBattery | CartridgeType::Mbc3Ram | 
            CartridgeType::Mbc3RamBattery
        )
    }

    pub fn has_timer(&self) -> bool {
        matches!(self, 
            CartridgeType::Mbc3TimerBattery | CartridgeType::Mbc3TimerRamBattery
        )
    }
}

#[derive(Debug)]
pub struct Cart {
    rom: Vec<u8>,
    ram: Vec<u8>,
    cartridge_type: CartridgeType,
    
    // MBC3 state
    rom_bank: u8,     // Current ROM bank (1-127)
    ram_bank: u8,     // Current RAM bank (0-3) or RTC register (0x08-0x0C)
    ram_rtc_enable: bool, // RAM/RTC access enable
    
    // RTC state (simplified - no actual time tracking for now)
    rtc_registers: [u8; 5], // S, M, H, DL, DH
}

impl Cart {
    pub fn new(path: &Path) -> Self {
        let buf = fs::read(path).unwrap_or_else(|e| {
            eprintln!("Failed to load ROM from path: {:?}", path);
            eprintln!("Current working directory: {:?}", std::env::current_dir().unwrap_or_default());
            eprintln!("Error: {}", e);
            panic!("expected valid file path: {}", e);
        });
        
        // Read cartridge type from header
        let cartridge_type = if buf.len() > 0x0147 {
            CartridgeType::from_byte(buf[0x0147]).unwrap_or(CartridgeType::RomOnly)
        } else {
            CartridgeType::RomOnly
        };
        
        // Determine RAM size from header
        let ram_size = if buf.len() > 0x0149 {
            match buf[0x0149] {
                0x00 => 0,      // No RAM
                0x02 => 8192,   // 8KB
                0x03 => 32768,  // 32KB (4 banks of 8KB)
                0x04 => 131072, // 128KB (16 banks of 8KB)
                0x05 => 65536,  // 64KB (8 banks of 8KB)
                _ => 0,
            }
        } else {
            0
        };
        
        #[cfg(debug_assertions)]
        {
            debug!("Loaded ROM: {} bytes", buf.len());
            debug!("Cartridge type: {:?}", cartridge_type);
            debug!("RAM size: {} bytes", ram_size);
        }
        
        Cart { 
            rom: buf,
            ram: vec![0; ram_size],
            cartridge_type,
            rom_bank: 1,           // MBC3 starts with ROM bank 1
            ram_bank: 0,           // Start with RAM bank 0
            ram_rtc_enable: false, // RAM/RTC access disabled by default
            rtc_registers: [0; 5], // Initialize RTC registers to 0
        }
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // ROM Bank 0 (0x0000-0x3FFF) - always accessible
            0x0000..=0x3FFF => {
                if (addr as usize) < self.rom.len() {
                    self.rom[addr as usize]
                } else {
                    0xFF // Return 0xFF for reads beyond ROM size
                }
            }
            // ROM Bank 1+ (0x4000-0x7FFF) - switchable bank
            0x4000..=0x7FFF => {
                let bank_offset = addr - 0x4000;
                let rom_addr = (self.rom_bank as usize * 0x4000) + bank_offset as usize;
                
                #[cfg(debug_assertions)]
                {
                    static mut ROM_READ_COUNT: u32 = 0;
                    unsafe {
                        ROM_READ_COUNT += 1;
                        if ROM_READ_COUNT <= 20 && addr == 0x4000 {
                            println!("Reading from ROM bank {} at 0x{:04X} -> ROM addr 0x{:06X}", 
                                self.rom_bank, addr, rom_addr);
                        }
                    }
                }
                
                if rom_addr < self.rom.len() {
                    self.rom[rom_addr]
                } else {
                    #[cfg(debug_assertions)]
                    println!("ROM read beyond bounds: bank {} addr 0x{:04X} -> ROM addr 0x{:06X} (ROM size: 0x{:06X})", 
                        self.rom_bank, addr, rom_addr, self.rom.len());
                    0xFF
                }
            }
            _ => 0xFF // Invalid addresses for ROM reads
        }
    }
    
    pub fn read_ram(&self, addr: u16) -> u8 {
        if !self.ram_rtc_enable {
            return 0xFF; // RAM/RTC access disabled
        }
        
        match addr {
            0xA000..=0xBFFF => {
                match self.ram_bank {
                    // RAM banks 0-3
                    0x00..=0x03 => {
                        if self.cartridge_type.has_ram() && !self.ram.is_empty() {
                            let ram_offset = addr - 0xA000;
                            let ram_addr = (self.ram_bank as usize * 0x2000) + ram_offset as usize;
                            if ram_addr < self.ram.len() {
                                self.ram[ram_addr]
                            } else {
                                0xFF
                            }
                        } else {
                            0xFF
                        }
                    }
                    // RTC registers (0x08-0x0C map to indices 0-4)
                    0x08..=0x0C => {
                        if self.cartridge_type.has_timer() {
                            let rtc_index = (self.ram_bank - 0x08) as usize;
                            self.rtc_registers[rtc_index]
                        } else {
                            0xFF
                        }
                    }
                    _ => 0xFF
                }
            }
            _ => 0xFF
        }
    }
    
    pub fn write(&mut self, addr: u16, value: u8) {
        // Handle Memory Bank Controller (MBC3) writes
        match addr {
            0x0000..=0x1FFF => {
                // RAM and Timer Enable (write 0x0A to enable, anything else to disable)
                self.ram_rtc_enable = value == 0x0A;
                #[cfg(debug_assertions)]
                if value == 0x0A {
                    debug!("MBC3: RAM/RTC enabled");
                } else {
                    debug!("MBC3: RAM/RTC disabled");
                }
            }
            0x2000..=0x3FFF => {
                // ROM Bank Number (1-127)
                let bank = if value == 0 { 1 } else { value & 0x7F }; // Banks 1-127, 0 becomes 1
                self.rom_bank = bank;
                #[cfg(debug_assertions)]
                {
                    static mut BANK_SWITCH_COUNT: u32 = 0;
                    unsafe {
                        BANK_SWITCH_COUNT += 1;
                        if BANK_SWITCH_COUNT <= 10 {
                            println!("MBC3: ROM bank switched to {} (attempt #{})", bank, BANK_SWITCH_COUNT);
                        }
                    }
                }
            }
            0x4000..=0x5FFF => {
                // RAM Bank Number (0x00-0x03) or RTC Register Select (0x08-0x0C)
                match value {
                    0x00..=0x03 => {
                        self.ram_bank = value;
                        #[cfg(debug_assertions)]
                        debug!("MBC3: RAM bank switched to {}", value);
                    }
                    0x08..=0x0C => {
                        self.ram_bank = value; // RTC register select
                        #[cfg(debug_assertions)]
                        debug!("MBC3: RTC register {} selected", value);
                    }
                    _ => {
                        // Invalid bank number - ignore
                        #[cfg(debug_assertions)]
                        debug!("MBC3: Invalid RAM/RTC bank number: 0x{:02X}", value);
                    }
                }
            }
            0x6000..=0x7FFF => {
                // Latch Clock Data (write 0x00 then 0x01 to latch RTC)
                // For now, we'll just ignore this since we don't implement real RTC
                #[cfg(debug_assertions)]
                debug!("MBC3: Clock latch write: 0x{:02X} (ignored)", value);
            }
            _ => {}
        }
    }
    
    pub fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.ram_rtc_enable {
            return; // RAM/RTC access disabled
        }
        
        match addr {
            0xA000..=0xBFFF => {
                match self.ram_bank {
                    // RAM banks 0-3
                    0x00..=0x03 => {
                        if self.cartridge_type.has_ram() && !self.ram.is_empty() {
                            let ram_offset = addr - 0xA000;
                            let ram_addr = (self.ram_bank as usize * 0x2000) + ram_offset as usize;
                            if ram_addr < self.ram.len() {
                                self.ram[ram_addr] = value;
                            }
                        }
                    }
                    // RTC registers (0x08-0x0C map to indices 0-4)
                    0x08..=0x0C => {
                        if self.cartridge_type.has_timer() {
                            let rtc_index = (self.ram_bank - 0x08) as usize;
                            self.rtc_registers[rtc_index] = value;
                            #[cfg(debug_assertions)]
                            debug!("MBC3: RTC register {} = 0x{:02X}", rtc_index, value);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    pub fn get_title(&self) -> String {
        if self.rom.len() >= 0x0143 {
            let title_bytes = &self.rom[0x0134..=0x0142];
            String::from_utf8_lossy(title_bytes).trim_end_matches('\0').to_string()
        } else {
            "Unknown".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
