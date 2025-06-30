use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::{InstructionKind, ArgKind};

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
    let value = match register {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        _ => panic!("Unsupported BIT instruction variant"),
    };
    cpu.test_bit(bit, value);
    8
}

fn execute_set(cpu: &mut Cpu, bit: u8, register: &ArgKind) -> u8 {
    let mask = 1 << bit;
    match register {
        ArgKind::A => cpu.registers.a |= mask,
        ArgKind::B => cpu.registers.b |= mask,
        ArgKind::C => cpu.registers.c |= mask,
        ArgKind::D => cpu.registers.d |= mask,
        ArgKind::E => cpu.registers.e |= mask,
        ArgKind::H => cpu.registers.h |= mask,
        ArgKind::L => cpu.registers.l |= mask,
        _ => panic!("Unsupported SET instruction variant"),
    }
    8
}

fn execute_res(cpu: &mut Cpu, bit: u8, register: &ArgKind) -> u8 {
    let mask = !(1 << bit);
    match register {
        ArgKind::A => cpu.registers.a &= mask,
        ArgKind::B => cpu.registers.b &= mask,
        ArgKind::C => cpu.registers.c &= mask,
        ArgKind::D => cpu.registers.d &= mask,
        ArgKind::E => cpu.registers.e &= mask,
        ArgKind::H => cpu.registers.h &= mask,
        ArgKind::L => cpu.registers.l &= mask,
        _ => panic!("Unsupported RES instruction variant"),
    }
    8
}

fn execute_rl(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = match reg {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        _ => panic!("Unsupported RL register"),
    };
    
    let old_carry = if cpu.registers.f.carry { 1 } else { 0 };
    let new_carry = (value & 0x80) != 0;
    let result = (value << 1) | old_carry;
    
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = new_carry;
    
    match reg {
        ArgKind::A => cpu.registers.a = result,
        ArgKind::B => cpu.registers.b = result,
        ArgKind::C => cpu.registers.c = result,
        ArgKind::D => cpu.registers.d = result,
        ArgKind::E => cpu.registers.e = result,
        ArgKind::H => cpu.registers.h = result,
        ArgKind::L => cpu.registers.l = result,
        _ => {}
    }
    8
}

fn execute_rr(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = match reg {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        _ => panic!("Unsupported RR register"),
    };
    
    let old_carry = if cpu.registers.f.carry { 0x80 } else { 0 };
    let new_carry = (value & 0x01) != 0;
    let result = (value >> 1) | old_carry;
    
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = new_carry;
    
    match reg {
        ArgKind::A => cpu.registers.a = result,
        ArgKind::B => cpu.registers.b = result,
        ArgKind::C => cpu.registers.c = result,
        ArgKind::D => cpu.registers.d = result,
        ArgKind::E => cpu.registers.e = result,
        ArgKind::H => cpu.registers.h = result,
        ArgKind::L => cpu.registers.l = result,
        _ => {}
    }
    8
}

fn execute_rlc(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let (value, new_value) = match reg {
        ArgKind::A => (cpu.registers.a, cpu.registers.a.rotate_left(1)),
        ArgKind::B => (cpu.registers.b, cpu.registers.b.rotate_left(1)),
        ArgKind::C => (cpu.registers.c, cpu.registers.c.rotate_left(1)),
        ArgKind::D => (cpu.registers.d, cpu.registers.d.rotate_left(1)),
        ArgKind::E => (cpu.registers.e, cpu.registers.e.rotate_left(1)),
        ArgKind::H => (cpu.registers.h, cpu.registers.h.rotate_left(1)),
        ArgKind::L => (cpu.registers.l, cpu.registers.l.rotate_left(1)),
        _ => panic!("Invalid RLC register"),
    };
    
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x80) != 0;
    
    match reg {
        ArgKind::A => cpu.registers.a = new_value,
        ArgKind::B => cpu.registers.b = new_value,
        ArgKind::C => cpu.registers.c = new_value,
        ArgKind::D => cpu.registers.d = new_value,
        ArgKind::E => cpu.registers.e = new_value,
        ArgKind::H => cpu.registers.h = new_value,
        ArgKind::L => cpu.registers.l = new_value,
        _ => {}
    }
    8
}

fn execute_rrc(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let (value, new_value) = match reg {
        ArgKind::A => (cpu.registers.a, cpu.registers.a.rotate_right(1)),
        ArgKind::B => (cpu.registers.b, cpu.registers.b.rotate_right(1)),
        ArgKind::C => (cpu.registers.c, cpu.registers.c.rotate_right(1)),
        ArgKind::D => (cpu.registers.d, cpu.registers.d.rotate_right(1)),
        ArgKind::E => (cpu.registers.e, cpu.registers.e.rotate_right(1)),
        ArgKind::H => (cpu.registers.h, cpu.registers.h.rotate_right(1)),
        ArgKind::L => (cpu.registers.l, cpu.registers.l.rotate_right(1)),
        _ => panic!("Invalid RRC register"),
    };
    
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x01) != 0;
    
    match reg {
        ArgKind::A => cpu.registers.a = new_value,
        ArgKind::B => cpu.registers.b = new_value,
        ArgKind::C => cpu.registers.c = new_value,
        ArgKind::D => cpu.registers.d = new_value,
        ArgKind::E => cpu.registers.e = new_value,
        ArgKind::H => cpu.registers.h = new_value,
        ArgKind::L => cpu.registers.l = new_value,
        _ => {}
    }
    8
}

fn execute_sla(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = match reg {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        _ => panic!("Invalid SLA register"),
    };
    
    let new_value = value << 1;
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x80) != 0;
    
    match reg {
        ArgKind::A => cpu.registers.a = new_value,
        ArgKind::B => cpu.registers.b = new_value,
        ArgKind::C => cpu.registers.c = new_value,
        ArgKind::D => cpu.registers.d = new_value,
        ArgKind::E => cpu.registers.e = new_value,
        ArgKind::H => cpu.registers.h = new_value,
        ArgKind::L => cpu.registers.l = new_value,
        _ => {}
    }
    8
}

fn execute_sra(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = match reg {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        _ => panic!("Invalid SRA register"),
    };
    
    let new_value = (value >> 1) | (value & 0x80); // Preserve sign bit
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x01) != 0;
    
    match reg {
        ArgKind::A => cpu.registers.a = new_value,
        ArgKind::B => cpu.registers.b = new_value,
        ArgKind::C => cpu.registers.c = new_value,
        ArgKind::D => cpu.registers.d = new_value,
        ArgKind::E => cpu.registers.e = new_value,
        ArgKind::H => cpu.registers.h = new_value,
        ArgKind::L => cpu.registers.l = new_value,
        _ => {}
    }
    8
}

fn execute_srl(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = match reg {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        _ => panic!("Invalid SRL register"),
    };
    
    let new_value = value >> 1;
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value & 0x01) != 0;
    
    match reg {
        ArgKind::A => cpu.registers.a = new_value,
        ArgKind::B => cpu.registers.b = new_value,
        ArgKind::C => cpu.registers.c = new_value,
        ArgKind::D => cpu.registers.d = new_value,
        ArgKind::E => cpu.registers.e = new_value,
        ArgKind::H => cpu.registers.h = new_value,
        ArgKind::L => cpu.registers.l = new_value,
        _ => {}
    }
    8
}

fn execute_swap(cpu: &mut Cpu, reg: &ArgKind) -> u8 {
    let value = match reg {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        _ => panic!("Invalid SWAP register"),
    };
    
    // Swap upper and lower nibbles
    let new_value = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    
    match reg {
        ArgKind::A => cpu.registers.a = new_value,
        ArgKind::B => cpu.registers.b = new_value,
        ArgKind::C => cpu.registers.c = new_value,
        ArgKind::D => cpu.registers.d = new_value,
        ArgKind::E => cpu.registers.e = new_value,
        ArgKind::H => cpu.registers.h = new_value,
        ArgKind::L => cpu.registers.l = new_value,
        _ => {}
    }
    8
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