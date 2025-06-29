use super::{InstructionKind, ArgKind};

#[derive(Clone, Copy)]
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
        
        // Jump relative instructions
        0x18 => Some(InstructionKind::JR(JumpCondition::Always, immediate.unwrap_or(0) as i8)),
        0x20 => Some(InstructionKind::JR(JumpCondition::NotZero, immediate.unwrap_or(0) as i8)),
        0x28 => Some(InstructionKind::JR(JumpCondition::Zero, immediate.unwrap_or(0) as i8)),
        0x30 => Some(InstructionKind::JR(JumpCondition::NotCarry, immediate.unwrap_or(0) as i8)),
        0x38 => Some(InstructionKind::JR(JumpCondition::Carry, immediate.unwrap_or(0) as i8)),
        
        // Call and return instructions
        0xCD => Some(InstructionKind::CALL(immediate16.unwrap_or(0))),
        0xC9 => Some(InstructionKind::RET),
        
        _ => None,
    }
}

pub fn get_control_flow_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        // Jump instructions (3 bytes: opcode + 16-bit address)
        0xC3 | 0xC2 | 0xCA | 0xD2 | 0xDA => Some(3),
        
        // Jump relative instructions (2 bytes: opcode + 8-bit signed offset)
        0x18 | 0x20 | 0x28 | 0x30 | 0x38 => Some(2),
        
        // Call instruction (3 bytes: opcode + 16-bit address)
        0xCD => Some(3),
        
        // Return instruction (1 byte: opcode only)
        0xC9 => Some(1),
        
        _ => None,
    }
}