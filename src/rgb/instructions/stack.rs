use super::{InstructionKind, ArgKind};

pub fn decode_stack_instruction(opcode: u8) -> Option<InstructionKind> {
    match opcode {
        // PUSH instructions
        0xC5 => Some(InstructionKind::PUSH(ArgKind::BC)),  // PUSH BC
        0xD5 => Some(InstructionKind::PUSH(ArgKind::DE)),  // PUSH DE
        0xE5 => Some(InstructionKind::PUSH(ArgKind::HL)),  // PUSH HL
        0xF5 => Some(InstructionKind::PUSH(ArgKind::AF)),  // PUSH AF
        
        // POP instructions
        0xC1 => Some(InstructionKind::POP(ArgKind::BC)),   // POP BC
        0xD1 => Some(InstructionKind::POP(ArgKind::DE)),   // POP DE
        0xE1 => Some(InstructionKind::POP(ArgKind::HL)),   // POP HL
        0xF1 => Some(InstructionKind::POP(ArgKind::AF)),   // POP AF
        
        _ => None,
    }
}

pub fn get_stack_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // All stack operations are 1 byte
        0xC5 | 0xD5 | 0xE5 | 0xF5 | 0xC1 | 0xD1 | 0xE1 | 0xF1 => Some(1),
        _ => None,
    }
}