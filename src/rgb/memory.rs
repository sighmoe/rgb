use super::ppu::Ppu;
use super::cart::Cart;
use std::fs;
use std::path::Path;
use log::debug;

pub struct MemoryMap {
    contents: [u8; 65536],
    ppu: Ppu,
    cart: Option<Cart>,
    pub bootstrap_enabled: bool,
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            contents: [0; 65536],
            ppu: Ppu::new(),
            cart: None,
            bootstrap_enabled: true,
        }
    }
    
    pub fn load_cartridge(&mut self, path: &Path) {
        let cart = Cart::new(path);
        println!("Cartridge loaded: {}", cart.get_title());
        self.cart = Some(cart);
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // Cartridge ROM area (0x0000-0x7FFF) - handle MBC writes
            0x0000..=0x7FFF => {
                if let Some(ref mut cart) = self.cart {
                    cart.write(addr, val);
                }
            }
            // PPU Registers (0xFF40-0xFF4B)
            0xFF40..=0xFF4B => {
                self.ppu.write_register(addr, val);
            }
            // VRAM (0x8000-0x9FFF)
            0x8000..=0x9FFF => {
                // Debug tile map writes for Nintendo logo area (0x9800 + background area)
                if addr >= 0x9800 && addr <= 0x9BFF && val != 0x00 {
                    let tile_x = (addr - 0x9800) % 32;
                    let tile_y = (addr - 0x9800) / 32;
                    static mut TILEMAP_WRITE_COUNT: u32 = 0;
                    unsafe {
                        TILEMAP_WRITE_COUNT += 1;
                        if TILEMAP_WRITE_COUNT <= 25 {
                            debug!("TileMap write: addr=0x{:04X}, tile_pos=({},{}), tile_id=0x{:02X}", 
                                addr, tile_x, tile_y, val);
                        }
                    }
                }
                self.ppu.write_vram(addr, val);
            }
            // OAM (0xFE00-0xFE9F)
            0xFE00..=0xFE9F => {
                self.ppu.write_oam(addr, val);
            }
            // Bootstrap disable register
            0xFF50 => {
                if val != 0 {
                    self.bootstrap_enabled = false;
                    debug!("Bootstrap ROM disabled, switching to cartridge");
                }
            }
            // Interrupt registers
            0xFF0F => {
                // IF register - Interrupt Flag
                self.contents[addr as usize] = val | 0xE0; // Upper 3 bits always set
            }
            0xFFFF => {
                // IE register - Interrupt Enable
                self.contents[addr as usize] = val;
            }
            // Regular memory
            _ => {
                self.contents[addr as usize] = val;
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let result = match addr {
            // Bootstrap ROM area (0x0000-0x00FF) - only accessible when bootstrap is enabled
            0x0000..=0x00FF if self.bootstrap_enabled => {
                self.contents[addr as usize]
            }
            // Cartridge ROM area (0x0000-0x7FFF)
            0x0000..=0x7FFF => {
                if let Some(ref cart) = self.cart {
                    cart.read(addr)
                } else {
                    // Read from fake cartridge data if no cart loaded
                    self.contents[addr as usize]
                }
            }
            // PPU Registers (0xFF40-0xFF4B)
            0xFF40..=0xFF4B => self.ppu.read_register(addr),
            // VRAM (0x8000-0x9FFF)
            0x8000..=0x9FFF => self.ppu.read_vram(addr),
            // OAM (0xFE00-0xFE9F)
            0xFE00..=0xFE9F => self.ppu.read_oam(addr),
            // Interrupt registers
            0xFF0F => {
                // IF register - return with upper 3 bits set
                self.contents[addr as usize] | 0xE0
            }
            0xFFFF => {
                // IE register
                self.contents[addr as usize]
            }
            // Regular memory
            _ => self.contents[addr as usize],
        };
        
        // Debug Nintendo logo reads during bootstrap
        if addr >= 0x0104 && addr <= 0x0133 && self.bootstrap_enabled {
            static mut LOGO_READ_COUNT: u32 = 0;
            unsafe {
                LOGO_READ_COUNT += 1;
                if LOGO_READ_COUNT <= 20 {
                    debug!("Logo read: addr=0x{:04X}, value=0x{:02X}", addr, result);
                }
            }
        }
        
        result
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
        
        // Load fake cartridge header with Nintendo logo so bootstrap ROM has something to display
        self.load_fake_cartridge_header();
    }
    
    fn load_fake_cartridge_header(&mut self) {
        // Only load fake header if no cartridge is loaded
        if self.cart.is_none() {
            debug!("Loading fake Nintendo logo data for bootstrap ROM");
            // Nintendo logo data that the bootstrap ROM expects at 0x0104-0x0133
            // This is the compressed logo data from a real Game Boy cartridge
            let nintendo_logo: [u8; 48] = [
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
            ];
            
            // Copy logo data to cartridge header location
            for (i, &byte) in nintendo_logo.iter().enumerate() {
                self.contents[0x0104 + i] = byte;
            }
            
            // Set header checksum (required for bootstrap ROM to pass)
            self.contents[0x014D] = 0x00; // Header checksum placeholder
            
            // Debug: Print first few bytes of logo data
            debug!("First 16 Nintendo logo bytes: {:02X?}", &nintendo_logo[0..16]);
        } else {
            debug!("Using Nintendo logo from loaded cartridge");
        }
    }

    pub fn get_ppu(&self) -> &Ppu {
        &self.ppu
    }

    #[allow(dead_code)] // Public API method
    pub fn get_ppu_mut(&mut self) -> &mut Ppu {
        &mut self.ppu
    }

    pub fn step_ppu(&mut self, cycles: u16) {
        self.ppu.step(cycles);
    }
}
