use super::{InstructionKind, ArgKind};

pub fn decode_load_instruction(opcode: u8, immediate: Option<u8>, immediate16: Option<u16>) -> Option<InstructionKind> {
    match opcode {
        // 16-bit loads
        0x01 => Some(InstructionKind::LD(ArgKind::BC, ArgKind::Immediate16(immediate16.unwrap_or(0)))),
        0x11 => Some(InstructionKind::LD(ArgKind::DE, ArgKind::Immediate16(immediate16.unwrap_or(0)))),
        0x21 => Some(InstructionKind::LD(ArgKind::HL, ArgKind::Immediate16(immediate16.unwrap_or(0)))),
        0x31 => Some(InstructionKind::LD(ArgKind::SP, ArgKind::Immediate16(immediate16.unwrap_or(0)))),
        
        // 8-bit immediate loads
        0x06 => Some(InstructionKind::LD(ArgKind::B, ArgKind::Immediate(immediate.unwrap_or(0)))),
        0x0E => Some(InstructionKind::LD(ArgKind::C, ArgKind::Immediate(immediate.unwrap_or(0)))),
        0x16 => Some(InstructionKind::LD(ArgKind::D, ArgKind::Immediate(immediate.unwrap_or(0)))),
        0x1E => Some(InstructionKind::LD(ArgKind::E, ArgKind::Immediate(immediate.unwrap_or(0)))),
        0x26 => Some(InstructionKind::LD(ArgKind::H, ArgKind::Immediate(immediate.unwrap_or(0)))),
        0x2E => Some(InstructionKind::LD(ArgKind::L, ArgKind::Immediate(immediate.unwrap_or(0)))),
        0x3E => Some(InstructionKind::LD(ArgKind::A, ArgKind::Immediate(immediate.unwrap_or(0)))),
        
        // Register-to-register loads
        0x41 => Some(InstructionKind::LD(ArgKind::B, ArgKind::C)),
        0x42 => Some(InstructionKind::LD(ArgKind::B, ArgKind::D)),
        0x43 => Some(InstructionKind::LD(ArgKind::B, ArgKind::E)),
        0x44 => Some(InstructionKind::LD(ArgKind::B, ArgKind::H)),
        0x45 => Some(InstructionKind::LD(ArgKind::B, ArgKind::L)),
        0x47 => Some(InstructionKind::LD(ArgKind::B, ArgKind::A)),
        0x48 => Some(InstructionKind::LD(ArgKind::C, ArgKind::B)),
        0x4A => Some(InstructionKind::LD(ArgKind::C, ArgKind::D)),
        0x4B => Some(InstructionKind::LD(ArgKind::C, ArgKind::E)),
        0x4C => Some(InstructionKind::LD(ArgKind::C, ArgKind::H)),
        0x4D => Some(InstructionKind::LD(ArgKind::C, ArgKind::L)),
        0x4F => Some(InstructionKind::LD(ArgKind::C, ArgKind::A)),
        0x50 => Some(InstructionKind::LD(ArgKind::D, ArgKind::B)),
        0x51 => Some(InstructionKind::LD(ArgKind::D, ArgKind::C)),
        0x53 => Some(InstructionKind::LD(ArgKind::D, ArgKind::E)),
        0x54 => Some(InstructionKind::LD(ArgKind::D, ArgKind::H)),
        0x55 => Some(InstructionKind::LD(ArgKind::D, ArgKind::L)),
        0x57 => Some(InstructionKind::LD(ArgKind::D, ArgKind::A)),
        0x58 => Some(InstructionKind::LD(ArgKind::E, ArgKind::B)),
        0x59 => Some(InstructionKind::LD(ArgKind::E, ArgKind::C)),
        0x5A => Some(InstructionKind::LD(ArgKind::E, ArgKind::D)),
        0x5C => Some(InstructionKind::LD(ArgKind::E, ArgKind::H)),
        0x5D => Some(InstructionKind::LD(ArgKind::E, ArgKind::L)),
        0x5F => Some(InstructionKind::LD(ArgKind::E, ArgKind::A)),
        0x60 => Some(InstructionKind::LD(ArgKind::H, ArgKind::B)),
        0x61 => Some(InstructionKind::LD(ArgKind::H, ArgKind::C)),
        0x62 => Some(InstructionKind::LD(ArgKind::H, ArgKind::D)),
        0x63 => Some(InstructionKind::LD(ArgKind::H, ArgKind::E)),
        0x65 => Some(InstructionKind::LD(ArgKind::H, ArgKind::L)),
        0x67 => Some(InstructionKind::LD(ArgKind::H, ArgKind::A)),
        0x68 => Some(InstructionKind::LD(ArgKind::L, ArgKind::B)),
        0x69 => Some(InstructionKind::LD(ArgKind::L, ArgKind::C)),
        0x6A => Some(InstructionKind::LD(ArgKind::L, ArgKind::D)),
        0x6B => Some(InstructionKind::LD(ArgKind::L, ArgKind::E)),
        0x6C => Some(InstructionKind::LD(ArgKind::L, ArgKind::H)),
        0x6F => Some(InstructionKind::LD(ArgKind::L, ArgKind::A)),
        0x78 => Some(InstructionKind::LD(ArgKind::A, ArgKind::B)),
        0x79 => Some(InstructionKind::LD(ArgKind::A, ArgKind::C)),
        0x7A => Some(InstructionKind::LD(ArgKind::A, ArgKind::D)),
        0x7B => Some(InstructionKind::LD(ArgKind::A, ArgKind::E)),
        0x7C => Some(InstructionKind::LD(ArgKind::A, ArgKind::H)),
        0x7D => Some(InstructionKind::LD(ArgKind::A, ArgKind::L)),
        0x7F => Some(InstructionKind::LD(ArgKind::A, ArgKind::A)),
        
        // Indirect loads from register pairs
        0x0A => Some(InstructionKind::LD_FROM_MEM(ArgKind::A, ArgKind::BC)), // LD A,(BC)
        0x1A => Some(InstructionKind::LD_FROM_MEM(ArgKind::A, ArgKind::DE)), // LD A,(DE)
        
        // Load A into memory at 16-bit address
        0xEA => Some(InstructionKind::LD_MEM_16(ArgKind::Immediate16(immediate16.unwrap_or(0)), ArgKind::A)), // LD (nn),A
        
        // Load from memory at 16-bit address
        0xFA => Some(InstructionKind::LD_FROM_MEM_16(ArgKind::A, ArgKind::Immediate16(immediate16.unwrap_or(0)))), // LD A,(nn)
        
        // Special load operation - LD HL,SP+r8
        0xF8 => Some(InstructionKind::LDHL_SP_R8(immediate.unwrap_or(0) as i8)),
        
        // Special load operation - LD SP,HL
        0xF9 => Some(InstructionKind::LD(ArgKind::SP, ArgKind::HL)),
        
        _ => None,
    }
}

pub fn get_load_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // 16-bit loads (3 bytes: opcode + 2 immediate bytes)
        0x01 | 0x11 | 0x21 | 0x31 => Some(3),
        
        // 8-bit immediate loads (2 bytes: opcode + 1 immediate byte)
        0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E | 0xF8 => Some(2),
        
        // Register-to-register loads and indirect loads (1 byte: opcode only)
        0x41 | 0x42 | 0x43 | 0x44 | 0x45 | 0x47 |
        0x48 | 0x4A | 0x4B | 0x4C | 0x4D | 0x4F |
        0x50 | 0x51 | 0x53 | 0x54 | 0x55 | 0x57 |
        0x58 | 0x59 | 0x5A | 0x5C | 0x5D | 0x5F |
        0x60 | 0x61 | 0x62 | 0x63 | 0x65 | 0x67 |
        0x68 | 0x69 | 0x6A | 0x6B | 0x6C | 0x6F |
        0x78 | 0x79 | 0x7A | 0x7B | 0x7C | 0x7D | 0x7F |
        0x0A | 0x1A | 0xF9 => Some(1),
        
        // Load A into memory at 16-bit address (3 bytes: opcode + 2 address bytes)
        0xEA | 0xFA => Some(3),
        
        _ => None,
    }
}