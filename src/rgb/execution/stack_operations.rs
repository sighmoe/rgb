use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::{InstructionKind, ArgKind};

pub fn execute(cpu: &mut Cpu, instruction: &InstructionKind) -> u8 {
    match instruction {
        InstructionKind::PUSH(reg_pair) => {
            execute_push(cpu, reg_pair)
        }
        InstructionKind::POP(reg_pair) => {
            execute_pop(cpu, reg_pair)
        }
        _ => panic!("Invalid stack operation instruction"),
    }
}

fn execute_push(cpu: &mut Cpu, reg_pair: &ArgKind) -> u8 {
    let value = match reg_pair {
        ArgKind::BC => cpu.registers.get_bc(),
        ArgKind::DE => cpu.registers.get_de(),
        ArgKind::HL => cpu.registers.get_hl(),
        ArgKind::AF => cpu.registers.get_af(),
        _ => panic!("Unsupported PUSH register pair"),
    };
    cpu.push_stack(value);
    16
}

fn execute_pop(cpu: &mut Cpu, reg_pair: &ArgKind) -> u8 {
    let value = cpu.pop_stack();
    match reg_pair {
        ArgKind::BC => cpu.registers.set_bc(value),
        ArgKind::DE => cpu.registers.set_de(value),
        ArgKind::HL => cpu.registers.set_hl(value),
        ArgKind::AF => cpu.registers.set_af(value),
        _ => panic!("Unsupported POP register pair"),
    }
    12
}