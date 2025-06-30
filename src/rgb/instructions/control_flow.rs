use super::InstructionKind;

#[derive(Clone, Copy, PartialEq)]
pub enum JumpCondition {
    Always,
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

pub fn decode_control_flow_instruction(opcode: u8, immediate: Option<u8>, immediate16: Option<u16>) -> Option<InstructionKind> {
    match opcode {
        // Jump instructions
        0xC3 => Some(InstructionKind::JP(JumpCondition::Always, immediate16.unwrap_or(0))),
        0xC2 => Some(InstructionKind::JP(JumpCondition::NotZero, immediate16.unwrap_or(0))),  
        0xCA => Some(InstructionKind::JP(JumpCondition::Zero, immediate16.unwrap_or(0))),     
        0xD2 => Some(InstructionKind::JP(JumpCondition::NotCarry, immediate16.unwrap_or(0))), 
        0xDA => Some(InstructionKind::JP(JumpCondition::Carry, immediate16.unwrap_or(0))),
        0xE9 => Some(InstructionKind::JP_HL), // JP (HL) - jump to address in HL    
        
        // Jump relative instructions
        0x18 => Some(InstructionKind::JR(JumpCondition::Always, immediate.unwrap_or(0) as i8)),
        0x20 => Some(InstructionKind::JR(JumpCondition::NotZero, immediate.unwrap_or(0) as i8)),
        0x28 => Some(InstructionKind::JR(JumpCondition::Zero, immediate.unwrap_or(0) as i8)),
        0x30 => Some(InstructionKind::JR(JumpCondition::NotCarry, immediate.unwrap_or(0) as i8)),
        0x38 => Some(InstructionKind::JR(JumpCondition::Carry, immediate.unwrap_or(0) as i8)),
        
        // Call and return instructions
        0xCD => Some(InstructionKind::CALL(immediate16.unwrap_or(0))),
        
        // Conditional call instructions
        0xC4 => Some(InstructionKind::CALL_COND(JumpCondition::NotZero, immediate16.unwrap_or(0))),
        0xCC => Some(InstructionKind::CALL_COND(JumpCondition::Zero, immediate16.unwrap_or(0))),
        0xD4 => Some(InstructionKind::CALL_COND(JumpCondition::NotCarry, immediate16.unwrap_or(0))),
        0xDC => Some(InstructionKind::CALL_COND(JumpCondition::Carry, immediate16.unwrap_or(0))),
        
        0xC9 => Some(InstructionKind::RET),
        
        // Conditional return instructions
        0xC0 => Some(InstructionKind::RET_COND(JumpCondition::NotZero)),
        0xC8 => Some(InstructionKind::RET_COND(JumpCondition::Zero)),
        0xD0 => Some(InstructionKind::RET_COND(JumpCondition::NotCarry)),
        0xD8 => Some(InstructionKind::RET_COND(JumpCondition::Carry)),
        
        // RST (restart) instructions
        0xC7 => Some(InstructionKind::RST(0x00)),
        0xCF => Some(InstructionKind::RST(0x08)),
        0xD7 => Some(InstructionKind::RST(0x10)),
        0xDF => Some(InstructionKind::RST(0x18)),
        0xE7 => Some(InstructionKind::RST(0x20)),
        0xEF => Some(InstructionKind::RST(0x28)),
        0xF7 => Some(InstructionKind::RST(0x30)),
        0xFF => Some(InstructionKind::RST(0x38)),
        
        _ => None,
    }
}

pub fn get_control_flow_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // Jump instructions (3 bytes: opcode + 16-bit address)
        0xC3 | 0xC2 | 0xCA | 0xD2 | 0xDA => Some(3),
        
        // JP (HL) instruction (1 byte: opcode only)
        0xE9 => Some(1),
        
        // Jump relative instructions (2 bytes: opcode + 8-bit signed offset)
        0x18 | 0x20 | 0x28 | 0x30 | 0x38 => Some(2),
        
        // Call instructions (3 bytes: opcode + 16-bit address)
        0xCD | 0xC4 | 0xCC | 0xD4 | 0xDC => Some(3),
        
        // Return instructions (1 byte: opcode only)
        0xC9 | 0xC0 | 0xC8 | 0xD0 | 0xD8 => Some(1),
        
        // RST instructions (1 byte: opcode only)
        0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => Some(1),
        
        _ => None,
    }
}