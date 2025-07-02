use crate::rgb::instructions::{Instruction, InstructionKind, decode_instruction, get_instruction_size, decode_cb_instruction, get_cb_instruction_size, JumpCondition};
use crate::rgb::instruction_timing::get_instruction_cycles;
use crate::rgb::memory::MemoryMap;
use crate::rgb::registers::Registers;

// Interrupt vector addresses
const VBLANK_VECTOR: u16 = 0x0040;
const LCD_STAT_VECTOR: u16 = 0x0048;
const TIMER_VECTOR: u16 = 0x0050;
const SERIAL_VECTOR: u16 = 0x0058;
const JOYPAD_VECTOR: u16 = 0x0060;

// Interrupt register addresses
const IE_REGISTER: u16 = 0xFFFF;  // Interrupt Enable
const IF_REGISTER: u16 = 0xFF0F;  // Interrupt Flag

// Interrupt bit positions
const VBLANK_BIT: u8 = 0;
const LCD_STAT_BIT: u8 = 1;
const TIMER_BIT: u8 = 2;
const SERIAL_BIT: u8 = 3;
const JOYPAD_BIT: u8 = 4;

pub struct Cpu {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub mmap: MemoryMap,
    pub halted: bool,
    // Interrupt handling
    pub ime: bool,        // Interrupt Master Enable
    pub ei_delay: bool,   // EI instruction has 1-instruction delay
    pub halt_bug: bool,   // HALT bug state for next instruction
}

impl Cpu {
    #[allow(dead_code)] // Public API method
    pub fn new() -> Self {
        let registers = Registers::new();
        let mut mmap = MemoryMap::new();
        mmap.load_bootstrap();

        Cpu {
            registers,
            pc: 0,
            sp: 0,
            mmap,
            halted: false,
            ime: false,      // Interrupts disabled on startup
            ei_delay: false, // No EI delay initially
            halt_bug: false, // No HALT bug initially
        }
    }

    /// Creates a new CPU in the state that would exist after the boot ROM completes
    /// This allows skipping the boot sequence and starting directly with cartridge execution
    #[allow(dead_code)] // Public API method
    pub fn new_post_boot() -> Self {
        let registers = Registers::new_post_boot();
        let mmap = MemoryMap::new_post_boot();

        Cpu {
            registers,
            pc: 0x0100,     // Cartridge entry point
            sp: 0xFFFE,     // Stack pointer at top of RAM
            mmap,
            halted: false,
            ime: false,     // Interrupts disabled after boot
            ei_delay: false,
            halt_bug: false,
        }
    }

    pub fn decode(&mut self) -> Instruction {
        let opcode = self.mmap.read(self.pc);
        let mut instruction = Instruction::default();
        instruction.instr = opcode;
        
        // Handle CB-prefixed instructions
        if opcode == 0xCB {
            let cb_opcode = self.mmap.read(self.pc + 1);
            instruction.kind = decode_cb_instruction(cb_opcode);
            self.pc += get_cb_instruction_size();
            return instruction;
        }
        
        // Get instruction size to determine how many bytes to read
        let size = get_instruction_size(opcode);
        
        // Handle HALT bug: if in HALT bug state, don't advance PC
        let pc_increment = if self.halt_bug {
            self.halt_bug = false; // Clear flag after handling
            0 // Don't advance PC for HALT bug
        } else {
            size
        };
        
        // Read immediate values based on instruction size
        let immediate8 = if size >= 2 {
            Some(self.mmap.read(self.pc + 1))
        } else {
            None
        };
        
        let immediate16 = if size >= 3 {
            let low_byte = self.mmap.read(self.pc + 1);
            let high_byte = self.mmap.read(self.pc + 2);
            Some((high_byte as u16) << 8 | low_byte as u16)
        } else {
            None
        };
        
        // Decode the instruction using the new modular system
        instruction.kind = decode_instruction(opcode, immediate8, immediate16);
        
        // Advance PC by instruction size (or 0 for HALT bug)
        self.pc += pc_increment;
        
        instruction
    }

    pub fn execute(&mut self, instruction: Instruction) -> u8 {
        
        // Calculate cycles for conditional instructions before moving the instruction
        let condition_taken = match &instruction.kind {
            InstructionKind::JP(condition, _) => {
                *condition == JumpCondition::Always || self.check_jump_condition(*condition)
            }
            InstructionKind::JR(condition, _) => {
                *condition == JumpCondition::Always || self.check_jump_condition(*condition)
            }
            InstructionKind::CALL_COND(condition, _) => {
                self.check_jump_condition(*condition)
            }
            InstructionKind::RET_COND(condition) => {
                self.check_jump_condition(*condition)
            }
            _ => false, // Non-conditional instructions
        };
        
        let actual_cycles = get_instruction_cycles(&instruction.kind, condition_taken);
        
        // Execute the instruction using the modular execution system
        let _cycles = self.execute_instruction(instruction);
        
        // Step timer and check for timer interrupt
        if self.mmap.step_timer(actual_cycles as u16) {
            self.request_timer_interrupt();
        }
        
        // Step PPU and check for PPU interrupts
        let (vblank_interrupt, stat_interrupt) = self.mmap.step_ppu(actual_cycles as u16);
        if vblank_interrupt {
            self.request_vblank_interrupt();
        }
        if stat_interrupt {
            self.request_lcd_stat_interrupt();
        }
        
        actual_cycles
    }

    pub fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    pub fn add16(&mut self, dest_value: u16, src_value: u16) -> u16 {
        let (new_value, did_overflow) = dest_value.overflowing_add(src_value);
        // 16-bit ADD affects flags differently than 8-bit:
        // Z: Not affected (keep current value)
        // N: Reset to 0
        // H: Set if carry from bit 11 to bit 12
        // C: Set if carry from bit 15 to bit 16 (overflow)
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half carry for 16-bit: carry from bit 11 (0x0FFF mask)
        self.registers.f.half_carry = (dest_value & 0x0FFF) + (src_value & 0x0FFF) > 0x0FFF;
        new_value
    }
    
    pub fn increment(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // Half carry occurs if bit 3 carried to bit 4
        self.registers.f.half_carry = (value & 0x0F) == 0x0F;
        // Carry flag is not affected by INC
        new_value
    }
    
    pub fn decrement(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        // Half carry occurs if bit 4 borrowed from bit 3
        self.registers.f.half_carry = (value & 0x0F) == 0x00;
        // Carry flag is not affected by DEC
        new_value
    }
    
    pub fn subtract(&mut self, value: u8) -> u8 {
        let result = self.registers.a.wrapping_sub(value);
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F);
        self.registers.f.carry = self.registers.a < value;
        result
    }
    
    pub fn subtract_with_carry(&mut self, value: u8) -> u8 {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let temp = value.wrapping_add(carry);
        let result = self.registers.a.wrapping_sub(temp);
        
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < ((value & 0x0F) + carry);
        self.registers.f.carry = (self.registers.a as u16) < (value as u16 + carry as u16);
        result
    }
    
    pub fn test_bit(&mut self, bit: u8, value: u8) {
        let bit_set = (value & (1 << bit)) != 0;
        self.registers.f.zero = !bit_set;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        // Carry flag is not affected by BIT
    }
    
    pub fn check_jump_condition(&self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::Always => true,
            JumpCondition::Zero => self.registers.f.zero,
            JumpCondition::NotZero => !self.registers.f.zero,
            JumpCondition::Carry => self.registers.f.carry,
            JumpCondition::NotCarry => !self.registers.f.carry,
        }
    }
    
    pub fn push_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.mmap.write(self.sp, ((value & 0xFF00) >> 8) as u8); // High byte
        self.sp = self.sp.wrapping_sub(1);
        self.mmap.write(self.sp, (value & 0x00FF) as u8); // Low byte
    }
    
    pub fn pop_stack(&mut self) -> u16 {
        let low = self.mmap.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let high = self.mmap.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let result = (high << 8) | low;
        result
    }
    
    // Interrupt handling methods
    pub fn check_interrupts(&mut self) -> bool {
        if !self.ime {
            return false;
        }
        
        let ie = self.mmap.read(IE_REGISTER);
        let if_reg = self.mmap.read(IF_REGISTER);
        let pending = ie & if_reg & 0x1F; // Only check lower 5 bits
        
        pending != 0
    }
    
    // Check for pending interrupts regardless of IME (for HALT bug)
    pub fn check_pending_interrupts(&mut self) -> bool {
        let ie = self.mmap.read(IE_REGISTER);
        let if_reg = self.mmap.read(IF_REGISTER);
        let pending = ie & if_reg & 0x1F; // Only check lower 5 bits
        
        pending != 0
    }
    
    pub fn handle_interrupt(&mut self) -> u8 {
        if !self.ime {
            return 0;
        }
        
        let ie = self.mmap.read(IE_REGISTER);
        let if_reg = self.mmap.read(IF_REGISTER);
        let pending = ie & if_reg & 0x1F;
        
        if pending == 0 {
            return 0;
        }
        
        
        // Find highest priority interrupt (lowest bit number)
        let interrupt_bit = if pending & (1 << VBLANK_BIT) != 0 {
            VBLANK_BIT
        } else if pending & (1 << LCD_STAT_BIT) != 0 {
            LCD_STAT_BIT
        } else if pending & (1 << TIMER_BIT) != 0 {
            TIMER_BIT
        } else if pending & (1 << SERIAL_BIT) != 0 {
            SERIAL_BIT
        } else if pending & (1 << JOYPAD_BIT) != 0 {
            JOYPAD_BIT
        } else {
            return 0; // No interrupt found
        };
        
        // Clear the interrupt flag
        let new_if = if_reg & !(1 << interrupt_bit);
        self.mmap.write(IF_REGISTER, new_if);
        
        // Disable interrupts
        self.ime = false;
        
        // Wake up from HALT if halted
        self.halted = false;
        
        // Push current PC to stack (takes 2 cycles)
        self.push_stack(self.pc);
        
        // Jump to interrupt vector
        let vector = match interrupt_bit {
            VBLANK_BIT => VBLANK_VECTOR,
            LCD_STAT_BIT => LCD_STAT_VECTOR,
            TIMER_BIT => TIMER_VECTOR,
            SERIAL_BIT => SERIAL_VECTOR,
            JOYPAD_BIT => JOYPAD_VECTOR,
            _ => unreachable!(),
        };
        
        
        self.pc = vector;
        
        // Interrupt handling takes 5 M-cycles (20 T-cycles)
        20
    }
    
    pub fn request_interrupt(&mut self, interrupt_bit: u8) {
        let if_reg = self.mmap.read(IF_REGISTER);
        self.mmap.write(IF_REGISTER, if_reg | (1 << interrupt_bit));
    }
    
    // Helper methods for requesting specific interrupts
    pub fn request_vblank_interrupt(&mut self) {
        #[cfg(debug_assertions)]
        {
            use log::debug;
            static mut VBLANK_COUNT: u32 = 0;
            unsafe {
                VBLANK_COUNT += 1;
                if VBLANK_COUNT <= 10 {
                    debug!("VBLANK interrupt requested #{}", VBLANK_COUNT);
                }
            }
        }
        self.request_interrupt(VBLANK_BIT);
    }
    
    pub fn request_lcd_stat_interrupt(&mut self) {
        #[cfg(debug_assertions)]
        {
            use log::debug;
            static mut STAT_COUNT: u32 = 0;
            unsafe {
                STAT_COUNT += 1;
                if STAT_COUNT <= 10 {
                    debug!("LCD STAT interrupt requested #{}", STAT_COUNT);
                }
            }
        }
        self.request_interrupt(LCD_STAT_BIT);
    }
    
    pub fn request_timer_interrupt(&mut self) {
        #[cfg(debug_assertions)]
        {
            use log::debug;
            static mut TIMER_INT_COUNT: u32 = 0;
            unsafe {
                TIMER_INT_COUNT += 1;
                if TIMER_INT_COUNT <= 10 {
                    debug!("TIMER interrupt requested #{}", TIMER_INT_COUNT);
                }
            }
        }
        self.request_interrupt(TIMER_BIT);
    }
    
    pub fn request_serial_interrupt(&mut self) {
        self.request_interrupt(SERIAL_BIT);
    }
    
    pub fn request_joypad_interrupt(&mut self) {
        self.request_interrupt(JOYPAD_BIT);
    }
    
    pub fn handle_ei_delay(&mut self) {
        if self.ei_delay {
            self.ime = true;
            self.ei_delay = false;
        }
    }
}

