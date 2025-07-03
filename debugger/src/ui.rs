use macroquad::prelude::*;
use crate::core::{Debugger, DebuggerState, CpuSnapshot};

const DEBUGGER_WINDOW_WIDTH: f32 = 400.0;
const DEBUGGER_WINDOW_HEIGHT: f32 = 600.0;
const BUTTON_WIDTH: f32 = 80.0;
const BUTTON_HEIGHT: f32 = 30.0;
const PADDING: f32 = 10.0;

pub struct DebuggerUI {
    pub show: bool,
    pub input_buffer: String,
    pub memory_address_input: String,
    pub breakpoint_input: String,
    pub window_pos: Vec2,
}

impl DebuggerUI {
    pub fn new() -> Self {
        Self {
            show: true,
            input_buffer: String::new(),
            memory_address_input: String::new(),
            breakpoint_input: String::new(),
            window_pos: Vec2::new(650.0, 50.0),
        }
    }
    
    pub fn draw(&mut self, debugger: &mut Debugger) {
        if !self.show {
            return;
        }
        
        let x = self.window_pos.x;
        let y = self.window_pos.y;
        
        // Background
        draw_rectangle(x, y, DEBUGGER_WINDOW_WIDTH, DEBUGGER_WINDOW_HEIGHT, Color::new(0.2, 0.2, 0.2, 0.9));
        draw_rectangle_lines(x, y, DEBUGGER_WINDOW_WIDTH, DEBUGGER_WINDOW_HEIGHT, 2.0, WHITE);
        
        let mut current_y = y + PADDING;
        
        // Title
        draw_text("Game Boy Debugger", x + PADDING, current_y + 20.0, 20.0, WHITE);
        current_y += 40.0;
        
        // State display
        let state_text = match debugger.state {
            DebuggerState::Running => "State: Running",
            DebuggerState::Paused => "State: Paused", 
            DebuggerState::Stepping => "State: Stepping",
        };
        draw_text(state_text, x + PADDING, current_y, 16.0, YELLOW);
        current_y += 25.0;
        
        // Step count
        let step_text = format!("Steps: {}", debugger.step_count);
        draw_text(&step_text, x + PADDING, current_y, 16.0, WHITE);
        current_y += 30.0;
        
        // Control buttons
        let button_y = current_y;
        
        // Step button
        if self.draw_button("Step", x + PADDING, button_y, BUTTON_WIDTH, BUTTON_HEIGHT) {
            debugger.step_one();
        }
        
        // Pause button
        if self.draw_button("Pause", x + PADDING + BUTTON_WIDTH + 5.0, button_y, BUTTON_WIDTH, BUTTON_HEIGHT) {
            debugger.pause();
        }
        
        // Resume button
        if self.draw_button("Resume", x + PADDING + (BUTTON_WIDTH + 5.0) * 2.0, button_y, BUTTON_WIDTH, BUTTON_HEIGHT) {
            debugger.resume();
        }
        
        current_y += BUTTON_HEIGHT + 20.0;
        
        // Step N instructions input
        draw_text("Step N instructions:", x + PADDING, current_y, 16.0, WHITE);
        current_y += 20.0;
        
        // Input field for step count (simplified - in real implementation would use proper input handling)
        draw_rectangle(x + PADDING, current_y, 100.0, 25.0, DARKGRAY);
        draw_rectangle_lines(x + PADDING, current_y, 100.0, 25.0, 1.0, WHITE);
        draw_text(&self.input_buffer, x + PADDING + 5.0, current_y + 17.0, 16.0, WHITE);
        
        if self.draw_button("Go", x + PADDING + 110.0, current_y - 2.0, 40.0, 25.0) {
            if let Ok(count) = self.input_buffer.parse::<u64>() {
                debugger.step_multiple(count);
                self.input_buffer.clear();
            }
        }
        
        current_y += 40.0;
        
        // CPU Registers display
        if let Some(ref snapshot) = debugger.current_snapshot {
            draw_text("CPU Registers:", x + PADDING, current_y, 16.0, YELLOW);
            current_y += 20.0;
            
            let reg_text = format!(
                "A: {:02X}  F: {:02X}  AF: {:04X}\nB: {:02X}  C: {:02X}  BC: {:04X}\nD: {:02X}  E: {:02X}  DE: {:04X}\nH: {:02X}  L: {:02X}  HL: {:04X}",
                snapshot.a, snapshot.f, ((snapshot.a as u16) << 8) | snapshot.f as u16,
                snapshot.b, snapshot.c, ((snapshot.b as u16) << 8) | snapshot.c as u16,
                snapshot.d, snapshot.e, ((snapshot.d as u16) << 8) | snapshot.e as u16,
                snapshot.h, snapshot.l, ((snapshot.h as u16) << 8) | snapshot.l as u16
            );
            
            for (i, line) in reg_text.lines().enumerate() {
                draw_text(line, x + PADDING, current_y + i as f32 * 18.0, 14.0, WHITE);
            }
            current_y += 80.0;
            
            // PC and SP
            let pc_sp_text = format!("PC: {:04X}  SP: {:04X}", snapshot.pc, snapshot.sp);
            draw_text(&pc_sp_text, x + PADDING, current_y, 16.0, LIME);
            current_y += 25.0;
            
            // Flags
            let flags_text = format!(
                "Flags: Z:{} N:{} H:{} C:{}",
                if snapshot.zero_flag { "1" } else { "0" },
                if snapshot.subtract_flag { "1" } else { "0" },
                if snapshot.half_carry_flag { "1" } else { "0" },
                if snapshot.carry_flag { "1" } else { "0" }
            );
            draw_text(&flags_text, x + PADDING, current_y, 14.0, ORANGE);
            current_y += 25.0;
            
            // IME and HALT status
            let status_text = format!("IME: {}  HALT: {}", 
                if snapshot.ime { "ON" } else { "OFF" },
                if snapshot.halted { "YES" } else { "NO" }
            );
            draw_text(&status_text, x + PADDING, current_y, 14.0, SKYBLUE);
            current_y += 30.0;
        }
        
        // Memory inspection
        draw_text("Memory Inspector:", x + PADDING, current_y, 16.0, YELLOW);
        current_y += 20.0;
        
        // Memory address input
        draw_text("Address (hex):", x + PADDING, current_y, 14.0, WHITE);
        current_y += 18.0;
        
        draw_rectangle(x + PADDING, current_y, 80.0, 25.0, DARKGRAY);
        draw_rectangle_lines(x + PADDING, current_y, 80.0, 25.0, 1.0, WHITE);
        draw_text(&self.memory_address_input, x + PADDING + 5.0, current_y + 17.0, 14.0, WHITE);
        
        current_y += 35.0;
        
        // Breakpoints section
        draw_text("Breakpoints:", x + PADDING, current_y, 16.0, YELLOW);
        current_y += 20.0;
        
        // Breakpoint input
        draw_rectangle(x + PADDING, current_y, 80.0, 25.0, DARKGRAY);
        draw_rectangle_lines(x + PADDING, current_y, 80.0, 25.0, 1.0, WHITE);
        draw_text(&self.breakpoint_input, x + PADDING + 5.0, current_y + 17.0, 14.0, WHITE);
        
        if self.draw_button("Add", x + PADDING + 90.0, current_y - 2.0, 40.0, 25.0) {
            if let Ok(addr) = u16::from_str_radix(&self.breakpoint_input, 16) {
                debugger.add_breakpoint(addr);
                self.breakpoint_input.clear();
            }
        }
        
        current_y += 35.0;
        
        // List breakpoints
        for (i, &bp) in debugger.breakpoints.iter().enumerate() {
            let bp_text = format!("${:04X}", bp);
            draw_text(&bp_text, x + PADDING, current_y + i as f32 * 18.0, 14.0, RED);
            
            if self.draw_button("X", x + PADDING + 60.0, current_y + i as f32 * 18.0 - 5.0, 20.0, 15.0) {
                debugger.remove_breakpoint(bp);
                break;
            }
        }
    }
    
    fn draw_button(&self, text: &str, x: f32, y: f32, width: f32, height: f32) -> bool {
        let mouse_pos = mouse_position();
        let is_hovered = mouse_pos.0 >= x && mouse_pos.0 <= x + width && 
                        mouse_pos.1 >= y && mouse_pos.1 <= y + height;
        
        let color = if is_hovered { LIGHTGRAY } else { GRAY };
        draw_rectangle(x, y, width, height, color);
        draw_rectangle_lines(x, y, width, height, 1.0, WHITE);
        
        let text_x = x + (width - text.len() as f32 * 6.0) / 2.0;
        let text_y = y + height / 2.0 + 5.0;
        draw_text(text, text_x, text_y, 16.0, BLACK);
        
        is_hovered && is_mouse_button_pressed(MouseButton::Left)
    }
    
    pub fn handle_input(&mut self) {
        // Handle keyboard input for text fields
        // This is a simplified implementation - in a real debugger you'd want proper input handling
        
        // Toggle debugger visibility with F1
        if is_key_pressed(KeyCode::F1) {
            self.show = !self.show;
        }
        
        // Handle number input for step count (simplified)
        for key in [KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
                   KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9] {
            if is_key_pressed(key) {
                let digit = match key {
                    KeyCode::Key0 => '0',
                    KeyCode::Key1 => '1',
                    KeyCode::Key2 => '2',
                    KeyCode::Key3 => '3',
                    KeyCode::Key4 => '4',
                    KeyCode::Key5 => '5',
                    KeyCode::Key6 => '6',
                    KeyCode::Key7 => '7',
                    KeyCode::Key8 => '8',
                    KeyCode::Key9 => '9',
                    _ => continue,
                };
                if self.input_buffer.len() < 10 {
                    self.input_buffer.push(digit);
                }
            }
        }
        
        // Backspace for input buffer
        if is_key_pressed(KeyCode::Backspace) {
            self.input_buffer.pop();
        }
        
        // Quick debugger controls
        if is_key_pressed(KeyCode::F5) {
            // F5 = Step one instruction (like Visual Studio)
        }
        
        if is_key_pressed(KeyCode::F9) {
            // F9 = Toggle breakpoint at current PC
        }
    }
    
    pub fn update_cpu_snapshot(&mut self, debugger: &mut Debugger, 
                              a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, h: u8, l: u8,
                              pc: u16, sp: u16, ime: bool, halted: bool,
                              zero_flag: bool, subtract_flag: bool, half_carry_flag: bool, carry_flag: bool) {
        debugger.current_snapshot = Some(CpuSnapshot {
            a, b, c, d, e, f, h, l, pc, sp, ime, halted,
            zero_flag, subtract_flag, half_carry_flag, carry_flag,
        });
    }
}