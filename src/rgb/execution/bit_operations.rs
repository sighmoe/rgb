use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::{InstructionKind, ArgKind};

// Helper functions for reading and writing values from/to registers or memory
fn read_value(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    match reg {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        ArgKind::HL => {
            let addr = cpu.registers.get_hl();
            cpu.mmap.read(addr)
        }
        _ => panic!("Unsupported register/memory location"),
    }
}

fn write_value(cpu: &mut Cpu, reg: &ArgKind, value: u8) {
    match reg {
        ArgKind::A => cpu.registers.a = value,
        ArgKind::B => cpu.registers.b = value,
        ArgKind::C => cpu.registers.c = value,
        ArgKind::D => cpu.registers.d = value,
        ArgKind::E => cpu.registers.e = value,
        ArgKind::H => cpu.registers.h = value,
        ArgKind::L => cpu.registers.l = value,
        ArgKind::HL => {
            let addr = cpu.registers.get_hl();
            cpu.mmap.write(addr, value);
        }
        _ => panic!("Unsupported register/memory location"),
    }
}

fn get_cycles(reg: &ArgKind) -> u8 {
    match reg {
        ArgKind::HL => 16, // (HL) operations take 16 cycles for rotate/shift
        _ => 8, // Register operations take 8 cycles
    }
}

fn get_bit_cycles(reg: &ArgKind) -> u8 {
    match reg {
        ArgKind::HL => 12, // (HL) BIT operations take 12 cycles
        _ => 8, // Register BIT operations take 8 cycles
    }
}

pub fn execute(cpu: &mut Cpu, instruction: &InstructionKind) -> u8 {
    match instruction {
        InstructionKind::BIT(bit, register) => {
            execute_bit(cpu, *bit, register)
        }
        InstructionKind::SET(bit, register) => {
            execute_set(cpu, *bit, register)
        }
        InstructionKind::RES(bit, register) => {
            execute_res(cpu, *bit, register)
        }
        InstructionKind::RL(reg) => {
            execute_rl(cpu, reg)
        }
        InstructionKind::RR(reg) => {
            execute_rr(cpu, reg)
        }
        InstructionKind::RLC(reg) => {
            execute_rlc(cpu, reg)
        }
        InstructionKind::RRC(reg) => {
            execute_rrc(cpu, reg)
        }
        InstructionKind::SLA(reg) => {
            execute_sla(cpu, reg)
        }
        InstructionKind::SRA(reg) => {
            execute_sra(cpu, reg)
        }
        InstructionKind::SRL(reg) => {
            execute_srl(cpu, reg)
        }
        InstructionKind::SWAP(reg) => {
            execute_swap(cpu, reg)
        }
        InstructionKind::RLCA => {
            execute_rlca(cpu)
        }
        InstructionKind::RRCA => {
            execute_rrca(cpu)
        }
        InstructionKind::RLA => {
            execute_rla(cpu)
        }
        InstructionKind::RRA => {
            execute_rra(cpu)
        }
        _ => panic!("Invalid bit operation instruction"),
    }
}

fn execute_bit(cpu: &mut Cpu, bit: u8, register: &ArgKind) -> u8 {
    let value = read_value(cpu, register);
    cpu.test_bit(bit, value);
    get_bit_cycles(register)
}

fn execute_set(cpu: &mut Cpu, bit: u8, register: &ArgKind) -> u8 {
    let mask = 1 << bit;
    let value = read_value(cpu, register);
    write_value(cpu, register, value | mask);
    get_cycles(register)
}

fn execute_res(cpu: &mut Cpu, bit: u8, register: &ArgKind) -> u8 {
    let mask = !(1 << bit);
    let value = read_value(cpu, register);
    write_value(cpu, register, value & mask);
    get_cycles(register)
}

fn execute_rl(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = read_value(cpu, reg);
    
    let old_carry = if cpu.registers.f.carry { 1 } else { 0 };
    let new_carry = (value & 0x80) != 0;
    let result = (value << 1) | old_carry;
    
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = new_carry;
    
    write_value(cpu, reg, result);
    get_cycles(reg)
}

fn execute_rr(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = read_value(cpu, reg);
    
    let old_carry = if cpu.registers.f.carry { 0x80 } else { 0 };
    let new_carry = (value & 0x01) != 0;
    let result = (value >> 1) | old_carry;
    
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = new_carry;
    
    write_value(cpu, reg, result);
    get_cycles(reg)
}

fn execute_rlc(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = read_value(cpu, reg);
    let new_value = value.rotate_left(1);
    
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x80) != 0;
    
    write_value(cpu, reg, new_value);
    get_cycles(reg)
}

fn execute_rrc(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = read_value(cpu, reg);
    let new_value = value.rotate_right(1);
    
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x01) != 0;
    
    write_value(cpu, reg, new_value);
    get_cycles(reg)
}

fn execute_sla(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = read_value(cpu, reg);
    
    let new_value = value << 1;
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x80) != 0;
    
    write_value(cpu, reg, new_value);
    get_cycles(reg)
}

fn execute_sra(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = read_value(cpu, reg);
    
    let new_value = (value >> 1) | (value & 0x80); // Preserve sign bit
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x01) != 0;
    
    write_value(cpu, reg, new_value);
    get_cycles(reg)
}

fn execute_srl(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = read_value(cpu, reg);
    
    let new_value = value >> 1;
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x01) != 0;
    
    write_value(cpu, reg, new_value);
    get_cycles(reg)
}

fn execute_swap(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = read_value(cpu, reg);
    
    // Swap upper and lower nibbles
    let new_value = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    
    write_value(cpu, reg, new_value);
    get_cycles(reg)
}

fn execute_rlca(cpu: &mut Cpu) -> u8 {
    // Rotate A left circular
    let carry = (cpu.registers.a & 0x80) != 0;
    cpu.registers.a = (cpu.registers.a << 1) | (if carry { 1 } else { 0 });
    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;
    4
}

fn execute_rrca(cpu: &mut Cpu) -> u8 {
    // Rotate A right circular
    let carry = (cpu.registers.a & 0x01) != 0;
    cpu.registers.a = (cpu.registers.a >> 1) | (if carry { 0x80 } else { 0 });
    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;
    4
}

fn execute_rla(cpu: &mut Cpu) -> u8 {
    // Rotate A left through carry
    let old_carry = cpu.registers.f.carry;
    let new_carry = (cpu.registers.a & 0x80) != 0;
    cpu.registers.a = (cpu.registers.a << 1) | (if old_carry { 1 } else { 0 });
    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = new_carry;
    4
}

fn execute_rra(cpu: &mut Cpu) -> u8 {
    // Rotate A right through carry
    let old_carry = cpu.registers.f.carry;
    let new_carry = (cpu.registers.a & 0x01) != 0;
    cpu.registers.a = (cpu.registers.a >> 1) | (if old_carry { 0x80 } else { 0 });
    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = new_carry;
    4
}