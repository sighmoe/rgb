use super::{InstructionKind, ArgKind};

pub fn decode_increment_instruction(opcode: u8) -> Option<InstructionKind> {
    match opcode {
        // Increment instructions
        0x04 => Some(InstructionKind::INC(ArgKind::B)),
        0x0C => Some(InstructionKind::INC(ArgKind::C)),
        0x14 => Some(InstructionKind::INC(ArgKind::D)),
        0x1C => Some(InstructionKind::INC(ArgKind::E)),
        0x24 => Some(InstructionKind::INC(ArgKind::H)),
        0x2C => Some(InstructionKind::INC(ArgKind::L)),
        0x3C => Some(InstructionKind::INC(ArgKind::A)),
        0x34 => Some(InstructionKind::INC_MEM(ArgKind::HL)), // INC (HL)
        
        // Decrement instructions
        0x05 => Some(InstructionKind::DEC(ArgKind::B)),
        0x0D => Some(InstructionKind::DEC(ArgKind::C)),
        0x15 => Some(InstructionKind::DEC(ArgKind::D)),
        0x1D => Some(InstructionKind::DEC(ArgKind::E)),
        0x25 => Some(InstructionKind::DEC(ArgKind::H)),
        0x2D => Some(InstructionKind::DEC(ArgKind::L)),
        0x3D => Some(InstructionKind::DEC(ArgKind::A)),
        0x35 => Some(InstructionKind::DEC_MEM(ArgKind::HL)), // DEC (HL)
        
        // 16-bit increment instructions
        0x03 => Some(InstructionKind::INC16(ArgKind::BC)),
        0x13 => Some(InstructionKind::INC16(ArgKind::DE)),
        0x23 => Some(InstructionKind::INC16(ArgKind::HL)),
        0x33 => Some(InstructionKind::INC16(ArgKind::SP)),
        
        // 16-bit decrement instructions
        0x0B => Some(InstructionKind::DEC16(ArgKind::BC)),
        0x1B => Some(InstructionKind::DEC16(ArgKind::DE)),
        0x2B => Some(InstructionKind::DEC16(ArgKind::HL)),
        0x3B => Some(InstructionKind::DEC16(ArgKind::SP)),
        
        _ => None,
    }
}

pub fn get_increment_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // 8-bit increment and decrement instructions are all 1 byte
        0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x3C | 0x34 |
        0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x3D | 0x35 => Some(1),
        
        // 16-bit increment and decrement instructions are all 1 byte
        0x03 | 0x13 | 0x23 | 0x33 | 0x0B | 0x1B | 0x2B | 0x3B => Some(1),
        
        _ => None,
    }
}