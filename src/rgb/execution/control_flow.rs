use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::{InstructionKind, JumpCondition};

pub fn execute(cpu: &mut Cpu, instruction: &InstructionKind) -> u8 {
    match instruction {
        InstructionKind::JP(condition, address) => {
            execute_jp(cpu, *condition, *address)
        }
        InstructionKind::JP_HL => {
            execute_jp_hl(cpu)
        }
        InstructionKind::JR(condition, offset) => {
            execute_jr(cpu, *condition, *offset)
        }
        InstructionKind::CALL(address) => {
            execute_call(cpu, *address)
        }
        InstructionKind::CALL_COND(condition, address) => {
            execute_call_cond(cpu, *condition, *address)
        }
        InstructionKind::RET => {
            execute_ret(cpu)
        }
        InstructionKind::RET_COND(condition) => {
            execute_ret_cond(cpu, *condition)
        }
        InstructionKind::RETI => {
            execute_reti(cpu)
        }
        InstructionKind::RST(addr) => {
            execute_rst(cpu, *addr)
        }
        _ => panic!("Invalid control flow instruction"),
    }
}

fn execute_jp(cpu: &mut Cpu, condition: JumpCondition, address: u16) -> u8 {
    if cpu.check_jump_condition(condition) {
        cpu.pc = address;
        16
    } else {
        12
    }
}

fn execute_jp_hl(cpu: &mut Cpu) -> u8 {
    cpu.pc = cpu.registers.get_hl();
    4
}

fn execute_jr(cpu: &mut Cpu, condition: JumpCondition, offset: i8) -> u8 {
    if cpu.check_jump_condition(condition) {
        cpu.pc = ((cpu.pc as i32) + (offset as i32)) as u16;
        12
    } else {
        8
    }
}

fn execute_call(cpu: &mut Cpu, address: u16) -> u8 {
    cpu.push_stack(cpu.pc);
    cpu.pc = address;
    24
}

fn execute_call_cond(cpu: &mut Cpu, condition: JumpCondition, address: u16) -> u8 {
    if cpu.check_jump_condition(condition) {
        cpu.push_stack(cpu.pc);
        cpu.pc = address;
        24
    } else {
        12
    }
}

fn execute_ret(cpu: &mut Cpu) -> u8 {
    let return_addr = cpu.pop_stack();
    cpu.pc = return_addr;
    16
}

fn execute_ret_cond(cpu: &mut Cpu, condition: JumpCondition) -> u8 {
    if cpu.check_jump_condition(condition) {
        cpu.pc = cpu.pop_stack();
        20
    } else {
        8
    }
}

fn execute_reti(cpu: &mut Cpu) -> u8 {
    // Return from interrupt: pop PC from stack and enable interrupts
    cpu.pc = cpu.pop_stack();
    // Enable interrupts immediately (no delay like EI)
    cpu.ime = true;
    16
}

fn execute_rst(cpu: &mut Cpu, addr: u8) -> u8 {
    cpu.push_stack(cpu.pc);
    cpu.pc = addr as u16;
    16
}