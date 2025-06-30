use super::{InstructionKind, ArgKind};

pub fn decode_arithmetic_instruction(opcode: u8, immediate: Option<u8>) -> Option<InstructionKind> {
    match opcode {
        // 16-bit ADD instructions (ADD HL,reg16)
        0x09 => Some(InstructionKind::ADD(ArgKind::HL, ArgKind::BC)), // ADD HL,BC
        0x19 => Some(InstructionKind::ADD(ArgKind::HL, ArgKind::DE)), // ADD HL,DE
        0x29 => Some(InstructionKind::ADD(ArgKind::HL, ArgKind::HL)), // ADD HL,HL
        0x39 => Some(InstructionKind::ADD(ArgKind::HL, ArgKind::SP)), // ADD HL,SP
        
        // 8-bit ADD instructions
        0x80 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::B)),
        0x81 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::C)),
        0x82 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::D)),
        0x83 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::E)),
        0x84 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::H)),
        0x85 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::L)),
        0x86 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::HL)), // ADD A,(HL) - memory version
        0x87 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::A)),
        0xC6 => Some(InstructionKind::ADD(ArgKind::A, ArgKind::Immediate(immediate.unwrap_or(0)))), // ADD A,d8
        
        // ADC instructions (add with carry)
        0x88 => Some(InstructionKind::ADC(ArgKind::A, ArgKind::B)),
        0x89 => Some(InstructionKind::ADC(ArgKind::A, ArgKind::C)),
        0x8A => Some(InstructionKind::ADC(ArgKind::A, ArgKind::D)),
        0x8B => Some(InstructionKind::ADC(ArgKind::A, ArgKind::E)),
        0x8C => Some(InstructionKind::ADC(ArgKind::A, ArgKind::H)),
        0x8D => Some(InstructionKind::ADC(ArgKind::A, ArgKind::L)),
        0x8E => Some(InstructionKind::ADC(ArgKind::A, ArgKind::HL)), // ADC A,(HL)
        0x8F => Some(InstructionKind::ADC(ArgKind::A, ArgKind::A)),
        0xCE => Some(InstructionKind::ADC(ArgKind::A, ArgKind::Immediate(immediate.unwrap_or(0)))), // ADC A,d8
        
        // SUB instructions
        0x90 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::B)),
        0x91 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::C)),
        0x92 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::D)),
        0x93 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::E)),
        0x94 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::H)),
        0x95 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::L)),
        0x96 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::HL)), // SUB A,(HL)
        0x97 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::A)),
        0xD6 => Some(InstructionKind::SUB(ArgKind::A, ArgKind::Immediate(immediate.unwrap_or(0)))), // SUB A,d8
        
        // SBC instructions (subtract with carry)
        0x98 => Some(InstructionKind::SBC(ArgKind::A, ArgKind::B)),
        0x99 => Some(InstructionKind::SBC(ArgKind::A, ArgKind::C)),
        0x9A => Some(InstructionKind::SBC(ArgKind::A, ArgKind::D)),
        0x9B => Some(InstructionKind::SBC(ArgKind::A, ArgKind::E)),
        0x9C => Some(InstructionKind::SBC(ArgKind::A, ArgKind::H)),
        0x9D => Some(InstructionKind::SBC(ArgKind::A, ArgKind::L)),
        0x9E => Some(InstructionKind::SBC(ArgKind::A, ArgKind::HL)), // SBC A,(HL)
        0x9F => Some(InstructionKind::SBC(ArgKind::A, ArgKind::A)),
        0xDE => Some(InstructionKind::SBC(ArgKind::A, ArgKind::Immediate(immediate.unwrap_or(0)))), // SBC A,d8
        
        _ => None,
    }
}

pub fn get_arithmetic_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // 16-bit ADD and 8-bit arithmetic register instructions are all 1 byte
        0x09 | 0x19 | 0x29 | 0x39 |
        0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x85 | 0x86 | 0x87 |
        0x88 | 0x89 | 0x8A | 0x8B | 0x8C | 0x8D | 0x8E | 0x8F |
        0x90 | 0x91 | 0x92 | 0x93 | 0x94 | 0x95 | 0x96 | 0x97 |
        0x98 | 0x99 | 0x9A | 0x9B | 0x9C | 0x9D | 0x9E | 0x9F => Some(1),
        
        // Immediate arithmetic instructions are 2 bytes
        0xC6 | 0xCE | 0xD6 | 0xDE => Some(2),
        _ => None,
    }
}