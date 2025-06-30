use std::fs;
use std::path::Path;
use log::debug;

#[derive(Debug)]
pub struct Cart {
    rom: Vec<u8>,
}

impl Cart {
    pub fn new(path: &Path) -> Self {
        let buf = fs::read(path).unwrap_or_else(|e| {
            eprintln!("Failed to load ROM from path: {:?}", path);
            eprintln!("Current working directory: {:?}", std::env::current_dir().unwrap_or_default());
            eprintln!("Error: {}", e);
            panic!("expected valid file path: {}", e);
        });
        debug!("Loaded ROM: {} bytes", buf.len());
        Cart { rom: buf }
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
                let rom_addr = 0x4000 + bank_offset; // For now, just use bank 1
                if (rom_addr as usize) < self.rom.len() {
                    self.rom[rom_addr as usize]
                } else {
                    0xFF
                }
            }
            _ => 0xFF // Invalid addresses
        }
    }
    
    pub fn write(&mut self, addr: u16, _value: u8) {
        // Handle Memory Bank Controller (MBC) writes
        match addr {
            0x0000..=0x1FFF => {
                // RAM Enable area - not implemented yet
            }
            0x2000..=0x3FFF => {
                // ROM Bank Number - not implemented yet (would handle bank switching)
            }
            0x4000..=0x5FFF => {
                // RAM Bank Number - not implemented yet
            }
            0x6000..=0x7FFF => {
                // Banking Mode Select - not implemented yet
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
