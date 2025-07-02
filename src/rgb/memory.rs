use super::ppu::Ppu;
use super::cart::Cart;
use super::timer::Timer;
use super::joypad::Joypad;
use std::fs;
use std::path::Path;
#[cfg(debug_assertions)]
use log::debug;

pub struct MemoryMap {
    contents: [u8; 65536],
    ppu: Ppu,
    timer: Timer,
    joypad: Joypad,
    cart: Option<Cart>,
    pub bootstrap_enabled: bool,
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            contents: [0; 65536],
            ppu: Ppu::new(),
            timer: Timer::new(),
            joypad: Joypad::new(),
            cart: None,
            bootstrap_enabled: true,
        }
    }
    
    /// Creates a new MemoryMap in the post-boot state for skipping boot sequence
    /// This initializes hardware registers to their expected post-boot values
    pub fn new_post_boot() -> Self {
        let mut mmap = MemoryMap {
            contents: [0; 65536],
            ppu: Ppu::new_post_boot(),
            timer: Timer::new_post_boot(),
            joypad: Joypad::new(),
            cart: None,
            bootstrap_enabled: false, // Bootstrap ROM already disabled
        };
        
        // Set post-boot hardware register values
        mmap.init_post_boot_registers();
        
        // Load standard tile graphics data that games expect after boot ROM
        mmap.init_post_boot_vram();
        
        mmap
    }
    
    /// Initialize hardware registers to their post-boot state
    fn init_post_boot_registers(&mut self) {
        // Sound registers (all disabled after boot)
        self.contents[0xFF10] = 0x80; // NR10
        self.contents[0xFF11] = 0xBF; // NR11
        self.contents[0xFF12] = 0xF3; // NR12
        self.contents[0xFF14] = 0xBF; // NR14
        self.contents[0xFF16] = 0x3F; // NR21
        self.contents[0xFF17] = 0x00; // NR22
        self.contents[0xFF19] = 0xBF; // NR24
        self.contents[0xFF1A] = 0x7F; // NR30
        self.contents[0xFF1B] = 0xFF; // NR31
        self.contents[0xFF1C] = 0x9F; // NR32
        self.contents[0xFF1E] = 0xBF; // NR34
        self.contents[0xFF20] = 0xFF; // NR41
        self.contents[0xFF21] = 0x00; // NR42
        self.contents[0xFF22] = 0x00; // NR43
        self.contents[0xFF23] = 0xBF; // NR44
        self.contents[0xFF24] = 0x77; // NR50
        self.contents[0xFF25] = 0xF3; // NR51
        self.contents[0xFF26] = 0xF1; // NR52
        
        // Interrupt registers (disabled after boot)
        self.contents[0xFF0F] = 0xE0; // IF - no interrupts pending
        self.contents[0xFFFF] = 0x00; // IE - all interrupts disabled
        
        // Bootstrap disable register (already disabled)
        self.contents[0xFF50] = 0x01; // Bootstrap ROM disabled
    }
    
    /// Initialize VRAM with tile graphics data that games expect after boot ROM completion
    fn init_post_boot_vram(&mut self) {
        // The boot ROM loads standard tile graphics data to VRAM that many games depend on
        // Pokemon specifically expects tile graphics at 0x8000+ range to be available
        
        // Create a simple tile pattern that Pokemon can use for tile ID 0x7F
        // This is a basic 8x8 tile with some pattern instead of all zeros
        let basic_tile_data = [
            0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,  // Row pattern
            0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,  // Row pattern  
        ];
        
        // Pokemon uses tile ID 0x7F in signed mode, which maps to:
        // 0x9000 + (127 * 16) = 0x97F0 (VRAM offset 0x17F0)
        let tile_addr = 0x17F0;
        
        // Copy tile data to VRAM  
        for (i, &byte) in basic_tile_data.iter().enumerate() {
            if tile_addr + i < 0x2000 {  // Ensure within VRAM bounds
                self.ppu.vram[tile_addr + i] = byte;
            }
        }
        
    }
    
    /// Disables the bootstrap ROM, allowing cartridge access to 0x0000-0x00FF
    pub fn disable_bootstrap(&mut self) {
        self.bootstrap_enabled = false;
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
            // Joypad Register (0xFF00)
            0xFF00 => {
                self.joypad.write_register(val);
            }
            // Timer Registers (0xFF04-0xFF07)
            0xFF04..=0xFF07 => {
                self.timer.write_register(addr, val);
            }
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
            // Bootstrap disable register
            0xFF50 => {
                if val != 0 {
                    self.bootstrap_enabled = false;
                    #[cfg(debug_assertions)]
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
            // Joypad Register (0xFF00)
            0xFF00 => self.joypad.read_register(),
            // Timer Registers (0xFF04-0xFF07)
            0xFF04..=0xFF07 => self.timer.read_register(addr),
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
            
        } else {
        }
    }

    pub fn get_ppu(&self) -> &Ppu {
        &self.ppu
    }

    #[allow(dead_code)] // Public API method
    pub fn get_ppu_mut(&mut self) -> &mut Ppu {
        &mut self.ppu
    }

    pub fn step_ppu(&mut self, cycles: u16) -> (bool, bool) {
        self.ppu.step(cycles);
        
        // Check for PPU interrupt flags and return them
        let vblank_interrupt = self.ppu.vblank_interrupt;
        let stat_interrupt = self.ppu.stat_interrupt;
        
        // Clear PPU interrupt flags after reading them
        self.ppu.vblank_interrupt = false;
        self.ppu.stat_interrupt = false;
        
        (vblank_interrupt, stat_interrupt)
    }

    pub fn step_timer(&mut self, cycles: u16) -> bool {
        self.timer.step(cycles)
    }
    
    pub fn update_joypad(&mut self, buttons: super::joypad::JoypadButtons) -> bool {
        self.joypad.update_buttons(buttons)
    }
    
    pub fn joypad_interrupt_requested(&self) -> bool {
        self.joypad.any_button_pressed()
    }
}
