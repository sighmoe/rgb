pub mod load;
pub mod arithmetic;
pub mod increment;
pub mod bit_operations;
pub mod control_flow;
pub mod cpu_control;
pub mod logical;
pub mod memory;
pub mod stack;

pub use load::{decode_load_instruction, get_load_instruction_size};
pub use arithmetic::{decode_arithmetic_instruction, get_arithmetic_instruction_size};
pub use increment::{decode_increment_instruction, get_increment_instruction_size};
pub use bit_operations::decode_bit_instruction;
pub use control_flow::{decode_control_flow_instruction, get_control_flow_instruction_size, JumpCondition};
pub use cpu_control::{decode_cpu_control_instruction, get_cpu_control_instruction_size};
pub use logical::{decode_logical_instruction, get_logical_instruction_size};
pub use memory::{decode_memory_instruction, get_memory_instruction_size};
pub use stack::{decode_stack_instruction, get_stack_instruction_size};

#[derive(Clone, Copy)]
pub enum ArgKind {
    Immediate(u8),
    Immediate16(u16),
    SP,
    BC,
    DE,
    HL,
    AF,
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)] // Allow descriptive instruction names
pub enum InstructionKind {
    LD(ArgKind, ArgKind),
    NOP,
    ADD(ArgKind, ArgKind),
    ADC(ArgKind, ArgKind), // Add with carry
    SUB(ArgKind, ArgKind),
    SBC(ArgKind, ArgKind),
    INC(ArgKind),
    DEC(ArgKind),
    INC_MEM(ArgKind), // increment memory at address
    DEC_MEM(ArgKind), // decrement memory at address
    INC16(ArgKind),   // 16-bit increment
    DEC16(ArgKind),   // 16-bit decrement
    BIT(u8, ArgKind), // bit number, register
    RES(u8, ArgKind), // reset bit, register
    SET(u8, ArgKind), // set bit, register
    JP(JumpCondition, u16), // condition, address
    JP_HL,                  // jump to address in HL
    JR(JumpCondition, i8), // condition, signed offset
    CALL(u16), // address
    CALL_COND(JumpCondition, u16), // conditional call
    RET,
    RET_COND(JumpCondition), // conditional return
    RETI, // return from interrupt (and enable interrupts)
    RST(u8), // restart - call to fixed address
    HALT,
    STOP,
    
    // Logical operations
    XOR(ArgKind, ArgKind),
    AND(ArgKind, ArgKind),
    OR(ArgKind, ArgKind),
    CP(ArgKind, ArgKind), // Compare
    CP_MEM(ArgKind, ArgKind), // Compare with memory
    
    // Memory operations
    LD_MEM(ArgKind, ArgKind),        // LD (addr),reg
    LD_MEM_INC(ArgKind, ArgKind),    // LD (addr+),reg
    LD_MEM_DEC(ArgKind, ArgKind),    // LD (addr-),reg
    LD_FROM_MEM(ArgKind, ArgKind),   // LD reg,(addr)
    LD_FROM_MEM_INC(ArgKind, ArgKind), // LD reg,(addr+)
    LD_FROM_MEM_DEC(ArgKind, ArgKind), // LD reg,(addr-)
    LD_MEM_16(ArgKind, ArgKind),     // LD (nn),reg - 16-bit address
    LD_FROM_MEM_16(ArgKind, ArgKind), // LD reg,(nn) - from 16-bit address
    
    // I/O operations
    #[allow(dead_code)] LDH_TO_C(ArgKind),      // LD (C),A
    #[allow(dead_code)] LDH_FROM_C(ArgKind),    // LD A,(C)
    #[allow(dead_code)] LDH_TO_N(ArgKind, u8),  // LDH (n),A
    #[allow(dead_code)] LDH_FROM_N(ArgKind, u8), // LDH A,(n)
    
    // Special load operations
    LDHL_SP_R8(i8), // LD HL,SP+r8 - Load HL with SP plus signed offset
    LD_SP_TO_MEM(u16), // LD (nn),SP - Store SP at 16-bit address
    
    // Interrupt control
    EI,  // Enable interrupts
    DI,  // Disable interrupts
    
    // Decimal operations
    DAA,  // Decimal Adjust Accumulator
    
    // Stack operations
    PUSH(ArgKind), // Push register pair onto stack
    POP(ArgKind),  // Pop register pair from stack
    
    // Rotate/shift operations
    RL(ArgKind),   // Rotate left through carry
    RR(ArgKind),   // Rotate right through carry
    RLA,           // Rotate A left through carry (special case)
    RRA,           // Rotate A right through carry (special case)
    RLCA,          // Rotate A left circular (special case)
    RRCA,          // Rotate A right circular (special case)
    RLC(ArgKind),  // Rotate left circular
    RRC(ArgKind),  // Rotate right circular
    SLA(ArgKind),  // Shift left arithmetic
    SRA(ArgKind),  // Shift right arithmetic
    SRL(ArgKind),  // Shift right logical
    SWAP(ArgKind), // Swap upper and lower nibbles
}

#[allow(dead_code)] // Instruction format fields for future use
pub struct Instruction {
    pub kind: InstructionKind,
    pub instr: u8,
    pub arg1: Option<ArgKind>,
    pub arg2: Option<ArgKind>,
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            kind: InstructionKind::NOP,
            instr: 0,
            arg1: None,
            arg2: None,
        }
    }
}


pub fn decode_instruction(opcode: u8, immediate: Option<u8>, immediate16: Option<u16>) -> InstructionKind {
    // Try load instructions first
    if let Some(load_instr) = decode_load_instruction(opcode, immediate, immediate16) {
        return load_instr;
    }
    
    // Try arithmetic instructions
    if let Some(arith_instr) = decode_arithmetic_instruction(opcode, immediate) {
        return arith_instr;
    }
    
    // Try increment instructions
    if let Some(inc_instr) = decode_increment_instruction(opcode) {
        return inc_instr;
    }
    
    // Try logical instructions
    if let Some(logical_instr) = decode_logical_instruction(opcode, immediate) {
        return logical_instr;
    }
    
    // Try memory instructions
    if let Some(memory_instr) = decode_memory_instruction(opcode, immediate) {
        return memory_instr;
    }
    
    // Try control flow instructions
    if let Some(cf_instr) = decode_control_flow_instruction(opcode, immediate, immediate16) {
        return cf_instr;
    }
    
    // Try stack instructions
    if let Some(stack_instr) = decode_stack_instruction(opcode) {
        return stack_instr;
    }
    
    // Try CPU control instructions
    if let Some(cpu_instr) = decode_cpu_control_instruction(opcode) {
        return cpu_instr;
    }
    
    // Handle other instructions
    match opcode {
        0x00 => InstructionKind::NOP,
        0x07 => InstructionKind::RLCA,
        0x08 => InstructionKind::LD_SP_TO_MEM(immediate16.unwrap_or(0)),
        0x0F => InstructionKind::RRCA,
        0x17 => InstructionKind::RLA,
        0x1F => InstructionKind::RRA,
        0x27 => InstructionKind::DAA,
        0xD9 => InstructionKind::RETI,
        _ => panic!("Unknown opcode: 0x{:02X}", opcode),
    }
}

pub fn get_instruction_size(opcode: u8) -> u16 {
    // Try load instructions first
    if let Some(size) = get_load_instruction_size(opcode) {
        return size;
    }
    
    // Try arithmetic instructions
    if let Some(size) = get_arithmetic_instruction_size(opcode) {
        return size;
    }
    
    // Try increment instructions
    if let Some(size) = get_increment_instruction_size(opcode) {
        return size;
    }
    
    // Try logical instructions
    if let Some(size) = get_logical_instruction_size(opcode) {
        return size;
    }
    
    // Try memory instructions
    if let Some(size) = get_memory_instruction_size(opcode) {
        return size;
    }
    
    // Try control flow instructions
    if let Some(size) = get_control_flow_instruction_size(opcode) {
        return size;
    }
    
    // Try stack instructions
    if let Some(size) = get_stack_instruction_size(opcode) {
        return size;
    }
    
    // Try CPU control instructions
    if let Some(size) = get_cpu_control_instruction_size(opcode) {
        return size;
    }
    
    // Handle other instructions
    match opcode {
        0x00 => 1, // NOP
        0x07 => 1, // RLCA
        0x08 => 3, // LD (nn),SP - 3 bytes: opcode + 16-bit address
        0x0F => 1, // RRCA
        0x17 => 1, // RLA
        0x1F => 1, // RRA
        0x27 => 1, // DAA
        0xD9 => 1, // RETI
        _ => panic!("Unknown opcode: 0x{:02X}", opcode),
    }
}

pub fn decode_cb_instruction(cb_opcode: u8) -> InstructionKind {
    let full_opcode = 0xCB00 | (cb_opcode as u16);
    if let Some(bit_instr) = decode_bit_instruction(full_opcode) {
        return bit_instr;
    }
    
    panic!("Unknown CB opcode: 0xCB{:02X}", cb_opcode);
}

pub fn get_cb_instruction_size() -> u16 {
    2 // All CB instructions are 2 bytes
}