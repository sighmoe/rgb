use super::ppu::Ppu;
use std::fs;

pub struct MemoryMap {
    contents: [u8; 65535],
    ppu: Ppu,
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            contents: [0; 65535],
            ppu: Ppu::new(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // PPU Registers (0xFF40-0xFF4B)
            0xFF40..=0xFF4B => {
                self.ppu.write_register(addr, val);
            }
            // VRAM (0x8000-0x9FFF)
            0x8000..=0x9FFF => {
                self.ppu.write_vram(addr, val);
            }
            // OAM (0xFE00-0xFE9F)
            0xFE00..=0xFE9F => {
                self.ppu.write_oam(addr, val);
            }
            // Regular memory
            _ => {
                self.contents[addr as usize] = val;
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // PPU Registers (0xFF40-0xFF4B)
            0xFF40..=0xFF4B => self.ppu.read_register(addr),
            // VRAM (0x8000-0x9FFF)
            0x8000..=0x9FFF => self.ppu.read_vram(addr),
            // OAM (0xFE00-0xFE9F)
            0xFE00..=0xFE9F => self.ppu.read_oam(addr),
            // Regular memory
            _ => self.contents[addr as usize],
        }
    }

    pub fn load_bootstrap(&mut self) {
        let buf = fs::read("test-roms/dmg_boot.bin")
            .expect("expected valid file path for bootstrap rom");
        assert_eq!(
            buf.len(),
            256,
            "expected bootstrap rom to be exactly 256 bytes"
        );

        self.contents[0..(255 + 1)].copy_from_slice(&buf);
    }

    pub fn get_ppu(&self) -> &Ppu {
        &self.ppu
    }

    pub fn get_ppu_mut(&mut self) -> &mut Ppu {
        &mut self.ppu
    }

    pub fn step_ppu(&mut self, cycles: u16) {
        self.ppu.step(cycles);
    }
}
