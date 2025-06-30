use super::{InstructionKind, ArgKind};

pub fn decode_memory_instruction(opcode: u8, immediate: Option<u8>) -> Option<InstructionKind> {
    match opcode {
        // Store A into memory locations
        0x02 => Some(InstructionKind::LD_MEM(ArgKind::BC, ArgKind::A)),     // LD (BC),A
        0x12 => Some(InstructionKind::LD_MEM(ArgKind::DE, ArgKind::A)),     // LD (DE),A
        0x32 => Some(InstructionKind::LD_MEM_DEC(ArgKind::HL, ArgKind::A)), // LD (HL-),A
        0x22 => Some(InstructionKind::LD_MEM_INC(ArgKind::HL, ArgKind::A)), // LD (HL+),A
        0x77 => Some(InstructionKind::LD_MEM(ArgKind::HL, ArgKind::A)),     // LD (HL),A
        0x70 => Some(InstructionKind::LD_MEM(ArgKind::HL, ArgKind::B)),     // LD (HL),B
        0x71 => Some(InstructionKind::LD_MEM(ArgKind::HL, ArgKind::C)),     // LD (HL),C
        0x72 => Some(InstructionKind::LD_MEM(ArgKind::HL, ArgKind::D)),     // LD (HL),D
        0x73 => Some(InstructionKind::LD_MEM(ArgKind::HL, ArgKind::E)),     // LD (HL),E
        0x74 => Some(InstructionKind::LD_MEM(ArgKind::HL, ArgKind::H)),     // LD (HL),H
        0x75 => Some(InstructionKind::LD_MEM(ArgKind::HL, ArgKind::L)),     // LD (HL),L
        0x36 => Some(InstructionKind::LD_MEM(ArgKind::HL, ArgKind::Immediate(immediate.unwrap_or(0)))), // LD (HL),d8
        
        // Load from memory locations  
        0x2A => Some(InstructionKind::LD_FROM_MEM_INC(ArgKind::A, ArgKind::HL)), // LD A,(HL+)
        0x3A => Some(InstructionKind::LD_FROM_MEM_DEC(ArgKind::A, ArgKind::HL)), // LD A,(HL-)
        0x7E => Some(InstructionKind::LD_FROM_MEM(ArgKind::A, ArgKind::HL)),     // LD A,(HL)
        0x46 => Some(InstructionKind::LD_FROM_MEM(ArgKind::B, ArgKind::HL)),     // LD B,(HL)
        0x4E => Some(InstructionKind::LD_FROM_MEM(ArgKind::C, ArgKind::HL)),     // LD C,(HL)
        0x56 => Some(InstructionKind::LD_FROM_MEM(ArgKind::D, ArgKind::HL)),     // LD D,(HL)
        0x5E => Some(InstructionKind::LD_FROM_MEM(ArgKind::E, ArgKind::HL)),     // LD E,(HL)
        0x66 => Some(InstructionKind::LD_FROM_MEM(ArgKind::H, ArgKind::HL)),     // LD H,(HL)
        0x6E => Some(InstructionKind::LD_FROM_MEM(ArgKind::L, ArgKind::HL)),     // LD L,(HL)
        
        // I/O port operations (0xFF00 + n)
        0xE2 => Some(InstructionKind::LDH_TO_C(ArgKind::A)),    // LD (C),A (0xFF00+C)
        0xF2 => Some(InstructionKind::LDH_FROM_C(ArgKind::A)),  // LD A,(C) (0xFF00+C)
        0xE0 => Some(InstructionKind::LDH_TO_N(ArgKind::A, immediate.unwrap_or(0))),   // LDH (n),A (0xFF00+n)
        0xF0 => Some(InstructionKind::LDH_FROM_N(ArgKind::A, immediate.unwrap_or(0))), // LDH A,(n) (0xFF00+n)
        
        _ => None,
    }
}

pub fn get_memory_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // Single byte instructions
        0x02 | 0x12 | 0x32 | 0x22 | 0x77 | 0x70..=0x75 | 0x2A | 0x3A | 0x7E | 
        0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0xE2 | 0xF2 => Some(1),
        
        // Two byte instructions (opcode + immediate)
        0x36 | 0xE0 | 0xF0 => Some(2),
        
        _ => None,
    }
}