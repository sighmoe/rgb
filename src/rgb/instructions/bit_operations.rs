use super::{InstructionKind, ArgKind};

pub fn decode_bit_instruction(opcode: u16) -> Option<InstructionKind> {
    // CB-prefixed instructions are 16-bit opcodes (0xCBxx)
    match opcode {
        // Rotate left through carry (RL)
        0xCB10 => Some(InstructionKind::RL(ArgKind::B)),
        0xCB11 => Some(InstructionKind::RL(ArgKind::C)),
        0xCB12 => Some(InstructionKind::RL(ArgKind::D)),
        0xCB13 => Some(InstructionKind::RL(ArgKind::E)),
        0xCB14 => Some(InstructionKind::RL(ArgKind::H)),
        0xCB15 => Some(InstructionKind::RL(ArgKind::L)),
        0xCB17 => Some(InstructionKind::RL(ArgKind::A)),
        
        // Rotate right through carry (RR)
        0xCB18 => Some(InstructionKind::RR(ArgKind::B)),
        0xCB19 => Some(InstructionKind::RR(ArgKind::C)),
        0xCB1A => Some(InstructionKind::RR(ArgKind::D)),
        0xCB1B => Some(InstructionKind::RR(ArgKind::E)),
        0xCB1C => Some(InstructionKind::RR(ArgKind::H)),
        0xCB1D => Some(InstructionKind::RR(ArgKind::L)),
        0xCB1F => Some(InstructionKind::RR(ArgKind::A)),
        // BIT 0, r
        0xCB40 => Some(InstructionKind::BIT(0, ArgKind::B)),
        0xCB41 => Some(InstructionKind::BIT(0, ArgKind::C)),
        0xCB42 => Some(InstructionKind::BIT(0, ArgKind::D)),
        0xCB43 => Some(InstructionKind::BIT(0, ArgKind::E)),
        0xCB44 => Some(InstructionKind::BIT(0, ArgKind::H)),
        0xCB45 => Some(InstructionKind::BIT(0, ArgKind::L)),
        0xCB47 => Some(InstructionKind::BIT(0, ArgKind::A)),
        
        // BIT 1, r
        0xCB48 => Some(InstructionKind::BIT(1, ArgKind::B)),
        0xCB49 => Some(InstructionKind::BIT(1, ArgKind::C)),
        0xCB4A => Some(InstructionKind::BIT(1, ArgKind::D)),
        0xCB4B => Some(InstructionKind::BIT(1, ArgKind::E)),
        0xCB4C => Some(InstructionKind::BIT(1, ArgKind::H)),
        0xCB4D => Some(InstructionKind::BIT(1, ArgKind::L)),
        0xCB4F => Some(InstructionKind::BIT(1, ArgKind::A)),
        
        // BIT 2, r
        0xCB50 => Some(InstructionKind::BIT(2, ArgKind::B)),
        0xCB51 => Some(InstructionKind::BIT(2, ArgKind::C)),
        0xCB52 => Some(InstructionKind::BIT(2, ArgKind::D)),
        0xCB53 => Some(InstructionKind::BIT(2, ArgKind::E)),
        0xCB54 => Some(InstructionKind::BIT(2, ArgKind::H)),
        0xCB55 => Some(InstructionKind::BIT(2, ArgKind::L)),
        0xCB57 => Some(InstructionKind::BIT(2, ArgKind::A)),
        
        // BIT 3, r
        0xCB58 => Some(InstructionKind::BIT(3, ArgKind::B)),
        0xCB59 => Some(InstructionKind::BIT(3, ArgKind::C)),
        0xCB5A => Some(InstructionKind::BIT(3, ArgKind::D)),
        0xCB5B => Some(InstructionKind::BIT(3, ArgKind::E)),
        0xCB5C => Some(InstructionKind::BIT(3, ArgKind::H)),
        0xCB5D => Some(InstructionKind::BIT(3, ArgKind::L)),
        0xCB5F => Some(InstructionKind::BIT(3, ArgKind::A)),
        
        // BIT 4, r
        0xCB60 => Some(InstructionKind::BIT(4, ArgKind::B)),
        0xCB61 => Some(InstructionKind::BIT(4, ArgKind::C)),
        0xCB62 => Some(InstructionKind::BIT(4, ArgKind::D)),
        0xCB63 => Some(InstructionKind::BIT(4, ArgKind::E)),
        0xCB64 => Some(InstructionKind::BIT(4, ArgKind::H)),
        0xCB65 => Some(InstructionKind::BIT(4, ArgKind::L)),
        0xCB67 => Some(InstructionKind::BIT(4, ArgKind::A)),
        
        // BIT 5, r
        0xCB68 => Some(InstructionKind::BIT(5, ArgKind::B)),
        0xCB69 => Some(InstructionKind::BIT(5, ArgKind::C)),
        0xCB6A => Some(InstructionKind::BIT(5, ArgKind::D)),
        0xCB6B => Some(InstructionKind::BIT(5, ArgKind::E)),
        0xCB6C => Some(InstructionKind::BIT(5, ArgKind::H)),
        0xCB6D => Some(InstructionKind::BIT(5, ArgKind::L)),
        0xCB6F => Some(InstructionKind::BIT(5, ArgKind::A)),
        
        // BIT 6, r
        0xCB70 => Some(InstructionKind::BIT(6, ArgKind::B)),
        0xCB71 => Some(InstructionKind::BIT(6, ArgKind::C)),
        0xCB72 => Some(InstructionKind::BIT(6, ArgKind::D)),
        0xCB73 => Some(InstructionKind::BIT(6, ArgKind::E)),
        0xCB74 => Some(InstructionKind::BIT(6, ArgKind::H)),
        0xCB75 => Some(InstructionKind::BIT(6, ArgKind::L)),
        0xCB77 => Some(InstructionKind::BIT(6, ArgKind::A)),
        
        // BIT 7, r
        0xCB78 => Some(InstructionKind::BIT(7, ArgKind::B)),
        0xCB79 => Some(InstructionKind::BIT(7, ArgKind::C)),
        0xCB7A => Some(InstructionKind::BIT(7, ArgKind::D)),
        0xCB7B => Some(InstructionKind::BIT(7, ArgKind::E)),
        0xCB7C => Some(InstructionKind::BIT(7, ArgKind::H)),
        0xCB7D => Some(InstructionKind::BIT(7, ArgKind::L)),
        0xCB7F => Some(InstructionKind::BIT(7, ArgKind::A)),
        
        _ => None,
    }
}

pub fn get_bit_instruction_size(opcode: u16) -> Option<u16> {
    // All CB-prefixed instructions are 2 bytes
    if opcode >= 0xCB40 && opcode <= 0xCB7F {
        Some(2)
    } else {
        None
    }
}