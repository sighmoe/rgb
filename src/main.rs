mod rgb;

use macroquad::prelude::*;
use rgb::cpu::Cpu;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{Duration, Instant};
use debugger::{Debugger, DebuggerUI};

struct GameBoyEmulator {
    cpu: Cpu,
    #[cfg(debug_assertions)]
    trace_writer: Option<BufWriter<File>>,
    #[cfg(debug_assertions)]
    trace_json: bool,
    #[cfg(debug_assertions)]
    instruction_count: u64,
    debugger: Option<Debugger>,
    debugger_ui: Option<DebuggerUI>,
}

impl GameBoyEmulator {
    fn new(rom_path: &str, skip_boot_rom: bool, trace_file: Option<String>, trace_json: bool, enable_debugger: bool) -> Self {
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
        
        let (debugger, debugger_ui) = if enable_debugger {
            (Some(Debugger::new()), Some(DebuggerUI::new()))
        } else {
            (None, None)
        };
        
        Self { 
            cpu,
            #[cfg(debug_assertions)]
            trace_writer,
            #[cfg(debug_assertions)]
            trace_json,
            #[cfg(debug_assertions)]
            instruction_count: 0,
            debugger,
            debugger_ui,
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
    let mut enable_debugger = false;
    
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
            "--debug" | "-d" => {
                enable_debugger = true;
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
                println!("  --debug, -d          Enable interactive debugger");
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
    
    let mut emulator = GameBoyEmulator::new(rom_path, skip_boot_rom, trace_file, trace_json, enable_debugger);
    
    // Game Boy timing constants
    const TARGET_FPS: f64 = 59.7; // Game Boy's actual refresh rate is ~59.7 Hz
    const FRAME_DURATION: Duration = Duration::from_nanos((1_000_000_000.0 / TARGET_FPS) as u64);
    
    let mut last_frame_time = Instant::now();
    
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
        

        // Run emulator until PPU completes a full frame (VBlank occurs)
        // Game Boy runs at ~4.194 MHz, with ~69,905 cycles per frame at 59.7 FPS
        let mut instructions_executed = 0;
        let mut total_cycles = 0;
        let mut loop_iterations = 0;
        const CYCLES_PER_FRAME: u32 = 70224; // 4.194 MHz / 59.7 FPS = ~70,224 cycles per frame
        let max_instructions_per_frame = 30000; // Reduced to make frames more reasonable
        let max_loop_iterations = 200000; // Safety limit for total loop iterations including HALT cycles
        
        loop {
            loop_iterations += 1;
            
            // Safety check to prevent infinite loops
            if instructions_executed >= max_instructions_per_frame {
                #[cfg(debug_assertions)]
                println!("SAFETY BREAK: Hit instruction limit {} after {} loop iterations", 
                    max_instructions_per_frame, loop_iterations);
                break;
            }
            
            if loop_iterations >= max_loop_iterations {
                #[cfg(debug_assertions)]
                println!("SAFETY BREAK: Hit loop iteration limit {} with {} instructions executed", 
                    max_loop_iterations, instructions_executed);
                break;
            }
            // Handle debugger logic (optimized for performance)
            let mut debugger_paused = false;
            if let Some(ref mut debugger) = emulator.debugger {
                // Check for breakpoints first (fastest check)
                if debugger.check_breakpoint(emulator.cpu.pc) {
                    debugger.pause();
                }
                
                // Check if we should execute this instruction
                if !debugger.should_execute() {
                    debugger_paused = true;
                    // Don't break here - continue with frame completion but skip instruction execution
                }
            }
            
            if emulator.cpu.halted {
                // When halted, still step the hardware
                let halt_cycles = 4u16;
                total_cycles += halt_cycles as u32;
                
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
                }
            } else if !debugger_paused {
                // Execute one instruction (only if debugger allows it)
                instructions_executed += 1; // Only count real instructions, not HALT loops
                let instruction = emulator.cpu.decode();
                
                // Record instruction in debugger
                if let Some(ref mut debugger) = emulator.debugger {
                    debugger.record_instruction(emulator.cpu.pc, instruction.instr);
                }
                
                
                
                let cycles = emulator.cpu.execute(instruction);
                total_cycles += cycles as u32;
                
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
                    break; // Frame complete, exit instruction loop
                }
                if stat_interrupt {
                    emulator.cpu.request_lcd_stat_interrupt();
                }
                
                // Alternative: Exit if we've consumed enough cycles for one frame
                // This can help prevent frames from running too long
                if total_cycles >= CYCLES_PER_FRAME {
                    #[cfg(debug_assertions)]
                    println!("CYCLE LIMIT: Completed frame with {} cycles", total_cycles);
                    break;
                }
            } else {
                // Debugger is paused - still step hardware to prevent lockup but don't execute instructions
                let pause_cycles = 4u16; // Minimal cycles to keep hardware running
                
                if emulator.cpu.mmap.step_timer(pause_cycles) {
                    emulator.cpu.request_timer_interrupt();
                }
                
                let (vblank_interrupt, stat_interrupt) = emulator.cpu.mmap.step_ppu(pause_cycles);
                if vblank_interrupt {
                    emulator.cpu.request_vblank_interrupt();
                    break; // Still complete frame even when paused
                }
                if stat_interrupt {
                    emulator.cpu.request_lcd_stat_interrupt();
                }
            }
        }
        
        // Basic frame completion debug output
        #[cfg(debug_assertions)]
        {
            static mut FRAME_COUNT: u32 = 0;
            unsafe {
                FRAME_COUNT += 1;
                if FRAME_COUNT <= 3 || FRAME_COUNT % 120 == 0 {
                    println!("Frame #{}: {} instructions, {} cycles", FRAME_COUNT, instructions_executed, total_cycles);
                }
            }
        }
        
        // Update debugger state once per frame (moved outside hot loop for performance)
        if let Some(ref mut debugger) = emulator.debugger {
            if let Some(ref mut ui) = emulator.debugger_ui {
                ui.update_cpu_snapshot(
                    debugger,
                    emulator.cpu.registers.a,
                    emulator.cpu.registers.b,
                    emulator.cpu.registers.c,
                    emulator.cpu.registers.d,
                    emulator.cpu.registers.e,
                    u8::from(emulator.cpu.registers.f),
                    emulator.cpu.registers.h,
                    emulator.cpu.registers.l,
                    emulator.cpu.pc,
                    emulator.cpu.sp,
                    emulator.cpu.ime,
                    emulator.cpu.halted,
                    emulator.cpu.registers.f.zero,
                    emulator.cpu.registers.f.subtract,
                    emulator.cpu.registers.f.half_carry,
                    emulator.cpu.registers.f.carry,
                );
            }
            
            // Update memory watches once per frame
            debugger.update_memory_watches(|addr| emulator.cpu.mmap.read(addr));
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
        

        // Display FPS and timing info
        let fps = get_fps();
        let frame_elapsed = last_frame_time.elapsed();
        let fps_text = format!("Game Boy Emulator - FPS: {:.1} | Frame time: {:.1}ms", 
            fps, frame_elapsed.as_secs_f32() * 1000.0);
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
            
        }

        // Handle debugger UI
        if let (Some(ref mut debugger), Some(ref mut debugger_ui)) = (&mut emulator.debugger, &mut emulator.debugger_ui) {
            debugger_ui.handle_input();
            debugger_ui.draw(debugger);
        }

        // Frame timing control - track timing but let macroquad handle frame limiting
        let frame_elapsed = last_frame_time.elapsed();
        last_frame_time = Instant::now();

        next_frame().await
    }
}
