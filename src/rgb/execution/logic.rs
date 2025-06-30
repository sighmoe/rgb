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
        InstructionKind::CPL => {
            execute_cpl(cpu)
        }
        InstructionKind::SCF => {
            execute_scf(cpu)
        }
        InstructionKind::CCF => {
            execute_ccf(cpu)
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
        ArgKind::HL => {
            let addr = cpu.registers.get_hl();
            cpu.mmap.read(addr)
        }
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
    match src {
        ArgKind::HL => 8, // (HL) operations take 8 cycles
        ArgKind::Immediate(_) => 8, // Immediate operations take 8 cycles
        _ => 4, // Register operations take 4 cycles
    }
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

fn execute_cpl(cpu: &mut Cpu) -> u8 {
    // CPL - Complement A register (flip all bits)
    cpu.registers.a = !cpu.registers.a; // Bitwise NOT
    
    // Set flags - CPL has specific flag behavior
    // Zero flag is not affected
    cpu.registers.f.subtract = true; // Always set
    cpu.registers.f.half_carry = true; // Always set
    // Carry flag is not affected
    
    4 // Takes 4 cycles
}

fn execute_scf(cpu: &mut Cpu) -> u8 {
    // SCF - Set Carry Flag
    // Zero flag is not affected
    cpu.registers.f.subtract = false; // Always cleared
    cpu.registers.f.half_carry = false; // Always cleared
    cpu.registers.f.carry = true; // Always set
    
    4 // Takes 4 cycles
}

fn execute_ccf(cpu: &mut Cpu) -> u8 {
    // CCF - Complement Carry Flag
    // Zero flag is not affected
    cpu.registers.f.subtract = false; // Always cleared
    cpu.registers.f.half_carry = false; // Always cleared
    cpu.registers.f.carry = !cpu.registers.f.carry; // Complement carry flag
    
    4 // Takes 4 cycles
}