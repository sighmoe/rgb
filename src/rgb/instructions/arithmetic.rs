use super::{InstructionKind, ArgKind};

pub fn decode_arithmetic_instruction(opcode: u8) -> Option<InstructionKind> {
    match opcode {
        0x80 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::B)),
        0x81 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::C)),
        0x82 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::D)),
        0x83 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::E)),
        0x84 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::H)),
        0x85 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::L)),
        0x87 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::A)),
        _ => None,
    }
}

pub fn get_arithmetic_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x85 | 0x87 => Some(1),
        _ => None,
    }
}