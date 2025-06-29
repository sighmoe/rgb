use super::{InstructionKind, ArgKind};

pub fn decode_logical_instruction(opcode: u8) -> Option<InstructionKind> {
    match opcode {
        // XOR instructions
        0xAF => Some(InstructionKind::XOR(ArgKind::A, ArgKind::A)), // XOR A,A
        0xA8 => Some(InstructionKind::XOR(ArgKind::A, ArgKind::B)), // XOR A,B
        0xA9 => Some(InstructionKind::XOR(ArgKind::A, ArgKind::C)), // XOR A,C
        0xAA => Some(InstructionKind::XOR(ArgKind::A, ArgKind::D)), // XOR A,D
        0xAB => Some(InstructionKind::XOR(ArgKind::A, ArgKind::E)), // XOR A,E
        0xAC => Some(InstructionKind::XOR(ArgKind::A, ArgKind::H)), // XOR A,H
        0xAD => Some(InstructionKind::XOR(ArgKind::A, ArgKind::L)), // XOR A,L
        0xEE => Some(InstructionKind::XOR(ArgKind::A, ArgKind::Immediate(0))), // XOR A,d8
        
        // AND instructions
        0xA7 => Some(InstructionKind::AND(ArgKind::A, ArgKind::A)), // AND A,A
        0xA0 => Some(InstructionKind::AND(ArgKind::A, ArgKind::B)), // AND A,B
        0xA1 => Some(InstructionKind::AND(ArgKind::A, ArgKind::C)), // AND A,C
        0xA2 => Some(InstructionKind::AND(ArgKind::A, ArgKind::D)), // AND A,D
        0xA3 => Some(InstructionKind::AND(ArgKind::A, ArgKind::E)), // AND A,E
        0xA4 => Some(InstructionKind::AND(ArgKind::A, ArgKind::H)), // AND A,H
        0xA5 => Some(InstructionKind::AND(ArgKind::A, ArgKind::L)), // AND A,L
        0xE6 => Some(InstructionKind::AND(ArgKind::A, ArgKind::Immediate(0))), // AND A,d8
        
        // OR instructions
        0xB7 => Some(InstructionKind::OR(ArgKind::A, ArgKind::A)), // OR A,A
        0xB0 => Some(InstructionKind::OR(ArgKind::A, ArgKind::B)), // OR A,B
        0xB1 => Some(InstructionKind::OR(ArgKind::A, ArgKind::C)), // OR A,C
        0xB2 => Some(InstructionKind::OR(ArgKind::A, ArgKind::D)), // OR A,D
        0xB3 => Some(InstructionKind::OR(ArgKind::A, ArgKind::E)), // OR A,E
        0xB4 => Some(InstructionKind::OR(ArgKind::A, ArgKind::H)), // OR A,H
        0xB5 => Some(InstructionKind::OR(ArgKind::A, ArgKind::L)), // OR A,L
        0xF6 => Some(InstructionKind::OR(ArgKind::A, ArgKind::Immediate(0))), // OR A,d8
        
        // CP (Compare) instructions
        0xBF => Some(InstructionKind::CP(ArgKind::A, ArgKind::A)), // CP A,A
        0xB8 => Some(InstructionKind::CP(ArgKind::A, ArgKind::B)), // CP A,B
        0xB9 => Some(InstructionKind::CP(ArgKind::A, ArgKind::C)), // CP A,C
        0xBA => Some(InstructionKind::CP(ArgKind::A, ArgKind::D)), // CP A,D
        0xBB => Some(InstructionKind::CP(ArgKind::A, ArgKind::E)), // CP A,E
        0xBC => Some(InstructionKind::CP(ArgKind::A, ArgKind::H)), // CP A,H
        0xBD => Some(InstructionKind::CP(ArgKind::A, ArgKind::L)), // CP A,L
        0xFE => Some(InstructionKind::CP(ArgKind::A, ArgKind::Immediate(0))), // CP A,d8
        
        _ => None,
    }
}

pub fn get_logical_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // Register to register operations (1 byte)
        0xAF | 0xA8..=0xAD | 0xA7 | 0xA0..=0xA5 | 
        0xB7 | 0xB0..=0xB5 | 0xBF | 0xB8..=0xBD => Some(1),
        
        // Immediate operations (2 bytes: opcode + immediate)
        0xEE | 0xE6 | 0xF6 | 0xFE => Some(2),
        
        _ => None,
    }
}