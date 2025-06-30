use crate::rgb::instructions::{Instruction, InstructionKind, ArgKind, decode_instruction, get_instruction_size, decode_cb_instruction, get_cb_instruction_size, JumpCondition};
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
        
        // Advance PC by instruction size
        self.pc += size;
        
        instruction
    }

    pub fn execute(&mut self, instruction: Instruction) -> u8 {
        match instruction.kind {
            InstructionKind::NOP => {
                // Do nothing
            }
            InstructionKind::LD(dest, src) => {
                match (dest, src) {
                    // 16-bit loads
                    (ArgKind::BC, ArgKind::Immediate16(value)) => {
                        self.registers.set_bc(value);
                    }
                    (ArgKind::DE, ArgKind::Immediate16(value)) => {
                        self.registers.set_de(value);
                    }
                    (ArgKind::HL, ArgKind::Immediate16(value)) => {
                        self.registers.set_hl(value);
                    }
                    (ArgKind::SP, ArgKind::Immediate16(value)) => {
                        self.sp = value;
                    }
                    // 8-bit immediate loads
                    (ArgKind::A, ArgKind::Immediate(value)) => {
                        self.registers.a = value;
                    }
                    (ArgKind::B, ArgKind::Immediate(value)) => {
                        self.registers.b = value;
                    }
                    (ArgKind::C, ArgKind::Immediate(value)) => {
                        self.registers.c = value;
                    }
                    (ArgKind::D, ArgKind::Immediate(value)) => {
                        self.registers.d = value;
                    }
                    (ArgKind::E, ArgKind::Immediate(value)) => {
                        self.registers.e = value;
                    }
                    (ArgKind::H, ArgKind::Immediate(value)) => {
                        self.registers.h = value;
                    }
                    (ArgKind::L, ArgKind::Immediate(value)) => {
                        self.registers.l = value;
                    }
                    // Register-to-register loads
                    (ArgKind::A, ArgKind::A) => {
                        self.registers.a = self.registers.a;
                    }
                    (ArgKind::A, ArgKind::B) => {
                        self.registers.a = self.registers.b;
                    }
                    (ArgKind::A, ArgKind::C) => {
                        self.registers.a = self.registers.c;
                    }
                    (ArgKind::A, ArgKind::D) => {
                        self.registers.a = self.registers.d;
                    }
                    (ArgKind::A, ArgKind::E) => {
                        self.registers.a = self.registers.e;
                    }
                    (ArgKind::A, ArgKind::H) => {
                        self.registers.a = self.registers.h;
                    }
                    (ArgKind::A, ArgKind::L) => {
                        self.registers.a = self.registers.l;
                    }
                    (ArgKind::B, ArgKind::A) => {
                        self.registers.b = self.registers.a;
                    }
                    (ArgKind::B, ArgKind::B) => {
                        self.registers.b = self.registers.b;
                    }
                    (ArgKind::B, ArgKind::C) => {
                        self.registers.b = self.registers.c;
                    }
                    (ArgKind::B, ArgKind::D) => {
                        self.registers.b = self.registers.d;
                    }
                    (ArgKind::B, ArgKind::E) => {
                        self.registers.b = self.registers.e;
                    }
                    (ArgKind::B, ArgKind::H) => {
                        self.registers.b = self.registers.h;
                    }
                    (ArgKind::B, ArgKind::L) => {
                        self.registers.b = self.registers.l;
                    }
                    (ArgKind::C, ArgKind::A) => {
                        self.registers.c = self.registers.a;
                    }
                    (ArgKind::C, ArgKind::B) => {
                        self.registers.c = self.registers.b;
                    }
                    (ArgKind::C, ArgKind::C) => {
                        self.registers.c = self.registers.c;
                    }
                    (ArgKind::C, ArgKind::D) => {
                        self.registers.c = self.registers.d;
                    }
                    (ArgKind::C, ArgKind::E) => {
                        self.registers.c = self.registers.e;
                    }
                    (ArgKind::C, ArgKind::H) => {
                        self.registers.c = self.registers.h;
                    }
                    (ArgKind::C, ArgKind::L) => {
                        self.registers.c = self.registers.l;
                    }
                    (ArgKind::D, ArgKind::A) => {
                        self.registers.d = self.registers.a;
                    }
                    (ArgKind::D, ArgKind::B) => {
                        self.registers.d = self.registers.b;
                    }
                    (ArgKind::D, ArgKind::C) => {
                        self.registers.d = self.registers.c;
                    }
                    (ArgKind::D, ArgKind::D) => {
                        self.registers.d = self.registers.d;
                    }
                    (ArgKind::D, ArgKind::E) => {
                        self.registers.d = self.registers.e;
                    }
                    (ArgKind::D, ArgKind::H) => {
                        self.registers.d = self.registers.h;
                    }
                    (ArgKind::D, ArgKind::L) => {
                        self.registers.d = self.registers.l;
                    }
                    (ArgKind::E, ArgKind::A) => {
                        self.registers.e = self.registers.a;
                    }
                    (ArgKind::E, ArgKind::B) => {
                        self.registers.e = self.registers.b;
                    }
                    (ArgKind::E, ArgKind::C) => {
                        self.registers.e = self.registers.c;
                    }
                    (ArgKind::E, ArgKind::D) => {
                        self.registers.e = self.registers.d;
                    }
                    (ArgKind::E, ArgKind::E) => {
                        self.registers.e = self.registers.e;
                    }
                    (ArgKind::E, ArgKind::H) => {
                        self.registers.e = self.registers.h;
                    }
                    (ArgKind::E, ArgKind::L) => {
                        self.registers.e = self.registers.l;
                    }
                    (ArgKind::H, ArgKind::A) => {
                        self.registers.h = self.registers.a;
                    }
                    (ArgKind::H, ArgKind::B) => {
                        self.registers.h = self.registers.b;
                    }
                    (ArgKind::H, ArgKind::C) => {
                        self.registers.h = self.registers.c;
                    }
                    (ArgKind::H, ArgKind::D) => {
                        self.registers.h = self.registers.d;
                    }
                    (ArgKind::H, ArgKind::E) => {
                        self.registers.h = self.registers.e;
                    }
                    (ArgKind::H, ArgKind::H) => {
                        self.registers.h = self.registers.h;
                    }
                    (ArgKind::H, ArgKind::L) => {
                        self.registers.h = self.registers.l;
                    }
                    (ArgKind::L, ArgKind::A) => {
                        self.registers.l = self.registers.a;
                    }
                    (ArgKind::L, ArgKind::B) => {
                        self.registers.l = self.registers.b;
                    }
                    (ArgKind::L, ArgKind::C) => {
                        self.registers.l = self.registers.c;
                    }
                    (ArgKind::L, ArgKind::D) => {
                        self.registers.l = self.registers.d;
                    }
                    (ArgKind::L, ArgKind::E) => {
                        self.registers.l = self.registers.e;
                    }
                    (ArgKind::L, ArgKind::H) => {
                        self.registers.l = self.registers.h;
                    }
                    (ArgKind::L, ArgKind::L) => {
                        self.registers.l = self.registers.l;
                    }
                    // Special case: LD SP,HL
                    (ArgKind::SP, ArgKind::HL) => {
                        self.sp = self.registers.get_hl();
                    }
                    _ => panic!("Unsupported LD instruction variant"),
                }
            }
            InstructionKind::LDHL_SP_R8(offset) => {
                // LD HL,SP+r8 - Load HL with SP plus signed 8-bit offset
                let sp_value = self.sp as i32;
                let offset_value = offset as i32;
                let result = sp_value.wrapping_add(offset_value) as u16;
                
                // Set flags based on 8-bit arithmetic (lower byte only)
                let sp_low = (self.sp & 0xFF) as u8;
                let offset_u8 = offset as u8;
                
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                // Half carry is set if there's a carry from bit 3 to bit 4
                self.registers.f.half_carry = (sp_low & 0x0F) + (offset_u8 & 0x0F) > 0x0F;
                // Carry is set if there's a carry from bit 7 to bit 8
                self.registers.f.carry = (sp_low as u16) + (offset_u8 as u16) > 0xFF;
                
                self.registers.set_hl(result);
            }
            
            InstructionKind::LD_SP_TO_MEM(address) => {
                // LD (nn),SP - Store SP at 16-bit address (little-endian)
                let sp_low = (self.sp & 0xFF) as u8;
                let sp_high = ((self.sp >> 8) & 0xFF) as u8;
                
                // Store SP in little-endian format (low byte first, then high byte)
                self.mmap.write(address, sp_low);
                self.mmap.write(address.wrapping_add(1), sp_high);
            }
            InstructionKind::ADD(dest, src) => {
                match (dest, src) {
                    (ArgKind::A, ArgKind::A) => {
                        self.registers.a = self.add(self.registers.a);
                    }
                    (ArgKind::A, ArgKind::B) => {
                        self.registers.a = self.add(self.registers.b);
                    }
                    (ArgKind::A, ArgKind::C) => {
                        self.registers.a = self.add(self.registers.c);
                    }
                    (ArgKind::A, ArgKind::D) => {
                        self.registers.a = self.add(self.registers.d);
                    }
                    (ArgKind::A, ArgKind::E) => {
                        self.registers.a = self.add(self.registers.e);
                    }
                    (ArgKind::A, ArgKind::H) => {
                        self.registers.a = self.add(self.registers.h);
                    }
                    (ArgKind::A, ArgKind::L) => {
                        self.registers.a = self.add(self.registers.l);
                    }
                    (ArgKind::A, ArgKind::HL) => {
                        let addr = self.registers.get_hl();
                        let value = self.mmap.read(addr);
                        self.registers.a = self.add(value);
                    }
                    (ArgKind::A, ArgKind::Immediate(value)) => {
                        self.registers.a = self.add(value);
                    }
                    // 16-bit ADD instructions (ADD HL,reg16)
                    (ArgKind::HL, ArgKind::BC) => {
                        let hl_value = self.registers.get_hl();
                        let bc_value = self.registers.get_bc();
                        let result = self.add16(hl_value, bc_value);
                        self.registers.set_hl(result);
                    }
                    (ArgKind::HL, ArgKind::DE) => {
                        let hl_value = self.registers.get_hl();
                        let de_value = self.registers.get_de();
                        let result = self.add16(hl_value, de_value);
                        self.registers.set_hl(result);
                    }
                    (ArgKind::HL, ArgKind::HL) => {
                        let hl_value = self.registers.get_hl();
                        let result = self.add16(hl_value, hl_value);
                        self.registers.set_hl(result);
                    }
                    (ArgKind::HL, ArgKind::SP) => {
                        let hl_value = self.registers.get_hl();
                        let result = self.add16(hl_value, self.sp);
                        self.registers.set_hl(result);
                    }
                    _ => panic!("Unsupported ADD instruction variant"),
                }
            }
            InstructionKind::SUB(dest, src) => {
                match (dest, src) {
                    (ArgKind::A, ArgKind::A) => {
                        self.registers.a = self.subtract(self.registers.a);
                    }
                    (ArgKind::A, ArgKind::B) => {
                        self.registers.a = self.subtract(self.registers.b);
                    }
                    (ArgKind::A, ArgKind::C) => {
                        self.registers.a = self.subtract(self.registers.c);
                    }
                    (ArgKind::A, ArgKind::D) => {
                        self.registers.a = self.subtract(self.registers.d);
                    }
                    (ArgKind::A, ArgKind::E) => {
                        self.registers.a = self.subtract(self.registers.e);
                    }
                    (ArgKind::A, ArgKind::H) => {
                        self.registers.a = self.subtract(self.registers.h);
                    }
                    (ArgKind::A, ArgKind::L) => {
                        self.registers.a = self.subtract(self.registers.l);
                    }
                    (ArgKind::A, ArgKind::HL) => {
                        let addr = self.registers.get_hl();
                        let value = self.mmap.read(addr);
                        self.registers.a = self.subtract(value);
                    }
                    (ArgKind::A, ArgKind::Immediate(value)) => {
                        self.registers.a = self.subtract(value);
                    }
                    _ => panic!("Unsupported SUB instruction variant"),
                }
            }
            InstructionKind::SBC(dest, src) => {
                match (dest, src) {
                    (ArgKind::A, ArgKind::A) => {
                        self.registers.a = self.subtract_with_carry(self.registers.a);
                    }
                    (ArgKind::A, ArgKind::B) => {
                        self.registers.a = self.subtract_with_carry(self.registers.b);
                    }
                    (ArgKind::A, ArgKind::C) => {
                        self.registers.a = self.subtract_with_carry(self.registers.c);
                    }
                    (ArgKind::A, ArgKind::D) => {
                        self.registers.a = self.subtract_with_carry(self.registers.d);
                    }
                    (ArgKind::A, ArgKind::E) => {
                        self.registers.a = self.subtract_with_carry(self.registers.e);
                    }
                    (ArgKind::A, ArgKind::H) => {
                        self.registers.a = self.subtract_with_carry(self.registers.h);
                    }
                    (ArgKind::A, ArgKind::L) => {
                        self.registers.a = self.subtract_with_carry(self.registers.l);
                    }
                    (ArgKind::A, ArgKind::HL) => {
                        let addr = self.registers.get_hl();
                        let value = self.mmap.read(addr);
                        self.registers.a = self.subtract_with_carry(value);
                    }
                    (ArgKind::A, ArgKind::Immediate(value)) => {
                        self.registers.a = self.subtract_with_carry(value);
                    }
                    _ => panic!("Unsupported SBC instruction variant"),
                }
            }
            InstructionKind::INC(register) => {
                match register {
                    ArgKind::A => {
                        self.registers.a = self.increment(self.registers.a);
                    }
                    ArgKind::B => {
                        self.registers.b = self.increment(self.registers.b);
                    }
                    ArgKind::C => {
                        self.registers.c = self.increment(self.registers.c);
                    }
                    ArgKind::D => {
                        self.registers.d = self.increment(self.registers.d);
                    }
                    ArgKind::E => {
                        self.registers.e = self.increment(self.registers.e);
                    }
                    ArgKind::H => {
                        self.registers.h = self.increment(self.registers.h);
                    }
                    ArgKind::L => {
                        self.registers.l = self.increment(self.registers.l);
                    }
                    _ => panic!("Unsupported INC instruction variant"),
                }
            }
            InstructionKind::INC_MEM(addr_reg) => {
                let address = match addr_reg {
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported INC_MEM address register"),
                };
                let value = self.mmap.read(address);
                let new_value = self.increment(value);
                self.mmap.write(address, new_value);
            }
            InstructionKind::DEC(register) => {
                match register {
                    ArgKind::A => {
                        self.registers.a = self.decrement(self.registers.a);
                    }
                    ArgKind::B => {
                        self.registers.b = self.decrement(self.registers.b);
                    }
                    ArgKind::C => {
                        self.registers.c = self.decrement(self.registers.c);
                    }
                    ArgKind::D => {
                        self.registers.d = self.decrement(self.registers.d);
                    }
                    ArgKind::E => {
                        self.registers.e = self.decrement(self.registers.e);
                    }
                    ArgKind::H => {
                        self.registers.h = self.decrement(self.registers.h);
                    }
                    ArgKind::L => {
                        self.registers.l = self.decrement(self.registers.l);
                    }
                    _ => panic!("Unsupported DEC instruction variant"),
                }
            }
            InstructionKind::DEC_MEM(addr_reg) => {
                let address = match addr_reg {
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported DEC_MEM address register"),
                };
                let value = self.mmap.read(address);
                let new_value = self.decrement(value);
                self.mmap.write(address, new_value);
            }
            InstructionKind::INC16(register) => {
                match register {
                    ArgKind::BC => {
                        let value = self.registers.get_bc().wrapping_add(1);
                        self.registers.set_bc(value);
                    }
                    ArgKind::DE => {
                        let value = self.registers.get_de().wrapping_add(1);
                        self.registers.set_de(value);
                    }
                    ArgKind::HL => {
                        let value = self.registers.get_hl().wrapping_add(1);
                        self.registers.set_hl(value);
                    }
                    ArgKind::SP => {
                        self.sp = self.sp.wrapping_add(1);
                    }
                    _ => panic!("Unsupported INC16 instruction variant"),
                }
                // 16-bit increment doesn't affect flags
            }
            InstructionKind::DEC16(register) => {
                match register {
                    ArgKind::BC => {
                        let value = self.registers.get_bc().wrapping_sub(1);
                        self.registers.set_bc(value);
                    }
                    ArgKind::DE => {
                        let value = self.registers.get_de().wrapping_sub(1);
                        self.registers.set_de(value);
                    }
                    ArgKind::HL => {
                        let value = self.registers.get_hl().wrapping_sub(1);
                        self.registers.set_hl(value);
                    }
                    ArgKind::SP => {
                        self.sp = self.sp.wrapping_sub(1);
                    }
                    _ => panic!("Unsupported DEC16 instruction variant"),
                }
                // 16-bit decrement doesn't affect flags
            }
            InstructionKind::BIT(bit, register) => {
                let value = match register {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    _ => panic!("Unsupported BIT instruction variant"),
                };
                self.test_bit(bit, value);
            }
            InstructionKind::RES(bit, register) => {
                let mask = !(1 << bit);
                match register {
                    ArgKind::A => self.registers.a &= mask,
                    ArgKind::B => self.registers.b &= mask,
                    ArgKind::C => self.registers.c &= mask,
                    ArgKind::D => self.registers.d &= mask,
                    ArgKind::E => self.registers.e &= mask,
                    ArgKind::H => self.registers.h &= mask,
                    ArgKind::L => self.registers.l &= mask,
                    _ => panic!("Unsupported RES instruction variant"),
                }
            }
            InstructionKind::SET(bit, register) => {
                let mask = 1 << bit;
                match register {
                    ArgKind::A => self.registers.a |= mask,
                    ArgKind::B => self.registers.b |= mask,
                    ArgKind::C => self.registers.c |= mask,
                    ArgKind::D => self.registers.d |= mask,
                    ArgKind::E => self.registers.e |= mask,
                    ArgKind::H => self.registers.h |= mask,
                    ArgKind::L => self.registers.l |= mask,
                    _ => panic!("Unsupported SET instruction variant"),
                }
            }
            InstructionKind::JP(condition, address) => {
                if self.check_jump_condition(condition) {
                    self.pc = address;
                } else {
                    // PC was already incremented in decode, no additional action needed
                }
            }
            InstructionKind::JP_HL => {
                self.pc = self.registers.get_hl();
            }
            InstructionKind::CALL(address) => {
                self.push_stack(self.pc);
                self.pc = address;
            }
            InstructionKind::CALL_COND(condition, address) => {
                if self.check_jump_condition(condition) {
                    self.push_stack(self.pc);
                    self.pc = address;
                }
            }
            InstructionKind::RET => {
                self.pc = self.pop_stack();
            }
            InstructionKind::RET_COND(condition) => {
                if self.check_jump_condition(condition) {
                    self.pc = self.pop_stack();
                }
            }
            InstructionKind::RETI => {
                // Return from interrupt: pop PC from stack and enable interrupts
                self.pc = self.pop_stack();
                // Enable interrupts immediately (no delay like EI)
                self.ime = true;
            }
            InstructionKind::HALT => {
                self.halted = true;
            }
            InstructionKind::STOP => {
                // STOP instruction - similar to HALT but stops CPU and LCD
                self.halted = true;
            }
            
            InstructionKind::DAA => {
                // Decimal Adjust Accumulator - adjusts A for BCD arithmetic
                let mut a = self.registers.a;
                let mut correction = 0;
                let mut carry = false;
                
                if self.registers.f.half_carry || (!self.registers.f.subtract && (a & 0x0F) > 0x09) {
                    correction |= 0x06;
                }
                
                if self.registers.f.carry || (!self.registers.f.subtract && a > 0x99) {
                    correction |= 0x60;
                    carry = true;
                }
                
                if self.registers.f.subtract {
                    a = a.wrapping_sub(correction);
                } else {
                    a = a.wrapping_add(correction);
                }
                
                self.registers.a = a;
                self.registers.f.zero = a == 0;
                self.registers.f.half_carry = false;
                self.registers.f.carry = carry;
                // Subtract flag is preserved
            }
            
            InstructionKind::RLCA => {
                // Rotate A left circular
                let carry = (self.registers.a & 0x80) != 0;
                self.registers.a = (self.registers.a << 1) | (if carry { 1 } else { 0 });
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = carry;
            }
            
            InstructionKind::RRCA => {
                // Rotate A right circular
                let carry = (self.registers.a & 0x01) != 0;
                self.registers.a = (self.registers.a >> 1) | (if carry { 0x80 } else { 0 });
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = carry;
            }
            
            InstructionKind::RLA => {
                // Rotate A left through carry
                let old_carry = self.registers.f.carry;
                let new_carry = (self.registers.a & 0x80) != 0;
                self.registers.a = (self.registers.a << 1) | (if old_carry { 1 } else { 0 });
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = new_carry;
            }
            
            InstructionKind::RRA => {
                // Rotate A right through carry
                let old_carry = self.registers.f.carry;
                let new_carry = (self.registers.a & 0x01) != 0;
                self.registers.a = (self.registers.a >> 1) | (if old_carry { 0x80 } else { 0 });
                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = new_carry;
            }
            
            // Logical operations
            InstructionKind::XOR(dest, src) => {
                let src_value = match src {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    ArgKind::HL => {
                        let addr = self.registers.get_hl();
                        self.mmap.read(addr)
                    }
                    ArgKind::Immediate(value) => value,
                    _ => panic!("Unsupported XOR source"),
                };
                if let ArgKind::A = dest {
                    self.registers.a ^= src_value;
                    self.registers.f.zero = self.registers.a == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = false;
                }
            }
            InstructionKind::AND(dest, src) => {
                let src_value = match src {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    ArgKind::Immediate(value) => value,
                    _ => panic!("Unsupported AND source"),
                };
                if let ArgKind::A = dest {
                    self.registers.a &= src_value;
                    self.registers.f.zero = self.registers.a == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = true;
                    self.registers.f.carry = false;
                }
            }
            InstructionKind::OR(dest, src) => {
                let src_value = match src {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    ArgKind::HL => {
                        let addr = self.registers.get_hl();
                        self.mmap.read(addr)
                    }
                    ArgKind::Immediate(value) => value,
                    _ => panic!("Unsupported OR source"),
                };
                if let ArgKind::A = dest {
                    self.registers.a |= src_value;
                    self.registers.f.zero = self.registers.a == 0;
                    self.registers.f.subtract = false;
                    self.registers.f.half_carry = false;
                    self.registers.f.carry = false;
                }
            }
            InstructionKind::CP(dest, src) => {
                let src_value = match src {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    ArgKind::Immediate(value) => value,
                    _ => panic!("Unsupported CP source"),
                };
                if let ArgKind::A = dest {
                    let result = self.registers.a.wrapping_sub(src_value);
                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = true;
                    self.registers.f.half_carry = (self.registers.a & 0x0F) < (src_value & 0x0F);
                    self.registers.f.carry = self.registers.a < src_value;
                }
            }
            InstructionKind::CP_MEM(dest, addr_reg) => {
                let address = match addr_reg {
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported CP_MEM address register"),
                };
                let src_value = self.mmap.read(address);
                if let ArgKind::A = dest {
                    let result = self.registers.a.wrapping_sub(src_value);
                    self.registers.f.zero = result == 0;
                    self.registers.f.subtract = true;
                    self.registers.f.half_carry = (self.registers.a & 0x0F) < (src_value & 0x0F);
                    self.registers.f.carry = self.registers.a < src_value;
                }
            }
            
            // Jump relative
            InstructionKind::JR(condition, offset) => {
                if self.check_jump_condition(condition) {
                    self.pc = ((self.pc as i32) + (offset as i32)) as u16;
                }
            }
            
            // Memory operations
            InstructionKind::LD_MEM(addr_reg, src) => {
                let address = match addr_reg {
                    ArgKind::BC => self.registers.get_bc(),
                    ArgKind::DE => self.registers.get_de(),
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported LD_MEM address register"),
                };
                let value = match src {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    ArgKind::Immediate(val) => val,
                    _ => panic!("Unsupported LD_MEM source"),
                };
                self.mmap.write(address, value);
            }
            InstructionKind::LD_MEM_DEC(addr_reg, src) => {
                let address = match addr_reg {
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported LD_MEM_DEC address register"),
                };
                let value = match src {
                    ArgKind::A => self.registers.a,
                    _ => panic!("Unsupported LD_MEM_DEC source"),
                };
                self.mmap.write(address, value);
                let new_hl = address.wrapping_sub(1);
                self.registers.set_hl(new_hl);
            }
            InstructionKind::LD_MEM_INC(addr_reg, src) => {
                let address = match addr_reg {
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported LD_MEM_INC address register"),
                };
                let value = match src {
                    ArgKind::A => self.registers.a,
                    _ => panic!("Unsupported LD_MEM_INC source"),
                };
                self.mmap.write(address, value);
                let new_hl = address.wrapping_add(1);
                self.registers.set_hl(new_hl);
            }
            InstructionKind::LD_FROM_MEM(dest, addr_reg) => {
                let address = match addr_reg {
                    ArgKind::BC => self.registers.get_bc(),
                    ArgKind::DE => self.registers.get_de(),
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported LD_FROM_MEM address register"),
                };
                let value = self.mmap.read(address);
                match dest {
                    ArgKind::A => self.registers.a = value,
                    ArgKind::B => self.registers.b = value,
                    ArgKind::C => self.registers.c = value,
                    ArgKind::D => self.registers.d = value,
                    ArgKind::E => self.registers.e = value,
                    ArgKind::H => self.registers.h = value,
                    ArgKind::L => self.registers.l = value,
                    _ => panic!("Unsupported LD_FROM_MEM destination"),
                }
            }
            InstructionKind::LD_FROM_MEM_DEC(dest, addr_reg) => {
                let address = match addr_reg {
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported LD_FROM_MEM_DEC address register"),
                };
                let value = self.mmap.read(address);
                if let ArgKind::A = dest {
                    self.registers.a = value;
                }
                let new_hl = address.wrapping_sub(1);
                self.registers.set_hl(new_hl);
            }
            InstructionKind::LD_FROM_MEM_INC(dest, addr_reg) => {
                let address = match addr_reg {
                    ArgKind::HL => self.registers.get_hl(),
                    _ => panic!("Unsupported LD_FROM_MEM_INC address register"),
                };
                let value = self.mmap.read(address);
                if let ArgKind::A = dest {
                    self.registers.a = value;
                }
                let new_hl = address.wrapping_add(1);
                self.registers.set_hl(new_hl);
            }
            InstructionKind::LD_MEM_16(addr, src) => {
                let address = match addr {
                    ArgKind::Immediate16(addr) => addr,
                    _ => panic!("Unsupported LD_MEM_16 address type"),
                };
                let value = match src {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    _ => panic!("Unsupported LD_MEM_16 source"),
                };
                self.mmap.write(address, value);
            }
            InstructionKind::LD_FROM_MEM_16(dest, addr) => {
                let address = match addr {
                    ArgKind::Immediate16(addr) => addr,
                    _ => panic!("Unsupported LD_FROM_MEM_16 address type"),
                };
                let value = self.mmap.read(address);
                match dest {
                    ArgKind::A => self.registers.a = value,
                    ArgKind::B => self.registers.b = value,
                    ArgKind::C => self.registers.c = value,
                    ArgKind::D => self.registers.d = value,
                    ArgKind::E => self.registers.e = value,
                    ArgKind::H => self.registers.h = value,
                    ArgKind::L => self.registers.l = value,
                    _ => panic!("Unsupported LD_FROM_MEM_16 destination"),
                };
            }
            
            // I/O operations
            InstructionKind::LDH_TO_C(_) => {
                let address = 0xFF00 + (self.registers.c as u16);
                self.mmap.write(address, self.registers.a);
            }
            InstructionKind::LDH_FROM_C(_) => {
                let address = 0xFF00 + (self.registers.c as u16);
                self.registers.a = self.mmap.read(address);
            }
            InstructionKind::LDH_TO_N(_, offset) => {
                let address = 0xFF00 + (offset as u16);
                self.mmap.write(address, self.registers.a);
            }
            InstructionKind::LDH_FROM_N(_, offset) => {
                let address = 0xFF00 + (offset as u16);
                self.registers.a = self.mmap.read(address);
            }
            
            // Interrupt control
            InstructionKind::EI => {
                // Enable interrupts after next instruction (1-instruction delay)
                self.ei_delay = true;
            }
            InstructionKind::DI => {
                // Disable interrupts immediately
                self.ime = false;
                self.ei_delay = false; // Cancel any pending EI
            }
            
            // Stack operations
            InstructionKind::PUSH(reg_pair) => {
                let value = match reg_pair {
                    ArgKind::BC => self.registers.get_bc(),
                    ArgKind::DE => self.registers.get_de(),
                    ArgKind::HL => self.registers.get_hl(),
                    ArgKind::AF => self.registers.get_af(),
                    _ => panic!("Unsupported PUSH register pair"),
                };
                self.push_stack(value);
            }
            InstructionKind::POP(reg_pair) => {
                let value = self.pop_stack();
                match reg_pair {
                    ArgKind::BC => self.registers.set_bc(value),
                    ArgKind::DE => self.registers.set_de(value),
                    ArgKind::HL => self.registers.set_hl(value),
                    ArgKind::AF => self.registers.set_af(value),
                    _ => panic!("Unsupported POP register pair"),
                }
            }
            
            // Rotate operations
            InstructionKind::RL(reg) => {
                let value = match reg {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    _ => panic!("Unsupported RL register"),
                };
                
                let old_carry = if self.registers.f.carry { 1 } else { 0 };
                let new_carry = (value & 0x80) != 0;
                let result = (value << 1) | old_carry;
                
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = new_carry;
                
                match reg {
                    ArgKind::A => self.registers.a = result,
                    ArgKind::B => self.registers.b = result,
                    ArgKind::C => self.registers.c = result,
                    ArgKind::D => self.registers.d = result,
                    ArgKind::E => self.registers.e = result,
                    ArgKind::H => self.registers.h = result,
                    ArgKind::L => self.registers.l = result,
                    _ => {}
                }
            }
            InstructionKind::RR(reg) => {
                let value = match reg {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    _ => panic!("Unsupported RR register"),
                };
                
                let old_carry = if self.registers.f.carry { 0x80 } else { 0 };
                let new_carry = (value & 0x01) != 0;
                let result = (value >> 1) | old_carry;
                
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = new_carry;
                
                match reg {
                    ArgKind::A => self.registers.a = result,
                    ArgKind::B => self.registers.b = result,
                    ArgKind::C => self.registers.c = result,
                    ArgKind::D => self.registers.d = result,
                    ArgKind::E => self.registers.e = result,
                    ArgKind::H => self.registers.h = result,
                    ArgKind::L => self.registers.l = result,
                    _ => {}
                }
            }
            
            
            // ADC - Add with carry
            InstructionKind::ADC(dest, src) => {
                let carry = if self.registers.f.carry { 1 } else { 0 };
                let (a_val, src_val) = match (dest, src) {
                    (ArgKind::A, ArgKind::B) => (self.registers.a, self.registers.b),
                    (ArgKind::A, ArgKind::C) => (self.registers.a, self.registers.c),
                    (ArgKind::A, ArgKind::D) => (self.registers.a, self.registers.d),
                    (ArgKind::A, ArgKind::E) => (self.registers.a, self.registers.e),
                    (ArgKind::A, ArgKind::H) => (self.registers.a, self.registers.h),
                    (ArgKind::A, ArgKind::L) => (self.registers.a, self.registers.l),
                    (ArgKind::A, ArgKind::A) => (self.registers.a, self.registers.a),
                    (ArgKind::A, ArgKind::HL) => (self.registers.a, self.mmap.read(self.registers.get_hl())),
                    (ArgKind::A, ArgKind::Immediate(val)) => (self.registers.a, val),
                    _ => panic!("Invalid ADC operands"),
                };
                
                let result = a_val.wrapping_add(src_val).wrapping_add(carry);
                let half_carry = (a_val & 0x0F) + (src_val & 0x0F) + carry > 0x0F;
                let full_carry = (a_val as u16) + (src_val as u16) + (carry as u16) > 0xFF;
                
                self.registers.a = result;
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = half_carry;
                self.registers.f.carry = full_carry;
            }
            
            // RST - Restart (call to fixed address)
            InstructionKind::RST(addr) => {
                self.push_stack(self.pc);
                self.pc = addr as u16;
            }
            
            // CB-prefixed rotate/shift instructions
            InstructionKind::RLC(reg) => {
                let (value, new_value) = match reg {
                    ArgKind::A => (self.registers.a, self.registers.a.rotate_left(1)),
                    ArgKind::B => (self.registers.b, self.registers.b.rotate_left(1)),
                    ArgKind::C => (self.registers.c, self.registers.c.rotate_left(1)),
                    ArgKind::D => (self.registers.d, self.registers.d.rotate_left(1)),
                    ArgKind::E => (self.registers.e, self.registers.e.rotate_left(1)),
                    ArgKind::H => (self.registers.h, self.registers.h.rotate_left(1)),
                    ArgKind::L => (self.registers.l, self.registers.l.rotate_left(1)),
                    _ => panic!("Invalid RLC register"),
                };
                
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = (value & 0x80) != 0;
                
                match reg {
                    ArgKind::A => self.registers.a = new_value,
                    ArgKind::B => self.registers.b = new_value,
                    ArgKind::C => self.registers.c = new_value,
                    ArgKind::D => self.registers.d = new_value,
                    ArgKind::E => self.registers.e = new_value,
                    ArgKind::H => self.registers.h = new_value,
                    ArgKind::L => self.registers.l = new_value,
                    _ => {}
                }
            }
            
            InstructionKind::RRC(reg) => {
                let (value, new_value) = match reg {
                    ArgKind::A => (self.registers.a, self.registers.a.rotate_right(1)),
                    ArgKind::B => (self.registers.b, self.registers.b.rotate_right(1)),
                    ArgKind::C => (self.registers.c, self.registers.c.rotate_right(1)),
                    ArgKind::D => (self.registers.d, self.registers.d.rotate_right(1)),
                    ArgKind::E => (self.registers.e, self.registers.e.rotate_right(1)),
                    ArgKind::H => (self.registers.h, self.registers.h.rotate_right(1)),
                    ArgKind::L => (self.registers.l, self.registers.l.rotate_right(1)),
                    _ => panic!("Invalid RRC register"),
                };
                
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = (value & 0x01) != 0;
                
                match reg {
                    ArgKind::A => self.registers.a = new_value,
                    ArgKind::B => self.registers.b = new_value,
                    ArgKind::C => self.registers.c = new_value,
                    ArgKind::D => self.registers.d = new_value,
                    ArgKind::E => self.registers.e = new_value,
                    ArgKind::H => self.registers.h = new_value,
                    ArgKind::L => self.registers.l = new_value,
                    _ => {}
                }
            }
            
            InstructionKind::SLA(reg) => {
                let value = match reg {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    _ => panic!("Invalid SLA register"),
                };
                
                let new_value = value << 1;
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = (value & 0x80) != 0;
                
                match reg {
                    ArgKind::A => self.registers.a = new_value,
                    ArgKind::B => self.registers.b = new_value,
                    ArgKind::C => self.registers.c = new_value,
                    ArgKind::D => self.registers.d = new_value,
                    ArgKind::E => self.registers.e = new_value,
                    ArgKind::H => self.registers.h = new_value,
                    ArgKind::L => self.registers.l = new_value,
                    _ => {}
                }
            }
            
            InstructionKind::SRA(reg) => {
                let value = match reg {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    _ => panic!("Invalid SRA register"),
                };
                
                let new_value = (value >> 1) | (value & 0x80); // Preserve sign bit
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = (value & 0x01) != 0;
                
                match reg {
                    ArgKind::A => self.registers.a = new_value,
                    ArgKind::B => self.registers.b = new_value,
                    ArgKind::C => self.registers.c = new_value,
                    ArgKind::D => self.registers.d = new_value,
                    ArgKind::E => self.registers.e = new_value,
                    ArgKind::H => self.registers.h = new_value,
                    ArgKind::L => self.registers.l = new_value,
                    _ => {}
                }
            }
            
            InstructionKind::SRL(reg) => {
                let value = match reg {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    _ => panic!("Invalid SRL register"),
                };
                
                let new_value = value >> 1;
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = (value & 0x01) != 0;
                
                match reg {
                    ArgKind::A => self.registers.a = new_value,
                    ArgKind::B => self.registers.b = new_value,
                    ArgKind::C => self.registers.c = new_value,
                    ArgKind::D => self.registers.d = new_value,
                    ArgKind::E => self.registers.e = new_value,
                    ArgKind::H => self.registers.h = new_value,
                    ArgKind::L => self.registers.l = new_value,
                    _ => {}
                }
            }
            
            InstructionKind::SWAP(reg) => {
                let value = match reg {
                    ArgKind::A => self.registers.a,
                    ArgKind::B => self.registers.b,
                    ArgKind::C => self.registers.c,
                    ArgKind::D => self.registers.d,
                    ArgKind::E => self.registers.e,
                    ArgKind::H => self.registers.h,
                    ArgKind::L => self.registers.l,
                    _ => panic!("Invalid SWAP register"),
                };
                
                // Swap upper and lower nibbles
                let new_value = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
                
                match reg {
                    ArgKind::A => self.registers.a = new_value,
                    ArgKind::B => self.registers.b = new_value,
                    ArgKind::C => self.registers.c = new_value,
                    ArgKind::D => self.registers.d = new_value,
                    ArgKind::E => self.registers.e = new_value,
                    ArgKind::H => self.registers.h = new_value,
                    ArgKind::L => self.registers.l = new_value,
                    _ => {}
                }
            }
        }
        
        // Calculate cycles for this instruction
        // For conditional instructions, we need to check if condition was taken
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
        
        get_instruction_cycles(&instruction.kind, condition_taken)
    }

    fn add(&mut self, value: u8) -> u8 {
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

    fn add16(&mut self, dest_value: u16, src_value: u16) -> u16 {
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
    
    fn increment(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // Half carry occurs if bit 3 carried to bit 4
        self.registers.f.half_carry = (value & 0x0F) == 0x0F;
        // Carry flag is not affected by INC
        new_value
    }
    
    fn decrement(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        // Half carry occurs if bit 4 borrowed from bit 3
        self.registers.f.half_carry = (value & 0x0F) == 0x00;
        // Carry flag is not affected by DEC
        new_value
    }
    
    fn subtract(&mut self, value: u8) -> u8 {
        let result = self.registers.a.wrapping_sub(value);
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F);
        self.registers.f.carry = self.registers.a < value;
        result
    }
    
    fn subtract_with_carry(&mut self, value: u8) -> u8 {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let temp = value.wrapping_add(carry);
        let result = self.registers.a.wrapping_sub(temp);
        
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < ((value & 0x0F) + carry);
        self.registers.f.carry = (self.registers.a as u16) < (value as u16 + carry as u16);
        result
    }
    
    fn test_bit(&mut self, bit: u8, value: u8) {
        let bit_set = (value & (1 << bit)) != 0;
        self.registers.f.zero = !bit_set;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        // Carry flag is not affected by BIT
    }
    
    fn check_jump_condition(&self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::Always => true,
            JumpCondition::Zero => self.registers.f.zero,
            JumpCondition::NotZero => !self.registers.f.zero,
            JumpCondition::Carry => self.registers.f.carry,
            JumpCondition::NotCarry => !self.registers.f.carry,
        }
    }
    
    fn push_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.mmap.write(self.sp, ((value & 0xFF00) >> 8) as u8); // High byte
        self.sp = self.sp.wrapping_sub(1);
        self.mmap.write(self.sp, (value & 0x00FF) as u8); // Low byte
    }
    
    fn pop_stack(&mut self) -> u16 {
        let low = self.mmap.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let high = self.mmap.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (high << 8) | low
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
        self.pc = match interrupt_bit {
            VBLANK_BIT => VBLANK_VECTOR,
            LCD_STAT_BIT => LCD_STAT_VECTOR,
            TIMER_BIT => TIMER_VECTOR,
            SERIAL_BIT => SERIAL_VECTOR,
            JOYPAD_BIT => JOYPAD_VECTOR,
            _ => unreachable!(),
        };
        
        // Interrupt handling takes 5 M-cycles (20 T-cycles)
        20
    }
    
    pub fn request_interrupt(&mut self, interrupt_bit: u8) {
        let if_reg = self.mmap.read(IF_REGISTER);
        self.mmap.write(IF_REGISTER, if_reg | (1 << interrupt_bit));
    }
    
    // Helper methods for requesting specific interrupts
    pub fn request_vblank_interrupt(&mut self) {
        self.request_interrupt(VBLANK_BIT);
    }
    
    pub fn request_lcd_stat_interrupt(&mut self) {
        self.request_interrupt(LCD_STAT_BIT);
    }
    
    pub fn request_timer_interrupt(&mut self) {
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

