use super::{InstructionKind, ArgKind};

pub fn decode_arithmetic_instruction(opcode: u8) -> Option<InstructionKind> {
    match opcode {
        // ADD instructions
        0x80 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::B)),
        0x81 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::C)),
        0x82 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::D)),
        0x83 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::E)),
        0x84 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::H)),
        0x85 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::L)),
        0x87 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::A)),
        
        // SUB instructions
        0x90 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::B)),
        0x91 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::C)),
        0x92 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::D)),
        0x93 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::E)),
        0x94 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::H)),
        0x95 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::L)),
        0x97 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::A)),
        
        // SBC instructions (subtract with carry)
        0x98 => Some(InstructionKind::SBC(ArgKind::A, ArgKind::B)),
        0x99 => Some(InstructionKind::SBC(ArgKind::A, ArgKind::C)),
        0x9A => Some(InstructionKind::SBC(ArgKind::A, ArgKind::D)),
        0x9B => Some(InstructionKind::SBC(ArgKind::A, ArgKind::E)),
        0x9C => Some(InstructionKind::SBC(ArgKind::A, ArgKind::H)),
        0x9D => Some(InstructionKind::SBC(ArgKind::A, ArgKind::L)),
        0x9F => Some(InstructionKind::SBC(ArgKind::A, ArgKind::A)),
        
        _ => None,
    }
}

pub fn get_arithmetic_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // ADD, SUB, and SBC instructions are all 1 byte
        0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x85 | 0x87 |
        0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x97 |
        0x98 | 0x99 | 0x9A | 0x9B | 0x9C | 0x9D | 0x9F => Some(1),
        _ => None,
    }
}