// Instruction cycle timing based on dmgops.json reference
use super::instructions::{InstructionKind, JumpCondition};

pub fn get_instruction_cycles(instruction: &InstructionKind, condition_taken: bool) -> u8 {
    match instruction {
        // Control flow instructions with conditional timing
        InstructionKind::JP(condition, _) => {
            match condition {
                JumpCondition::Always => 16,
                _ => if condition_taken { 16 } else { 12 },
            }
        }
        InstructionKind::JP_HL => 4,
        InstructionKind::JR(condition, _) => {
            match condition {
                JumpCondition::Always => 12,
                _ => if condition_taken { 12 } else { 8 },
            }
        }
        InstructionKind::CALL(_) => 24,
        InstructionKind::CALL_COND(_, _) => {
            if condition_taken { 24 } else { 12 }
        }
        InstructionKind::RET => 16,
        InstructionKind::RET_COND(_) => {
            if condition_taken { 20 } else { 8 }
        }
        InstructionKind::RETI => 16, // RETI takes 16 cycles
        InstructionKind::RST(_) => 16,
        
        // Load instructions
        InstructionKind::LD(dest, src) => {
            // This is simplified - would need to check specific dest/src combinations
            // for accurate timing from dmgops.json
            match (dest, src) {
                // 16-bit immediate loads: 12 cycles
                (super::instructions::ArgKind::BC, super::instructions::ArgKind::Immediate16(_)) |
                (super::instructions::ArgKind::DE, super::instructions::ArgKind::Immediate16(_)) |
                (super::instructions::ArgKind::HL, super::instructions::ArgKind::Immediate16(_)) |
                (super::instructions::ArgKind::SP, super::instructions::ArgKind::Immediate16(_)) => 12,
                
                // 8-bit immediate loads: 8 cycles
                (_, super::instructions::ArgKind::Immediate(_)) => 8,
                
                // Special case: LD SP,HL takes 8 cycles
                (super::instructions::ArgKind::SP, super::instructions::ArgKind::HL) => 8,
                
                // Register to register: 4 cycles
                _ => 4,
            }
        }
        
        // Memory load/store instructions
        InstructionKind::LD_MEM(_, src) => {
            match src {
                super::instructions::ArgKind::Immediate(_) => 12, // LD (HL),u8: 12 cycles
                _ => 8, // LD (HL),r: 8 cycles
            }
        },
        InstructionKind::LD_MEM_INC(_, _) => 8,
        InstructionKind::LD_MEM_DEC(_, _) => 8,
        InstructionKind::LD_FROM_MEM(_, _) => 8,
        InstructionKind::LD_FROM_MEM_INC(_, _) => 8,
        InstructionKind::LD_FROM_MEM_DEC(_, _) => 8,
        InstructionKind::LD_MEM_16(_, _) => 16,  // LD (nn),A: 16 cycles
        InstructionKind::LD_FROM_MEM_16(_, _) => 16, // LD A,(nn): 16 cycles
        InstructionKind::LDHL_SP_R8(_) => 12, // LD HL,SP+r8: 12 cycles
        InstructionKind::LD_SP_TO_MEM(_) => 20, // LD (nn),SP: 20 cycles
        
        // Arithmetic instructions
        InstructionKind::ADD(dest, src) => {
            match (dest, src) {
                // 16-bit ADD: 8 cycles
                (super::instructions::ArgKind::HL, _) => 8,
                // 8-bit ADD with memory: 8 cycles
                (_, super::instructions::ArgKind::HL) => 8,
                // 8-bit ADD immediate: 8 cycles
                (_, super::instructions::ArgKind::Immediate(_)) => 8,
                // 8-bit ADD register: 4 cycles
                _ => 4,
            }
        }
        InstructionKind::ADC(_, src) => {
            match src {
                super::instructions::ArgKind::HL => 8, // ADC (HL): 8 cycles
                super::instructions::ArgKind::Immediate(_) => 8, // ADC d8: 8 cycles
                _ => 4, // ADC r: 4 cycles
            }
        }
        InstructionKind::SUB(_, src) => {
            match src {
                super::instructions::ArgKind::HL => 8, // SUB (HL): 8 cycles
                super::instructions::ArgKind::Immediate(_) => 8, // SUB d8: 8 cycles
                _ => 4, // SUB r: 4 cycles
            }
        }
        InstructionKind::SBC(_, src) => {
            match src {
                super::instructions::ArgKind::HL => 8, // SBC (HL): 8 cycles
                super::instructions::ArgKind::Immediate(_) => 8, // SBC d8: 8 cycles
                _ => 4, // SBC r: 4 cycles
            }
        }
        
        // Increment/Decrement
        InstructionKind::INC(_) => 4,
        InstructionKind::DEC(_) => 4,
        InstructionKind::INC_MEM(_) => 12, // INC (HL): 12 cycles
        InstructionKind::DEC_MEM(_) => 12, // DEC (HL): 12 cycles
        InstructionKind::INC16(_) => 8,
        InstructionKind::DEC16(_) => 8,
        
        // Bit operations (CB-prefixed)
        InstructionKind::BIT(_, _) => 8,
        InstructionKind::RES(_, _) => 8,
        InstructionKind::SET(_, _) => 8,
        InstructionKind::RL(_) => 8,
        InstructionKind::RR(_) => 8,
        InstructionKind::RLC(_) => 8,
        InstructionKind::RRC(_) => 8,
        InstructionKind::SLA(_) => 8,
        InstructionKind::SRA(_) => 8,
        InstructionKind::SRL(_) => 8,
        
        // Logical operations
        InstructionKind::XOR(_, src) => {
            match src {
                super::instructions::ArgKind::Immediate(_) => 8, // XOR A,u8: 8 cycles
                super::instructions::ArgKind::HL => 8, // XOR A,(HL): 8 cycles
                _ => 4, // XOR A,r: 4 cycles
            }
        }
        InstructionKind::AND(_, src) => {
            match src {
                super::instructions::ArgKind::Immediate(_) => 8, // AND A,u8: 8 cycles
                super::instructions::ArgKind::HL => 8, // AND A,(HL): 8 cycles
                _ => 4, // AND A,r: 4 cycles
            }
        }
        InstructionKind::OR(_, src) => {
            match src {
                super::instructions::ArgKind::Immediate(_) => 8, // OR A,u8: 8 cycles
                super::instructions::ArgKind::HL => 8, // OR A,(HL): 8 cycles
                _ => 4, // OR A,r: 4 cycles
            }
        }
        InstructionKind::CP(_, src) => {
            match src {
                super::instructions::ArgKind::Immediate(_) => 8, // CP d8: 8 cycles
                _ => 4, // CP r: 4 cycles
            }
        }
        InstructionKind::CP_MEM(_, _) => 8,
        
        // I/O operations
        InstructionKind::LDH_TO_C(_) => 8,
        InstructionKind::LDH_FROM_C(_) => 8,
        InstructionKind::LDH_TO_N(_, _) => 12,
        InstructionKind::LDH_FROM_N(_, _) => 12,
        
        // Stack operations
        InstructionKind::PUSH(_) => 16,
        InstructionKind::POP(_) => 12,
        
        // Rotate operations
        InstructionKind::RLA => 4,
        InstructionKind::RRA => 4,
        InstructionKind::RLCA => 4,
        InstructionKind::RRCA => 4,
        InstructionKind::SWAP(_) => 8, // SWAP takes 8 cycles
        
        // Misc
        InstructionKind::NOP => 4,
        InstructionKind::HALT => 4,
        InstructionKind::STOP => 4,
        InstructionKind::EI => 4,
        InstructionKind::DI => 4,
        InstructionKind::DAA => 4, // DAA takes 4 cycles
    }
}