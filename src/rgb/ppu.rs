// Game Boy PPU (Picture Processing Unit) Implementation
// Based on DMG (original Game Boy) specifications

use log::debug;

// PPU Constants
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const TILE_SIZE: usize = 8;
pub const TILES_PER_ROW: usize = 32;
pub const VRAM_SIZE: usize = 0x2000; // 8KB
pub const OAM_SIZE: usize = 160; // 160 bytes
pub const MAX_SPRITES: usize = 40;
pub const MAX_SPRITES_PER_LINE: usize = 10;

// PPU Register Addresses
pub const LCDC_ADDR: u16 = 0xFF40; // LCD Control
pub const STAT_ADDR: u16 = 0xFF41; // LCD Status
pub const SCY_ADDR: u16 = 0xFF42;  // Background Scroll Y
pub const SCX_ADDR: u16 = 0xFF43;  // Background Scroll X
pub const LY_ADDR: u16 = 0xFF44;   // Current Scanline
pub const LYC_ADDR: u16 = 0xFF45;  // Scanline Compare
pub const WY_ADDR: u16 = 0xFF4A;   // Window Y Position
pub const WX_ADDR: u16 = 0xFF4B;   // Window X Position
pub const BGP_ADDR: u16 = 0xFF47;  // Background Palette
pub const OBP0_ADDR: u16 = 0xFF48; // Object Palette 0
pub const OBP1_ADDR: u16 = 0xFF49; // Object Palette 1

// PPU Modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PpuMode {
    HBlank = 0,  // Mode 0
    VBlank = 1,  // Mode 1
    OamScan = 2, // Mode 2
    Drawing = 3, // Mode 3
}

// PPU Timing (in CPU cycles) - Game Boy DMG specs
pub const OAM_SCAN_CYCLES: u16 = 80;   // Mode 2: OAM scan
pub const DRAWING_CYCLES: u16 = 172;   // Mode 3: Drawing (variable, but avg 172)
pub const HBLANK_CYCLES: u16 = 204;    // Mode 0: H-Blank  
pub const SCANLINE_CYCLES: u16 = 456;  // Total cycles per scanline (80+172+204)
pub const VBLANK_LINES: u8 = 10;       // 10 lines of VBlank (144-153)

// LCDC Register Bits
pub struct LcdcFlags {
    pub lcd_enable: bool,        // Bit 7
    pub window_tile_map: bool,   // Bit 6: 0=9800-9BFF, 1=9C00-9FFF
    pub window_enable: bool,     // Bit 5
    pub bg_window_tiles: bool,   // Bit 4: 0=8800-97FF, 1=8000-8FFF
    pub bg_tile_map: bool,       // Bit 3: 0=9800-9BFF, 1=9C00-9FFF
    pub sprite_size: bool,       // Bit 2: 0=8x8, 1=8x16
    pub sprite_enable: bool,     // Bit 1
    pub bg_enable: bool,         // Bit 0
}

impl LcdcFlags {
    pub fn from_byte(byte: u8) -> Self {
        Self {
            lcd_enable: (byte & 0x80) != 0,
            window_tile_map: (byte & 0x40) != 0,
            window_enable: (byte & 0x20) != 0,
            bg_window_tiles: (byte & 0x10) != 0,
            bg_tile_map: (byte & 0x08) != 0,
            sprite_size: (byte & 0x04) != 0,
            sprite_enable: (byte & 0x02) != 0,
            bg_enable: (byte & 0x01) != 0,
        }
    }

    pub fn to_byte(&self) -> u8 {
        (if self.lcd_enable { 0x80 } else { 0 })
            | (if self.window_tile_map { 0x40 } else { 0 })
            | (if self.window_enable { 0x20 } else { 0 })
            | (if self.bg_window_tiles { 0x10 } else { 0 })
            | (if self.bg_tile_map { 0x08 } else { 0 })
            | (if self.sprite_size { 0x04 } else { 0 })
            | (if self.sprite_enable { 0x02 } else { 0 })
            | (if self.bg_enable { 0x01 } else { 0 })
    }
}

// STAT Register Bits
pub struct StatFlags {
    pub lyc_interrupt: bool,     // Bit 6
    pub oam_interrupt: bool,     // Bit 5
    pub vblank_interrupt: bool,  // Bit 4
    pub hblank_interrupt: bool,  // Bit 3
    pub lyc_flag: bool,          // Bit 2: LY == LYC
    pub mode: PpuMode,           // Bits 1-0
}

impl StatFlags {
    pub fn from_byte(byte: u8) -> Self {
        let mode = match byte & 0x03 {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::OamScan,
            3 => PpuMode::Drawing,
            _ => unreachable!(),
        };

        Self {
            lyc_interrupt: (byte & 0x40) != 0,
            oam_interrupt: (byte & 0x20) != 0,
            vblank_interrupt: (byte & 0x10) != 0,
            hblank_interrupt: (byte & 0x08) != 0,
            lyc_flag: (byte & 0x04) != 0,
            mode,
        }
    }

    pub fn to_byte(&self) -> u8 {
        (if self.lyc_interrupt { 0x40 } else { 0 })
            | (if self.oam_interrupt { 0x20 } else { 0 })
            | (if self.vblank_interrupt { 0x10 } else { 0 })
            | (if self.hblank_interrupt { 0x08 } else { 0 })
            | (if self.lyc_flag { 0x04 } else { 0 })
            | (self.mode as u8)
    }
}

// Sprite Attributes (OAM Entry)
#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub y: u8,          // Y position
    pub x: u8,          // X position
    pub tile: u8,       // Tile number
    pub flags: u8,      // Attributes
}

impl Sprite {
    pub fn from_oam_bytes(bytes: &[u8; 4]) -> Self {
        Self {
            y: bytes[0],
            x: bytes[1],
            tile: bytes[2],
            flags: bytes[3],
        }
    }

    pub fn priority(&self) -> bool { (self.flags & 0x80) == 0 }
    pub fn flip_y(&self) -> bool { (self.flags & 0x40) != 0 }
    pub fn flip_x(&self) -> bool { (self.flags & 0x20) != 0 }
    pub fn palette(&self) -> bool { (self.flags & 0x10) != 0 }
}

// PPU Structure
pub struct Ppu {
    // Video RAM and OAM
    pub vram: [u8; VRAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    
    // PPU Registers
    pub lcdc: LcdcFlags,
    pub stat: StatFlags,
    pub scy: u8,  // Background scroll Y
    pub scx: u8,  // Background scroll X
    pub ly: u8,   // Current scanline
    pub lyc: u8,  // Scanline compare
    pub wy: u8,   // Window Y position
    pub wx: u8,   // Window X position
    pub bgp: u8,  // Background palette
    pub obp0: u8, // Object palette 0
    pub obp1: u8, // Object palette 1
    
    // PPU State
    pub mode: PpuMode,
    pub cycles: u16,
    pub frame_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub scanline_sprites: Vec<Sprite>,
    
    // Interrupts
    pub vblank_interrupt: bool,
    pub stat_interrupt: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            lcdc: LcdcFlags::from_byte(0x91), // Default LCDC value
            stat: StatFlags::from_byte(0x00),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bgp: 0xFC, // Default palette
            obp0: 0xFF,
            obp1: 0xFF,
            mode: PpuMode::OamScan,
            cycles: 0,
            frame_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            scanline_sprites: Vec::with_capacity(MAX_SPRITES_PER_LINE),
            vblank_interrupt: false,
            stat_interrupt: false,
        }
    }
    
    /// Creates a new PPU in the post-boot state for skipping boot sequence
    /// Initializes registers to their expected values after boot ROM completion
    pub fn new_post_boot() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            lcdc: LcdcFlags::from_byte(0x91), // LCD enabled, BG on, sprites on
            stat: StatFlags::from_byte(0x00), // Mode 0 (H-Blank)
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bgp: 0xFC, // Background palette (11-11-11-00)
            obp0: 0xFF, // Object palette 0 (all black)
            obp1: 0xFF, // Object palette 1 (all black)
            mode: PpuMode::HBlank, // Start in H-Blank mode
            cycles: 0,
            frame_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            scanline_sprites: Vec::with_capacity(MAX_SPRITES_PER_LINE),
            vblank_interrupt: false,
            stat_interrupt: false,
        }
    }

    // Step PPU by CPU cycles - process all accumulated cycles
    pub fn step(&mut self, cycles: u16) {
        if !self.lcdc.lcd_enable {
            return;
        }

        self.cycles += cycles;

        // Continue processing until all cycles are consumed
        loop {
            let consumed = match self.mode {
                PpuMode::OamScan => self.handle_oam_scan(),
                PpuMode::Drawing => self.handle_drawing(),
                PpuMode::HBlank => self.handle_hblank(),
                PpuMode::VBlank => self.handle_vblank(),
            };
            
            // Break if no state transition occurred (not enough cycles)
            if !consumed {
                break;
            }
        }
    }

    fn handle_oam_scan(&mut self) -> bool {
        if self.cycles >= OAM_SCAN_CYCLES {
            self.cycles -= OAM_SCAN_CYCLES;
            self.scan_oam();
            self.set_mode(PpuMode::Drawing);
            true
        } else {
            false
        }
    }

    fn handle_drawing(&mut self) -> bool {
        if self.cycles >= DRAWING_CYCLES {
            self.cycles -= DRAWING_CYCLES;
            self.render_scanline();
            self.set_mode(PpuMode::HBlank);
            true
        } else {
            false
        }
    }

    fn handle_hblank(&mut self) -> bool {
        if self.cycles >= HBLANK_CYCLES {
            self.cycles -= HBLANK_CYCLES;
            self.ly += 1;
            self.update_lyc_flag();

            if self.ly >= SCREEN_HEIGHT as u8 {
                self.set_mode(PpuMode::VBlank);
                self.vblank_interrupt = true;
            } else {
                self.set_mode(PpuMode::OamScan);
            }
            true
        } else {
            false
        }
    }

    fn handle_vblank(&mut self) -> bool {
        if self.cycles >= SCANLINE_CYCLES {
            self.cycles -= SCANLINE_CYCLES;
            self.ly += 1;
            self.update_lyc_flag();

            if self.ly >= (SCREEN_HEIGHT as u8 + VBLANK_LINES) {
                self.ly = 0;
                self.set_mode(PpuMode::OamScan);
            }
            true
        } else {
            false
        }
    }

    fn set_mode(&mut self, mode: PpuMode) {
        self.mode = mode;
        self.stat.mode = mode;

        // Trigger STAT interrupts based on mode
        match mode {
            PpuMode::HBlank if self.stat.hblank_interrupt => self.stat_interrupt = true,
            PpuMode::VBlank if self.stat.vblank_interrupt => self.stat_interrupt = true,
            PpuMode::OamScan if self.stat.oam_interrupt => self.stat_interrupt = true,
            _ => {}
        }
    }

    fn update_lyc_flag(&mut self) {
        let lyc_match = self.ly == self.lyc;
        self.stat.lyc_flag = lyc_match;
        
        if lyc_match && self.stat.lyc_interrupt {
            self.stat_interrupt = true;
        }
    }

    fn scan_oam(&mut self) {
        self.scanline_sprites.clear();
        
        if !self.lcdc.sprite_enable {
            return;
        }

        let sprite_height = if self.lcdc.sprite_size { 16 } else { 8 };
        
        for i in 0..MAX_SPRITES {
            let sprite_addr = i * 4;
            let sprite_data = [
                self.oam[sprite_addr],
                self.oam[sprite_addr + 1],
                self.oam[sprite_addr + 2],
                self.oam[sprite_addr + 3],
            ];
            
            let sprite = Sprite::from_oam_bytes(&sprite_data);
            
            // Check if sprite is on current scanline
            let sprite_y = sprite.y.wrapping_sub(16);
            if self.ly >= sprite_y && self.ly < sprite_y + sprite_height {
                self.scanline_sprites.push(sprite);
                
                if self.scanline_sprites.len() >= MAX_SPRITES_PER_LINE {
                    break;
                }
            }
        }
    }

    fn render_scanline(&mut self) {
        if !self.lcdc.lcd_enable {
            return;
        }

        let y = self.ly as usize;
        if y >= SCREEN_HEIGHT {
            return;
        }

        // Render background
        if self.lcdc.bg_enable {
            self.render_background_line(y);
        }

        // Render window
        if self.lcdc.window_enable && self.ly >= self.wy {
            self.render_window_line(y);
        }

        // Render sprites
        if self.lcdc.sprite_enable {
            self.render_sprites_line(y);
        }
    }

    fn render_background_line(&mut self, y: usize) {
        let scroll_y = self.scy.wrapping_add(y as u8) as usize;
        let tile_y = scroll_y / TILE_SIZE;
        let pixel_y = scroll_y % TILE_SIZE;

        for x in 0..SCREEN_WIDTH {
            let scroll_x = self.scx.wrapping_add(x as u8) as usize;
            let tile_x = scroll_x / TILE_SIZE;
            let pixel_x = scroll_x % TILE_SIZE;

            // VRAM addresses:
            // 0x9800-0x9BFF: Background Tile Map 0 (VRAM offset 0x1800)
            // 0x9C00-0x9FFF: Background Tile Map 1 (VRAM offset 0x1C00)
            let tile_map_addr = if self.lcdc.bg_tile_map { 0x1C00 } else { 0x1800 };
            let tile_index = (tile_y % 32) * TILES_PER_ROW + (tile_x % 32);
            let tile_id = self.vram[tile_map_addr + tile_index];

            let tile_data_addr = if self.lcdc.bg_window_tiles {
                // Unsigned addressing (0x8000-0x8FFF)
                tile_id as usize * 16
            } else {
                // Signed addressing (0x8800-0x97FF) with base at 0x9000
                let signed_tile_id = tile_id as i8 as i16;
                (0x1000_i16 + signed_tile_id * 16) as usize
            };

            let pixel_color = self.get_tile_pixel(tile_data_addr, pixel_x, pixel_y);
            let final_color = self.apply_palette(pixel_color, self.bgp);
            
            // Nintendo logo tile mapping verified to be working correctly
            
            self.frame_buffer[y * SCREEN_WIDTH + x] = final_color;
        }
    }

    fn render_window_line(&mut self, y: usize) {
        if self.wx >= 167 || y < self.wy as usize {
            return;
        }

        let window_y = y - self.wy as usize;
        let tile_y = window_y / TILE_SIZE;
        let pixel_y = window_y % TILE_SIZE;

        let start_x = if self.wx < 7 { 0 } else { self.wx as usize - 7 };

        for x in start_x..SCREEN_WIDTH {
            let window_x = x - start_x;
            let tile_x = window_x / TILE_SIZE;
            let pixel_x = window_x % TILE_SIZE;

            let tile_map_addr = if self.lcdc.window_tile_map { 0x1C00 } else { 0x1800 };
            let tile_index = tile_y * TILES_PER_ROW + tile_x;
            let tile_id = self.vram[tile_map_addr + tile_index];

            let tile_data_addr = if self.lcdc.bg_window_tiles {
                tile_id as usize * 16
            } else {
                let signed_tile_id = tile_id as i8 as i16;
                (0x1000_i16 + signed_tile_id * 16) as usize
            };

            let pixel_color = self.get_tile_pixel(tile_data_addr, pixel_x, pixel_y);
            let final_color = self.apply_palette(pixel_color, self.bgp);
            
            self.frame_buffer[y * SCREEN_WIDTH + x] = final_color;
        }
    }

    fn render_sprites_line(&mut self, y: usize) {
        // Render sprites in reverse order for proper priority
        for sprite in self.scanline_sprites.iter().rev() {
            let sprite_y = sprite.y.wrapping_sub(16) as usize;
            let sprite_x = sprite.x.wrapping_sub(8) as usize;
            
            if sprite_x >= SCREEN_WIDTH {
                continue;
            }

            let line_in_sprite = y - sprite_y;
            let actual_line = if sprite.flip_y() {
                if self.lcdc.sprite_size { 15 - line_in_sprite } else { 7 - line_in_sprite }
            } else {
                line_in_sprite
            };

            let tile_id = if self.lcdc.sprite_size {
                sprite.tile & 0xFE // 8x16 sprites use even tile numbers
            } else {
                sprite.tile
            };

            let tile_data_addr = tile_id as usize * 16 + actual_line * 2;

            for pixel_x in 0..TILE_SIZE {
                let screen_x = sprite_x + pixel_x;
                if screen_x >= SCREEN_WIDTH {
                    break;
                }

                let actual_pixel_x = if sprite.flip_x() { 7 - pixel_x } else { pixel_x };
                let pixel_color = self.get_tile_pixel(tile_data_addr, actual_pixel_x, 0);
                
                if pixel_color == 0 {
                    continue; // Transparent pixel
                }

                // Check sprite priority
                if sprite.priority() || self.frame_buffer[y * SCREEN_WIDTH + screen_x] == 0 {
                    let palette = if sprite.palette() { self.obp1 } else { self.obp0 };
                    let final_color = self.apply_palette(pixel_color, palette);
                    self.frame_buffer[y * SCREEN_WIDTH + screen_x] = final_color;
                }
            }
        }
    }

    fn get_tile_pixel(&self, tile_data_addr: usize, pixel_x: usize, pixel_y: usize) -> u8 {
        let byte_offset = tile_data_addr + pixel_y * 2;
        
        // Ensure we don't read outside VRAM bounds
        if byte_offset + 1 >= VRAM_SIZE {
            return 0;
        }
        
        let low_byte = self.vram[byte_offset];
        let high_byte = self.vram[byte_offset + 1];
        
        let bit = 7 - pixel_x;
        let low_bit = (low_byte >> bit) & 1;
        let high_bit = (high_byte >> bit) & 1;
        
        let pixel_color = (high_bit << 1) | low_bit;
        
        // Pixel extraction verified to be working correctly for Nintendo logo
        
        pixel_color
    }

    fn apply_palette(&self, color: u8, palette: u8) -> u8 {
        let final_color = match color {
            0 => palette & 0x03,
            1 => (palette >> 2) & 0x03,
            2 => (palette >> 4) & 0x03,
            3 => (palette >> 6) & 0x03,
            _ => 0,
        };
        
        // Palette processing working correctly
        
        final_color
    }

    // Memory-mapped register access
    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            LCDC_ADDR => self.lcdc.to_byte(),
            STAT_ADDR => self.stat.to_byte(),
            SCY_ADDR => self.scy,
            SCX_ADDR => self.scx,
            LY_ADDR => self.ly,
            LYC_ADDR => self.lyc,
            WY_ADDR => self.wy,
            WX_ADDR => self.wx,
            BGP_ADDR => self.bgp,
            OBP0_ADDR => self.obp0,
            OBP1_ADDR => self.obp1,
            _ => 0xFF,
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            LCDC_ADDR => {
                let old_enable = self.lcdc.lcd_enable;
                let old_bg_window_tiles = self.lcdc.bg_window_tiles;
                self.lcdc = LcdcFlags::from_byte(value);
                
                // Debug LCDC changes during bootstrap
                if old_bg_window_tiles != self.lcdc.bg_window_tiles {
                    debug!("LCDC tile addressing changed: bg_window_tiles={} (was {})", 
                        self.lcdc.bg_window_tiles, old_bg_window_tiles);
                }
                
                // Handle LCD disable
                if old_enable && !self.lcdc.lcd_enable {
                    self.ly = 0;
                    self.cycles = 0;
                    self.mode = PpuMode::HBlank;
                    self.stat.mode = PpuMode::HBlank;
                }
            },
            STAT_ADDR => {
                // Only bits 6-3 are writable
                let old_stat = self.stat.to_byte();
                let new_stat = (value & 0x78) | (old_stat & 0x87);
                self.stat = StatFlags::from_byte(new_stat);
            },
            SCY_ADDR => self.scy = value,
            SCX_ADDR => self.scx = value,
            LYC_ADDR => {
                self.lyc = value;
                self.update_lyc_flag();
            },
            WY_ADDR => self.wy = value,
            WX_ADDR => self.wx = value,
            BGP_ADDR => self.bgp = value,
            OBP0_ADDR => self.obp0 = value,
            OBP1_ADDR => self.obp1 = value,
            _ => {},
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        if self.mode == PpuMode::Drawing {
            return 0xFF; // VRAM inaccessible during drawing
        }
        self.vram[(addr - 0x8000) as usize]
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        if self.mode == PpuMode::Drawing {
            return; // VRAM inaccessible during drawing
        }
        
        
        // Debug all VRAM writes during Nintendo logo setup
        if addr >= 0x8000 && addr <= 0x9FFF && value != 0x00 {
            static mut VRAM_WRITE_COUNT: u32 = 0;
            unsafe {
                VRAM_WRITE_COUNT += 1;
                if VRAM_WRITE_COUNT <= 50 {
                    let area = if addr < 0x9800 { "TILES" } else { "TILEMAP" };
                    debug!("VRAM {}: addr=0x{:04X}, value=0x{:02X}", area, addr, value);
                }
            }
        }
        
        self.vram[(addr - 0x8000) as usize] = value;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        if self.mode == PpuMode::Drawing || self.mode == PpuMode::OamScan {
            return 0xFF; // OAM inaccessible during drawing and OAM scan
        }
        self.oam[(addr - 0xFE00) as usize]
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        if self.mode == PpuMode::Drawing || self.mode == PpuMode::OamScan {
            return; // OAM inaccessible during drawing and OAM scan
        }
        self.oam[(addr - 0xFE00) as usize] = value;
    }

    // Get the current frame buffer for display
    pub fn get_frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }

    // Check and clear interrupt flags
    #[allow(dead_code)] // Public API method
    pub fn take_vblank_interrupt(&mut self) -> bool {
        let interrupt = self.vblank_interrupt;
        self.vblank_interrupt = false;
        interrupt
    }

    #[allow(dead_code)] // Public API method
    pub fn take_stat_interrupt(&mut self) -> bool {
        let interrupt = self.stat_interrupt;
        self.stat_interrupt = false;
        interrupt
    }
}