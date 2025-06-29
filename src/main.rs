mod rgb;

use macroquad::prelude::*;
use rgb::{cpu::Cpu, memory::MemoryMap, registers::Registers};

struct GameBoyEmulator {
    cpu: Cpu,
}

impl GameBoyEmulator {
    fn new() -> Self {
        let mut mmap = MemoryMap::new();
        // Load bootstrap ROM
        mmap.load_bootstrap();
        
        Self {
            cpu: Cpu {
                registers: Registers::new(),
                pc: 0x0000,  // Start at bootstrap ROM
                sp: 0xFFFE,  // Initial stack pointer
                mmap,
                halted: false,
            }
        }
    }

    fn step(&mut self) {
        if !self.cpu.halted {
            // Try to execute instruction, but don't crash on unknown opcodes
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let instruction = self.cpu.decode();
                self.cpu.execute(instruction);
            })) {
                Ok(()) => {}, // Instruction executed successfully
                Err(_) => {
                    // Unknown opcode, just increment PC and continue
                    self.cpu.pc = self.cpu.pc.wrapping_add(1);
                }
            }
        }
        
        // Step PPU with CPU cycles (simplified: assume 4 cycles per instruction)
        self.cpu.mmap.step_ppu(4);
    }

    fn get_frame_buffer(&self) -> &[u8] {
        self.cpu.mmap.get_ppu().get_frame_buffer()
    }
}

#[macroquad::main("Game Boy Emulator")]
async fn main() {
    let mut emulator = GameBoyEmulator::new();
    
    loop {
        clear_background(BLACK);

        // Run emulator steps
        for _ in 0..1000 {  // Run multiple steps per frame
            emulator.step();
        }

        // Get frame buffer from PPU
        let frame_buffer = emulator.get_frame_buffer();
        
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
