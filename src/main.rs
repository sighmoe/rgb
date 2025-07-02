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
    // Set target FPS to 60 (matching Game Boy refresh rate)
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

        // Run emulator for one frame (approximately 1/60th of a second)
        // Game Boy CPU runs at ~4.194 MHz, so we need ~69,905 cycles per frame at 60fps
        const CYCLES_PER_FRAME: u32 = 69905;
        let mut cycles_executed = 0;
        
        while cycles_executed < CYCLES_PER_FRAME {
            // Write trace before decoding/executing instruction
            #[cfg(debug_assertions)]
            emulator.write_trace();
            
            let instruction = emulator.cpu.decode();
            let cycles = emulator.cpu.execute(instruction);
            
            // Handle EI delay after instruction execution
            emulator.cpu.handle_ei_delay();
            
            // Handle interrupts
            if emulator.cpu.check_interrupts() {
                let interrupt_cycles = emulator.cpu.handle_interrupt();
                cycles_executed += interrupt_cycles as u32;
            }
            
            // If CPU is halted but an interrupt is pending, wake up
            if emulator.cpu.halted && emulator.cpu.check_pending_interrupts() {
                emulator.cpu.halted = false;
            }
            
            // If halted, advance timer and PPU but don't execute instructions
            if emulator.cpu.halted {
                let halt_cycles = 4u16; // Advance 4 cycles while halted
                
                // Step timer and check for timer interrupt
                if emulator.cpu.mmap.step_timer(halt_cycles) {
                    emulator.cpu.request_timer_interrupt();
                }
                
                // Step PPU and check for PPU interrupts
                let (vblank_interrupt, stat_interrupt) = emulator.cpu.mmap.step_ppu(halt_cycles);
                if vblank_interrupt {
                    emulator.cpu.request_vblank_interrupt();
                }
                if stat_interrupt {
                    emulator.cpu.request_lcd_stat_interrupt();
                }
                
                cycles_executed += halt_cycles as u32;
            } else {
                // Step timer for instruction cycles
                if emulator.cpu.mmap.step_timer(cycles as u16) {
                    emulator.cpu.request_timer_interrupt();
                }
                
                // Step PPU for instruction cycles
                let (vblank_interrupt, stat_interrupt) = emulator.cpu.mmap.step_ppu(cycles as u16);
                if vblank_interrupt {
                    emulator.cpu.request_vblank_interrupt();
                }
                if stat_interrupt {
                    emulator.cpu.request_lcd_stat_interrupt();
                }
                
                cycles_executed += cycles as u32;
            }
        }
        
        
        
        // Get frame buffer from PPU
        let frame_buffer = emulator.get_frame_buffer();
        

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
        

        // Display FPS and emulator info
        let fps = get_fps();
        let fps_text = format!("Game Boy Emulator - FPS: {:.1}", fps);
        draw_text(&fps_text, 10.0, screen_height() - 20.0, 20.0, WHITE);

        next_frame().await
    }
}
