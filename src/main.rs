mod rgb;

use macroquad::prelude::*;
use rgb::cpu::Cpu;
use std::fs::File;
use std::io::{BufWriter, Write};

struct GameBoyEmulator {
    cpu: Cpu,
    #[cfg(debug_assertions)]
    trace_writer: Option<BufWriter<File>>,
    #[cfg(debug_assertions)]
    trace_json: bool,
    #[cfg(debug_assertions)]
    instruction_count: u64,
}

impl GameBoyEmulator {
    fn new(rom_path: &str, skip_boot_rom: bool, trace_file: Option<String>, trace_json: bool) -> Self {
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
        
        #[cfg(debug_assertions)]
        let trace_writer = if let Some(trace_path) = trace_file {
            match File::create(&trace_path) {
                Ok(file) => {
                    let mut writer = BufWriter::new(file);
                    if trace_json {
                        // Write JSON array opening
                        writeln!(writer, "[").unwrap();
                    }
                    Some(writer)
                }
                Err(e) => {
                    eprintln!("Warning: Failed to create trace file '{}': {}", trace_path, e);
                    None
                }
            }
        } else {
            None
        };
        
        Self { 
            cpu,
            #[cfg(debug_assertions)]
            trace_writer,
            #[cfg(debug_assertions)]
            trace_json,
            #[cfg(debug_assertions)]
            instruction_count: 0,
        }
    }
    
    #[cfg(debug_assertions)]
    fn write_trace(&mut self) {
        if let Some(ref mut writer) = self.trace_writer {
            let pc = self.cpu.pc;
            let sp = self.cpu.sp;
            let regs = &self.cpu.registers;
            
            // Read the next few bytes for instruction context
            let mem1 = self.cpu.mmap.read(pc);
            let mem2 = self.cpu.mmap.read(pc.wrapping_add(1));
            let mem3 = self.cpu.mmap.read(pc.wrapping_add(2));
            let mem4 = self.cpu.mmap.read(pc.wrapping_add(3));
            
            if self.trace_json {
                let comma = if self.instruction_count > 0 { "," } else { "" };
                let trace_entry = format!(
                    r#"{}{{
    "instruction": {},
    "A": "{:02X}",
    "F": "{:02X}",
    "B": "{:02X}",
    "C": "{:02X}",
    "D": "{:02X}",
    "E": "{:02X}",
    "H": "{:02X}",
    "L": "{:02X}",
    "SP": "{:04X}",
    "PC": "{:04X}",
    "memory": ["{:02X}", "{:02X}", "{:02X}", "{:02X}"]
}}"#,
                    comma,
                    self.instruction_count,
                    regs.a,
                    u8::from(regs.f),
                    regs.b,
                    regs.c,
                    regs.d,
                    regs.e,
                    regs.h,
                    regs.l,
                    sp,
                    pc,
                    mem1, mem2, mem3, mem4
                );
                writeln!(writer, "{}", trace_entry).unwrap();
            } else {
                // Format: "A: 01 F: B0 B: 00 C: 13 D: 00 E: D8 H: 01 L: 4D SP: FFFE PC: 00:0101 (C3 13 02 CE)"
                let trace_line = format!(
                    "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
                    regs.a,
                    u8::from(regs.f),
                    regs.b,
                    regs.c,
                    regs.d,
                    regs.e,
                    regs.h,
                    regs.l,
                    sp,
                    pc,
                    mem1, mem2, mem3, mem4
                );
                writeln!(writer, "{}", trace_line).unwrap();
            }
            
            self.instruction_count += 1;
        }
    }

    fn get_frame_buffer(&self) -> &[u8] {
        self.cpu.mmap.get_ppu().get_frame_buffer()
    }
}

#[cfg(debug_assertions)]
impl Drop for GameBoyEmulator {
    fn drop(&mut self) {
        if let Some(ref mut writer) = self.trace_writer {
            if self.trace_json {
                // Close JSON array
                writeln!(writer, "]").unwrap();
            }
            // Flush the writer
            writer.flush().unwrap();
        }
    }
}

#[macroquad::main("Game Boy Emulator")]
async fn main() {
    // Set target FPS to 60 (matching Game Boy refresh rate) with optimized screen size
    request_new_screen_size(640.0, 576.0); // 160*4 x 144*4 scale
    
    // Initialize logger (only in debug builds)
    #[cfg(debug_assertions)]
    env_logger::init();
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut rom_path = "./test-roms/pkmn.gb"; // Default ROM if no argument provided
    let mut skip_boot_rom = false;
    let mut trace_file: Option<String> = None;
    let mut trace_json = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--skip-boot" | "-s" => {
                skip_boot_rom = true;
                i += 1;
            }
            "--trace" | "-t" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --trace requires a file path");
                    return;
                }
                trace_file = Some(args[i + 1].clone());
                i += 2;
            }
            "--trace-json" => {
                trace_json = true;
                i += 1;
            }
            "--help" | "-h" => {
                println!("Game Boy Emulator");
                println!("Usage: {} [options] [rom_path]", args[0]);
                println!();
                println!("Options:");
                println!("  --skip-boot, -s      Skip the Game Boy boot sequence and start directly with the ROM");
                println!("  --trace, -t <file>   Write execution trace to the specified file");
                println!("  --trace-json         Format trace output as JSON (requires --trace)");
                println!("  --help, -h           Show this help message");
                println!();
                println!("Debug tracing is only available in debug builds.");
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
    
    // Validate trace options
    if trace_json && trace_file.is_none() {
        eprintln!("Error: --trace-json requires --trace <file>");
        return;
    }
    
    let mut emulator = GameBoyEmulator::new(rom_path, skip_boot_rom, trace_file, trace_json);
    
    
    loop {
        
        clear_background(GRAY);

        // Poll keyboard input and update joypad state
        let joypad_buttons = rgb::joypad::JoypadButtons {
            a: is_key_down(KeyCode::Z),          // Z key = A button
            b: is_key_down(KeyCode::X),          // X key = B button  
            start: is_key_down(KeyCode::Enter),       // Enter = START button
            select: is_key_down(KeyCode::RightShift), // Right Shift = SELECT button
            up: is_key_down(KeyCode::Up),        // Arrow keys = D-pad
            down: is_key_down(KeyCode::Down),
            left: is_key_down(KeyCode::Left),
            right: is_key_down(KeyCode::Right),
        };
        
        
        // Update joypad and check for button press interrupts
        let button_pressed = emulator.cpu.mmap.update_joypad(joypad_buttons);
        if button_pressed {
            emulator.cpu.request_joypad_interrupt();
        }

        // Run emulator for optimal 60fps performance 
        let instructions_per_frame = 4000; // Higher count for better performance
        
        for _ in 0..instructions_per_frame {
            if emulator.cpu.halted {
                // When halted, still step the hardware
                let halt_cycles = 4u16;
                
                if emulator.cpu.mmap.step_timer(halt_cycles) {
                    emulator.cpu.request_timer_interrupt();
                }
                
                let (vblank_interrupt, stat_interrupt) = emulator.cpu.mmap.step_ppu(halt_cycles);
                if vblank_interrupt {
                    emulator.cpu.request_vblank_interrupt();
                }
                if stat_interrupt {
                    emulator.cpu.request_lcd_stat_interrupt();
                }
                
                // Check for pending interrupts to wake up (HALT wakes up regardless of IME)
                let ie = emulator.cpu.mmap.read(0xFFFF);
                let if_reg = emulator.cpu.mmap.read(0xFF0F);
                let pending = ie & if_reg & 0x1F;
                if pending != 0 {
                    emulator.cpu.halted = false;
                    #[cfg(debug_assertions)]
                    {
                        static mut WAKE_COUNT: u32 = 0;
                        unsafe {
                            WAKE_COUNT += 1;
                            if WAKE_COUNT <= 10 {
                                println!("CPU woke from HALT #{} - IE: 0x{:02X}, IF: 0x{:02X}, pending: 0x{:02X}", 
                                    WAKE_COUNT, ie, if_reg, pending);
                            }
                        }
                    }
                }
            } else {
                // Execute one instruction
                let instruction = emulator.cpu.decode();
                
                // Save opcode for debugging after execution
                #[cfg(debug_assertions)]
                let opcode = instruction.instr;
                #[cfg(debug_assertions)]
                let pc_before = emulator.cpu.pc;
                
                // Debug specific problematic instructions
                #[cfg(debug_assertions)]
                {
                    if instruction.instr == 0x76 {
                        static mut HALT_COUNT: u32 = 0;
                        unsafe {
                            HALT_COUNT += 1;
                            if HALT_COUNT <= 10 {
                                println!("HALT instruction #{} at PC: 0x{:04X}, IME: {}, IE: 0x{:02X}, IF: 0x{:02X}", 
                                    HALT_COUNT, emulator.cpu.pc, emulator.cpu.ime, 
                                    emulator.cpu.mmap.read(0xFFFF), emulator.cpu.mmap.read(0xFF0F));
                            }
                        }
                    }
                    
                    // Debug the problematic LD (0x2000), A instruction
                    if instruction.instr == 0xEA && pc_before == 0x35DC {
                        static mut LD_DEBUG_COUNT: u32 = 0;
                        unsafe {
                            LD_DEBUG_COUNT += 1;
                            if LD_DEBUG_COUNT <= 5 {
                                println!("About to execute LD (0x2000), A #{} at PC: 0x{:04X}, A=0x{:02X}", 
                                    LD_DEBUG_COUNT, pc_before, emulator.cpu.registers.a);
                            }
                        }
                    }
                }
                
                let cycles = emulator.cpu.execute(instruction);
                
                // Debug PC after execution for the problematic instruction
                #[cfg(debug_assertions)]
                if opcode == 0xEA && pc_before == 0x35DC {
                    static mut LD_AFTER_COUNT: u32 = 0;
                    unsafe {
                        LD_AFTER_COUNT += 1;
                        if LD_AFTER_COUNT <= 5 {
                            println!("After LD execution #{}: PC was 0x{:04X}, now 0x{:04X}", 
                                LD_AFTER_COUNT, pc_before, emulator.cpu.pc);
                        }
                    }
                }
                
                emulator.cpu.handle_ei_delay();
                
                // Handle interrupts
                if emulator.cpu.check_interrupts() {
                    emulator.cpu.handle_interrupt();
                }
                
                // Step hardware with cycles from instruction
                if emulator.cpu.mmap.step_timer(cycles as u16) {
                    emulator.cpu.request_timer_interrupt();
                }
                
                let (vblank_interrupt, stat_interrupt) = emulator.cpu.mmap.step_ppu(cycles as u16);
                if vblank_interrupt {
                    emulator.cpu.request_vblank_interrupt();
                    #[cfg(debug_assertions)]
                    {
                        static mut VBLANK_COUNT: u32 = 0;
                        unsafe {
                            VBLANK_COUNT += 1;
                            if VBLANK_COUNT <= 10 || VBLANK_COUNT % 60 == 0 {
                                println!("VBlank interrupt #{} triggered", VBLANK_COUNT);
                            }
                        }
                    }
                }
                if stat_interrupt {
                    emulator.cpu.request_lcd_stat_interrupt();
                }
            }
        }
        
        
        
        // Get frame buffer from PPU
        let frame_buffer = emulator.get_frame_buffer();
        
        #[cfg(debug_assertions)]
        static mut LAST_FRAME_HASH: u64 = 0;
        #[cfg(debug_assertions)]
        static mut FRAME_CHANGE_COUNT: u32 = 0;
        #[cfg(debug_assertions)]
        static mut LAST_NON_ZERO_COUNT: usize = 0;
        #[cfg(debug_assertions)]
        static mut LAST_FRAME_BUFFER: Option<Vec<u8>> = None;
        
        #[cfg(debug_assertions)]
        let non_zero_count = frame_buffer.iter().filter(|&&pixel| pixel != 0).count();
        #[cfg(debug_assertions)]
        let current_hash = {
            // Simplified hash for better performance
            let mut hash = 0u64;
            for (i, &pixel) in frame_buffer.iter().step_by(4).enumerate() {
                if pixel != 0 {
                    hash = hash.wrapping_add((i as u64) * (pixel as u64 + 1));
                }
            }
            hash
        };
        
        #[cfg(debug_assertions)]
        {
            // Count how many render loops we go through total
            static mut TOTAL_RENDER_LOOPS: u32 = 0;
            static mut LAST_NON_ZERO_COUNT_DEBUG: usize = 0;
            
            unsafe {
                TOTAL_RENDER_LOOPS += 1;
                
                // Check if pixel count changed even if hash didn't (less frequent)
                if non_zero_count != LAST_NON_ZERO_COUNT_DEBUG {
                    LAST_NON_ZERO_COUNT_DEBUG = non_zero_count;
                    if TOTAL_RENDER_LOOPS % 1000 == 0 {
                        println!("Pixel count changed to {} (no frame hash change) at loop {}", non_zero_count, TOTAL_RENDER_LOOPS);
                    }
                }
                
                if current_hash != LAST_FRAME_HASH {
                    FRAME_CHANGE_COUNT += 1;
                    LAST_FRAME_HASH = current_hash;
                    
                    // Check if we have the bottom area changing (where blocks fall to)
                    let area_changes = if let Some(ref last_fb) = LAST_FRAME_BUFFER {
                        let mut bottom_changes = 0;
                        let mut middle_changes = 0;
                        let mut top_changes = 0;
                        
                        for y in 0..144 {
                            for x in 0..160 {
                                let idx = y * 160 + x;
                                if frame_buffer[idx] != last_fb[idx] {
                                    if y >= 120 { // Bottom area
                                        bottom_changes += 1;
                                    } else if y >= 60 && y < 120 { // Middle area (falling zone)
                                        middle_changes += 1;
                                    } else { // Top area
                                        top_changes += 1;
                                    }
                                }
                            }
                        }
                        (top_changes, middle_changes, bottom_changes)
                    } else {
                        (0, 0, 0)
                    };
                    
                    // Log significant frame changes - minimal logging for performance
                    if FRAME_CHANGE_COUNT <= 5 || FRAME_CHANGE_COUNT % 60 == 0 {
                        println!("FRAME UPDATE #{} (after {} render loops): {} pixels, changes: top:{} mid:{} bot:{}", 
                            FRAME_CHANGE_COUNT, TOTAL_RENDER_LOOPS, non_zero_count, area_changes.0, area_changes.1, area_changes.2);
                    }
                    
                    LAST_NON_ZERO_COUNT = non_zero_count;
                    LAST_FRAME_BUFFER = Some(frame_buffer.to_vec());
                } else if TOTAL_RENDER_LOOPS % 5000 == 0 {
                    static mut LAST_PC: u16 = 0;
                    static mut PC_STUCK_COUNT: u32 = 0;
                    
                    let pc = emulator.cpu.pc;
                    let halted = emulator.cpu.halted;
                    
                    // Read the instruction at current PC to see what's happening
                    let opcode = emulator.cpu.mmap.read(pc);
                    let next_byte = emulator.cpu.mmap.read(pc.wrapping_add(1));
                    let third_byte = emulator.cpu.mmap.read(pc.wrapping_add(2));
                    
                    // For LD (nn), A instruction (0xEA), show the full 16-bit address
                    let full_address = if opcode == 0xEA {
                        Some((third_byte as u16) << 8 | next_byte as u16)
                    } else {
                        None
                    };
                    
                    // If this is the LCD status polling loop, check what we're reading
                    let stat_value = if opcode == 0xF0 && next_byte == 0x41 {
                        Some(emulator.cpu.mmap.read(0xFF41))
                    } else {
                        None
                    };
                    
                    if pc == LAST_PC {
                        PC_STUCK_COUNT += 1;
                    } else {
                        PC_STUCK_COUNT = 0;
                        LAST_PC = pc;
                    }
                    
                    // If CPU is halted, check interrupt states
                    let (ie_reg, if_reg, ime) = if halted {
                        let ie = emulator.cpu.mmap.read(0xFFFF);
                        let if_val = emulator.cpu.mmap.read(0xFF0F);
                        (Some(ie), Some(if_val), emulator.cpu.ime)
                    } else {
                        (None, None, false)
                    };
                    
                    if let Some(stat) = stat_value {
                        println!("No frame change for {} loops - PC: 0x{:04X} (stuck: {}), Halted: {}, Pixels: {}, Opcode: 0x{:02X} 0x{:02X}, STAT: 0x{:02X} (mode: {})", 
                            TOTAL_RENDER_LOOPS, pc, PC_STUCK_COUNT, halted, non_zero_count, opcode, next_byte, stat, stat & 0x03);
                    } else if let (Some(ie), Some(if_val)) = (ie_reg, if_reg) {
                        println!("No frame change for {} loops - PC: 0x{:04X} (stuck: {}), Halted: {}, Pixels: {}, Opcode: 0x{:02X} 0x{:02X}, IE: 0x{:02X}, IF: 0x{:02X}, IME: {}", 
                            TOTAL_RENDER_LOOPS, pc, PC_STUCK_COUNT, halted, non_zero_count, opcode, next_byte, ie, if_val, ime);
                    } else if let Some(addr) = full_address {
                        let a_register = emulator.cpu.registers.a;
                        let stat_reg = emulator.cpu.mmap.read(0xFF41);
                        println!("No frame change for {} loops - PC: 0x{:04X} (stuck: {}), Halted: {}, Pixels: {}, Opcode: 0x{:02X} (LD (0x{:04X}), A=0x{:02X}), STAT:0x{:02X}", 
                            TOTAL_RENDER_LOOPS, pc, PC_STUCK_COUNT, halted, non_zero_count, opcode, addr, a_register, stat_reg);
                    } else {
                        println!("No frame change for {} loops - PC: 0x{:04X} (stuck: {}), Halted: {}, Pixels: {}, Opcode: 0x{:02X} 0x{:02X}", 
                            TOTAL_RENDER_LOOPS, pc, PC_STUCK_COUNT, halted, non_zero_count, opcode, next_byte);
                    }
                }
            }
        }
        
        

        // Draw the Game Boy screen (160x144)
        let scale = 4.0;
        for y in 0..144 {
            for x in 0..160 {
                let pixel = frame_buffer[y * 160 + x];
                // Original Game Boy green monochrome colors
                let color = match pixel {
                    0 => Color::new(0.616, 0.733, 0.059, 1.0), // Lightest green
                    1 => Color::new(0.541, 0.675, 0.059, 1.0), // Light green
                    2 => Color::new(0.188, 0.384, 0.188, 1.0), // Dark green
                    3 => Color::new(0.063, 0.247, 0.063, 1.0), // Darkest green
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
        

        // Display FPS  
        let fps = get_fps();
        let fps_text = format!("Game Boy Emulator - FPS: {:.1}", fps);
        draw_text(&fps_text, 10.0, screen_height() - 20.0, 20.0, WHITE);
        
        #[cfg(debug_assertions)]
        {
            let pc = emulator.cpu.pc;
            let halted = emulator.cpu.halted;
            let ly = emulator.cpu.mmap.read(0xFF44); // Current scanline
            let lcdc = emulator.cpu.mmap.read(0xFF40); // LCD control
            let ppu_ly = emulator.cpu.mmap.get_ppu().ly;
            let lcd_enabled = (lcdc & 0x80) != 0;
            let debug_text = format!("PC: 0x{:04X} | LY: {} | PPU_LY: {} | LCD: {} | Halted: {}", 
                pc, ly, ppu_ly, lcd_enabled, halted);
            draw_text(&debug_text, 10.0, screen_height() - 40.0, 16.0, WHITE);
            
            unsafe {
                let changes_text = format!("Frame Changes: {} | LCDC: 0x{:02X}", FRAME_CHANGE_COUNT, lcdc);
                draw_text(&changes_text, 10.0, screen_height() - 60.0, 16.0, WHITE);
            }
        }

        next_frame().await
    }
}
