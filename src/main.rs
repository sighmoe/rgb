mod rgb;

use macroquad::prelude::*;
use rgb::cpu::Cpu;

struct GameBoyEmulator {
    cpu: Cpu,
}

impl GameBoyEmulator {
    fn new(rom_path: &str, skip_boot_rom: bool) -> Self {
        let mut cpu = if skip_boot_rom {
            Cpu::new_post_boot()
        } else {
            Cpu::new()
        };
        
        // Load cartridge from provided path
        cpu.mmap.load_cartridge(std::path::Path::new(rom_path));
        
        if skip_boot_rom {
            // Disable bootstrap ROM and start at cartridge entry point
            cpu.mmap.disable_bootstrap();
            cpu.pc = 0x0100;  // Cartridge entry point
        } else {
            // Set initial state for Game Boy boot sequence
            cpu.pc = 0x0000;  // Start at bootstrap ROM
            cpu.sp = 0xFFFE;  // Initial stack pointer
        }
        
        Self { cpu }
    }

    fn step(&mut self) {
        // Handle interrupts first (even when halted, interrupts can wake CPU)
        if self.cpu.check_interrupts() {
            let interrupt_cycles = self.cpu.handle_interrupt();
            if interrupt_cycles > 0 {
                // Step PPU for interrupt handling cycles, checking for new interrupts
                let (vblank_interrupt, stat_interrupt) = self.cpu.mmap.step_ppu(interrupt_cycles as u16);
                if vblank_interrupt {
                    self.cpu.request_vblank_interrupt();
                }
                if stat_interrupt {
                    self.cpu.request_lcd_stat_interrupt();
                }
                return;
            }
        }
        
        // If CPU is halted, still need to step timer and PPU to generate interrupts
        if self.cpu.halted {
            // When halted, advance 4 cycles (1 M-cycle) to keep timer/PPU running
            let halt_cycles = 4u16;
            
            // Step timer and check for timer interrupt
            if self.cpu.mmap.step_timer(halt_cycles) {
                self.cpu.request_timer_interrupt();
            }
            
            // Step PPU and check for PPU interrupts
            let (vblank_interrupt, stat_interrupt) = self.cpu.mmap.step_ppu(halt_cycles);
            if vblank_interrupt {
                self.cpu.request_vblank_interrupt();
            }
            if stat_interrupt {
                self.cpu.request_lcd_stat_interrupt();
            }
            
            return;
        }
        
        
        if !self.cpu.halted {
            // Debug: Check if we're stuck in a loop
            static mut LAST_PC: u16 = 0;
            static mut STUCK_COUNT: u32 = 0;
            static mut STEP_COUNT: u32 = 0;
            unsafe {
                STEP_COUNT += 1;
                
                // Debug timer and interrupt registers periodically
                if STEP_COUNT % 50000 == 0 {
                    #[cfg(debug_assertions)]
                    {
                        use log::debug;
                        let div = self.cpu.mmap.read(0xFF04);
                        let tima = self.cpu.mmap.read(0xFF05);
                        let tma = self.cpu.mmap.read(0xFF06);
                        let tac = self.cpu.mmap.read(0xFF07);
                        let ie = self.cpu.mmap.read(0xFFFF);
                        let if_reg = self.cpu.mmap.read(0xFF0F);
                        debug!("STEP {}: PC=0x{:04X}, DIV=0x{:02X}, TIMA=0x{:02X}, TMA=0x{:02X}, TAC=0x{:02X}, IE=0x{:02X}, IF=0x{:02X}, IME={}", 
                            STEP_COUNT, self.cpu.pc, div, tima, tma, tac, ie, if_reg, self.cpu.ime);
                    }
                }
                
                if LAST_PC == self.cpu.pc {
                    STUCK_COUNT += 1;
                    if STUCK_COUNT > 100 {
                        #[cfg(debug_assertions)]
                        {
                            use log::debug;
                            if self.cpu.pc == 0x00E9 {
                                let hl = self.cpu.registers.get_hl();
                                let de = self.cpu.registers.get_de();
                                let mem_val = self.cpu.mmap.read(hl);
                                debug!("BOOTSTRAP LOGO VERIFICATION LOOP:");
                                debug!("  PC: 0x{:04X}, A: 0x{:02X}, HL: 0x{:04X}, DE: 0x{:04X}", 
                                    self.cpu.pc, self.cpu.registers.a, hl, de);
                                debug!("  Memory at (HL): 0x{:02X}, Zero flag: {}", 
                                    mem_val, self.cpu.registers.f.zero);
                                debug!("  Expected comparison: A(0x{:02X}) vs (HL)(0x{:02X})", 
                                    self.cpu.registers.a, mem_val);
                                debug!("  DE should be in range 0x00A8-0x00D7, current offset: 0x{:02X}", 
                                    de.wrapping_sub(0x00A8));
                            } else {
                                debug!("STUCK at PC: 0x{:04X}, instruction: 0x{:02X}", 
                                    self.cpu.pc, 
                                    self.cpu.mmap.read(self.cpu.pc)
                                );
                            }
                        }
                        STUCK_COUNT = 0;
                    }
                } else {
                    STUCK_COUNT = 0;
                }
                LAST_PC = self.cpu.pc;
            }
            
            let instruction = self.cpu.decode();
            let cycles = self.cpu.execute(instruction);
            
            // Handle EI delay after instruction execution
            self.cpu.handle_ei_delay();
        }
    }

    fn get_frame_buffer(&self) -> &[u8] {
        self.cpu.mmap.get_ppu().get_frame_buffer()
    }
}

#[macroquad::main("Game Boy Emulator")]
async fn main() {
    // Initialize logger (only in debug builds)
    #[cfg(debug_assertions)]
    env_logger::init();
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut rom_path = "./test-roms/pkmn.gb"; // Default ROM if no argument provided
    let mut skip_boot_rom = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--skip-boot" | "-s" => {
                skip_boot_rom = true;
                i += 1;
            }
            "--help" | "-h" => {
                println!("Game Boy Emulator");
                println!("Usage: {} [options] [rom_path]", args[0]);
                println!();
                println!("Options:");
                println!("  --skip-boot, -s    Skip the Game Boy boot sequence and start directly with the ROM");
                println!("  --help, -h         Show this help message");
                println!();
                println!("If no ROM path is provided, defaults to './test-roms/pkmn.gb'");
                return;
            }
            arg if !arg.starts_with("--") => {
                rom_path = arg;
                i += 1;
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                eprintln!("Use --help for usage information");
                return;
            }
        }
    }
    
    let mut emulator = GameBoyEmulator::new(rom_path, skip_boot_rom);
    
    loop {
        clear_background(GRAY);

        // Run emulator steps  
        for _ in 0..50000 {  // Much faster execution for testing
            emulator.step();
            
            // Break if CPU is halted to prevent infinite loops
            if emulator.cpu.halted {
                break;
            }
        }
        
        // Remove test pattern to see actual bootstrap ROM output
        
        // Get frame buffer from PPU
        let frame_buffer = emulator.get_frame_buffer();
        
        // Debug: Print some state occasionally
        static mut FRAME_COUNTER: u32 = 0;
        unsafe {
            FRAME_COUNTER += 1;
            if FRAME_COUNTER % 30 == 0 {  // Print more frequently with faster speed
                let ly_value = emulator.cpu.mmap.read(0xFF44);
                #[cfg(debug_assertions)]
                {
                    use log::debug;
                    if ly_value == 144 {
                        debug!("*** LY REACHED 144! PC: 0x{:04X}, A: 0x{:02X} ***", emulator.cpu.pc, emulator.cpu.registers.a);
                    }
                    debug!("PC: 0x{:04X}, A: 0x{:02X}, C: 0x{:02X}, E: 0x{:02X}, LY: {}, Bootstrap: {}", 
                        emulator.cpu.pc, 
                        emulator.cpu.registers.a,
                        emulator.cpu.registers.c,
                        emulator.cpu.registers.e,
                        ly_value,
                        if emulator.cpu.pc < 0x0100 { "ON" } else { "OFF" }
                    );
                }
            }
            
            #[cfg(debug_assertions)]
            {
                use log::debug;
                // Debug when entering logo verification
                if emulator.cpu.pc == 0x00E0 {
                    let hl = emulator.cpu.registers.get_hl();
                    let de = emulator.cpu.registers.get_de();
                    debug!("ENTERING LOGO VERIFICATION: HL=0x{:04X}, DE=0x{:04X}", hl, de);
                }
                
                // Debug PPU state during bootstrap logo rendering
                if emulator.cpu.pc >= 0x0070 && emulator.cpu.pc < 0x00E0 && FRAME_COUNTER % 60 == 0 {
                    let lcdc = emulator.cpu.mmap.read(0xFF40);
                    let bgp = emulator.cpu.mmap.read(0xFF47);
                    let scy = emulator.cpu.mmap.read(0xFF42);
                    let scx = emulator.cpu.mmap.read(0xFF43);
                    debug!("PPU DEBUG: LCDC=0x{:02X} (bg_window_tiles={}), BGP=0x{:02X}, SCY={}, SCX={}", 
                        lcdc, (lcdc & 0x10) != 0, bgp, scy, scx);
                }
            }
        }
        
        // Debug logging (only in debug builds)
        #[cfg(debug_assertions)]
        unsafe {
            if FRAME_COUNTER % 30 == 0 && (emulator.cpu.pc >= 0x0060 || !emulator.cpu.mmap.bootstrap_enabled) {
                use log::debug;
                debug!("=== NINTENDO LOGO ANALYSIS ===");
                let frame_buffer = emulator.get_frame_buffer();
                
                // Print the actual logo area with better contrast
                for debug_y in 64..80 {
                    let mut line = String::new();
                    for debug_x in 32..128 {
                        let pixel = frame_buffer[debug_y * 160 + debug_x];
                        let char = match pixel {
                            0 => ' ',  // White (background)
                            1 => '.',  // Light gray
                            2 => '#',  // Dark gray
                            3 => '█',  // Black (logo)
                            _ => '?',  // Error
                        };
                        line.push(char);
                    }
                    debug!("Y{:02}: {}", debug_y, line);
                }
                
                // Debug tile addressing mode issue
                let lcdc = emulator.cpu.mmap.read(0xFF40);
                let bg_window_tiles = (lcdc & 0x10) != 0;
                debug!("LCDC: 0x{:02X}, bg_window_tiles: {} ({})", 
                    lcdc, bg_window_tiles,
                    if bg_window_tiles { "unsigned 0x8000-0x8FFF" } else { "signed 0x8800-0x97FF base 0x9000" }
                );
                
                debug!("=== END NINTENDO LOGO ANALYSIS ===");
            }
        }

        // Draw the Game Boy screen (160x144)
        let scale = 4.0;
        for y in 0..144 {
            for x in 0..160 {
                let pixel = frame_buffer[y * 160 + x];
                let color = match pixel {
                    0 => WHITE,      // Lightest
                    1 => LIGHTGRAY,  // Light
                    2 => DARKGRAY,   // Dark  
                    3 => BLACK,      // Darkest
                    _ => MAGENTA,    // Error color
                };
                
                draw_rectangle(
                    x as f32 * scale,
                    y as f32 * scale,
                    scale,
                    scale,
                    color
                );
            }
        }

        draw_text("Game Boy Emulator", 10.0, screen_height() - 20.0, 20.0, WHITE);

        next_frame().await
    }
}
