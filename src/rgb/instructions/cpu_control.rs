use super::InstructionKind;

pub fn decode_cpu_control_instruction(opcode: u8) -> Option<InstructionKind> {
    match opcode {
        0x76 => Some(InstructionKind::HALT),
        0xF3 => Some(InstructionKind::DI),   // Disable interrupts
        0xFB => Some(InstructionKind::EI),   // Enable interrupts
        0x00 => Some(InstructionKind::NOP),  // No operation
        0x17 => Some(InstructionKind::RLA),  // Rotate A left
        0x1F => Some(InstructionKind::RRA),  // Rotate A right
        0x10 => Some(InstructionKind::STOP), // Stop
        _ => None,
    }
}

pub fn get_cpu_control_instruction_size(opcode: u8) -> Option<u16> {
    match opcode {
        0x76 => Some(1), // HALT
        0xF3 => Some(1), // DI
        0xFB => Some(1), // EI
        0x00 => Some(1), // NOP
        0x17 => Some(1), // RLA
        0x1F => Some(1), // RRA
        0x10 => Some(2), // STOP (2 bytes: opcode + 0x00)
        _ => None,
    }
}