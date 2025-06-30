use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::{InstructionKind, ArgKind};

pub fn execute(cpu: &mut Cpu, instruction: &InstructionKind) -> u8 {
    match instruction {
        InstructionKind::AND(dest, src) => {
            execute_and(cpu, dest, src)
        }
        InstructionKind::OR(dest, src) => {
            execute_or(cpu, dest, src)
        }
        InstructionKind::XOR(dest, src) => {
            execute_xor(cpu, dest, src)
        }
        InstructionKind::CP(dest, src) => {
            execute_cp(cpu, dest, src)
        }
        InstructionKind::CP_MEM(dest, addr_reg) => {
            execute_cp_mem(cpu, dest, addr_reg)
        }
        _ => panic!("Invalid logic instruction"),
    }
}

fn execute_and(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    let src_value = match src {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        ArgKind::Immediate(value) => *value,
        _ => panic!("Unsupported AND source"),
    };
    if let ArgKind::A = dest {
        cpu.registers.a &= src_value;
        cpu.registers.f.zero = cpu.registers.a == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = false;
    }
    4
}

fn execute_or(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    let src_value = match src {
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
        ArgKind::Immediate(value) => *value,
        _ => panic!("Unsupported OR source"),
    };
    if let ArgKind::A = dest {
        cpu.registers.a |= src_value;
        cpu.registers.f.zero = cpu.registers.a == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = false;
    }
    4
}

fn execute_xor(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    let src_value = match src {
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
        ArgKind::Immediate(value) => *value,
        _ => panic!("Unsupported XOR source"),
    };
    if let ArgKind::A = dest {
        cpu.registers.a ^= src_value;
        cpu.registers.f.zero = cpu.registers.a == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = false;
    }
    4
}

fn execute_cp(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    let src_value = match src {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        ArgKind::Immediate(value) => *value,
        _ => panic!("Unsupported CP source"),
    };
    if let ArgKind::A = dest {
        let result = cpu.registers.a.wrapping_sub(src_value);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = (cpu.registers.a & 0x0F) < (src_value & 0x0F);
        cpu.registers.f.carry = cpu.registers.a < src_value;
    }
    4
}

fn execute_cp_mem(cpu: &mut Cpu, dest: &ArgKind, addr_reg: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported CP_MEM address register"),
    };
    let src_value = cpu.mmap.read(address);
    if let ArgKind::A = dest {
        let result = cpu.registers.a.wrapping_sub(src_value);
        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = (cpu.registers.a & 0x0F) < (src_value & 0x0F);
        cpu.registers.f.carry = cpu.registers.a < src_value;
    }
    8
}