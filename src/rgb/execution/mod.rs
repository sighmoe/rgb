pub mod data_transfer;
pub mod arithmetic;
pub mod logic;
pub mod bit_operations;
pub mod control_flow;
pub mod stack_operations;
pub mod system_control;

use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::{Instruction, InstructionKind};

impl Cpu {
    pub fn execute_instruction(&mut self, instruction: Instruction) -> u8 {
        match instruction.kind {
            // Data Transfer Instructions
            InstructionKind::LD(..) |
            InstructionKind::LDHL_SP_R8(..) |
            InstructionKind::LD_SP_TO_MEM(..) |
            InstructionKind::LD_MEM(..) |
            InstructionKind::LD_MEM_DEC(..) |
            InstructionKind::LD_MEM_INC(..) |
            InstructionKind::LD_FROM_MEM(..) |
            InstructionKind::LD_FROM_MEM_DEC(..) |
            InstructionKind::LD_FROM_MEM_INC(..) |
            InstructionKind::LD_MEM_16(..) |
            InstructionKind::LD_FROM_MEM_16(..) |
            InstructionKind::LDH_TO_C(..) |
            InstructionKind::LDH_FROM_C(..) |
            InstructionKind::LDH_TO_N(..) |
            InstructionKind::LDH_FROM_N(..) => {
                data_transfer::execute(self, &instruction.kind)
            }

            // Arithmetic Instructions
            InstructionKind::ADD(..) |
            InstructionKind::SUB(..) |
            InstructionKind::ADC(..) |
            InstructionKind::SBC(..) |
            InstructionKind::INC(..) |
            InstructionKind::INC_MEM(..) |
            InstructionKind::DEC(..) |
            InstructionKind::DEC_MEM(..) |
            InstructionKind::INC16(..) |
            InstructionKind::DEC16(..) |
            InstructionKind::DAA |
            InstructionKind::ADD_SP_R8(..) => {
                arithmetic::execute(self, &instruction.kind)
            }

            // Logic Instructions
            InstructionKind::AND(..) |
            InstructionKind::OR(..) |
            InstructionKind::XOR(..) |
            InstructionKind::CP(..) |
            InstructionKind::CP_MEM(..) |
            InstructionKind::CPL |
            InstructionKind::SCF |
            InstructionKind::CCF => {
                logic::execute(self, &instruction.kind)
            }

            // Bit Operations
            InstructionKind::BIT(..) |
            InstructionKind::SET(..) |
            InstructionKind::RES(..) |
            InstructionKind::RL(..) |
            InstructionKind::RR(..) |
            InstructionKind::RLC(..) |
            InstructionKind::RRC(..) |
            InstructionKind::SLA(..) |
            InstructionKind::SRA(..) |
            InstructionKind::SRL(..) |
            InstructionKind::SWAP(..) |
            InstructionKind::RLCA |
            InstructionKind::RRCA |
            InstructionKind::RLA |
            InstructionKind::RRA => {
                bit_operations::execute(self, &instruction.kind)
            }

            // Control Flow Instructions
            InstructionKind::JP(..) |
            InstructionKind::JP_HL |
            InstructionKind::JR(..) |
            InstructionKind::CALL(..) |
            InstructionKind::CALL_COND(..) |
            InstructionKind::RET |
            InstructionKind::RET_COND(..) |
            InstructionKind::RETI |
            InstructionKind::RST(..) => {
                control_flow::execute(self, &instruction.kind)
            }

            // Stack Operations
            InstructionKind::PUSH(..) |
            InstructionKind::POP(..) => {
                stack_operations::execute(self, &instruction.kind)
            }

            // System Control Instructions
            InstructionKind::NOP |
            InstructionKind::HALT |
            InstructionKind::STOP |
            InstructionKind::EI |
            InstructionKind::DI => {
                system_control::execute(self, &instruction.kind)
            }
        }
    }
}