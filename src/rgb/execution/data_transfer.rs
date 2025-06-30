use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::{InstructionKind, ArgKind};

pub fn execute(cpu: &mut Cpu, instruction: &InstructionKind) -> u8 {
    match instruction {
        InstructionKind::LD(dest, src) => {
            execute_ld(cpu, dest, src)
        }
        InstructionKind::LDHL_SP_R8(offset) => {
            execute_ldhl_sp_r8(cpu, *offset)
        }
        InstructionKind::LD_SP_TO_MEM(address) => {
            execute_ld_sp_to_mem(cpu, *address)
        }
        InstructionKind::LD_MEM(addr_reg, src) => {
            execute_ld_mem(cpu, addr_reg, src)
        }
        InstructionKind::LD_MEM_DEC(addr_reg, src) => {
            execute_ld_mem_dec(cpu, addr_reg, src)
        }
        InstructionKind::LD_MEM_INC(addr_reg, src) => {
            execute_ld_mem_inc(cpu, addr_reg, src)
        }
        InstructionKind::LD_FROM_MEM(dest, addr_reg) => {
            execute_ld_from_mem(cpu, dest, addr_reg)
        }
        InstructionKind::LD_FROM_MEM_DEC(dest, addr_reg) => {
            execute_ld_from_mem_dec(cpu, dest, addr_reg)
        }
        InstructionKind::LD_FROM_MEM_INC(dest, addr_reg) => {
            execute_ld_from_mem_inc(cpu, dest, addr_reg)
        }
        InstructionKind::LD_MEM_16(addr, src) => {
            execute_ld_mem_16(cpu, addr, src)
        }
        InstructionKind::LD_FROM_MEM_16(dest, addr) => {
            execute_ld_from_mem_16(cpu, dest, addr)
        }
        InstructionKind::LDH_TO_C(_) => {
            execute_ldh_to_c(cpu)
        }
        InstructionKind::LDH_FROM_C(_) => {
            execute_ldh_from_c(cpu)
        }
        InstructionKind::LDH_TO_N(_, offset) => {
            execute_ldh_to_n(cpu, *offset)
        }
        InstructionKind::LDH_FROM_N(_, offset) => {
            execute_ldh_from_n(cpu, *offset)
        }
        _ => panic!("Invalid data transfer instruction"),
    }
}

fn execute_ld(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    match (dest, src) {
        // 16-bit loads
        (ArgKind::BC, ArgKind::Immediate16(value)) => {
            cpu.registers.set_bc(*value);
        }
        (ArgKind::DE, ArgKind::Immediate16(value)) => {
            cpu.registers.set_de(*value);
        }
        (ArgKind::HL, ArgKind::Immediate16(value)) => {
            cpu.registers.set_hl(*value);
        }
        (ArgKind::SP, ArgKind::Immediate16(value)) => {
            cpu.sp = *value;
        }
        // 8-bit immediate loads
        (ArgKind::A, ArgKind::Immediate(value)) => {
            cpu.registers.a = *value;
        }
        (ArgKind::B, ArgKind::Immediate(value)) => {
            cpu.registers.b = *value;
        }
        (ArgKind::C, ArgKind::Immediate(value)) => {
            cpu.registers.c = *value;
        }
        (ArgKind::D, ArgKind::Immediate(value)) => {
            cpu.registers.d = *value;
        }
        (ArgKind::E, ArgKind::Immediate(value)) => {
            cpu.registers.e = *value;
        }
        (ArgKind::H, ArgKind::Immediate(value)) => {
            cpu.registers.h = *value;
        }
        (ArgKind::L, ArgKind::Immediate(value)) => {
            cpu.registers.l = *value;
        }
        // Register-to-register loads
        (ArgKind::A, ArgKind::A) => {
            cpu.registers.a = cpu.registers.a;
        }
        (ArgKind::A, ArgKind::B) => {
            cpu.registers.a = cpu.registers.b;
        }
        (ArgKind::A, ArgKind::C) => {
            cpu.registers.a = cpu.registers.c;
        }
        (ArgKind::A, ArgKind::D) => {
            cpu.registers.a = cpu.registers.d;
        }
        (ArgKind::A, ArgKind::E) => {
            cpu.registers.a = cpu.registers.e;
        }
        (ArgKind::A, ArgKind::H) => {
            cpu.registers.a = cpu.registers.h;
        }
        (ArgKind::A, ArgKind::L) => {
            cpu.registers.a = cpu.registers.l;
        }
        (ArgKind::B, ArgKind::A) => {
            cpu.registers.b = cpu.registers.a;
        }
        (ArgKind::B, ArgKind::B) => {
            cpu.registers.b = cpu.registers.b;
        }
        (ArgKind::B, ArgKind::C) => {
            cpu.registers.b = cpu.registers.c;
        }
        (ArgKind::B, ArgKind::D) => {
            cpu.registers.b = cpu.registers.d;
        }
        (ArgKind::B, ArgKind::E) => {
            cpu.registers.b = cpu.registers.e;
        }
        (ArgKind::B, ArgKind::H) => {
            cpu.registers.b = cpu.registers.h;
        }
        (ArgKind::B, ArgKind::L) => {
            cpu.registers.b = cpu.registers.l;
        }
        (ArgKind::C, ArgKind::A) => {
            cpu.registers.c = cpu.registers.a;
        }
        (ArgKind::C, ArgKind::B) => {
            cpu.registers.c = cpu.registers.b;
        }
        (ArgKind::C, ArgKind::C) => {
            cpu.registers.c = cpu.registers.c;
        }
        (ArgKind::C, ArgKind::D) => {
            cpu.registers.c = cpu.registers.d;
        }
        (ArgKind::C, ArgKind::E) => {
            cpu.registers.c = cpu.registers.e;
        }
        (ArgKind::C, ArgKind::H) => {
            cpu.registers.c = cpu.registers.h;
        }
        (ArgKind::C, ArgKind::L) => {
            cpu.registers.c = cpu.registers.l;
        }
        (ArgKind::D, ArgKind::A) => {
            cpu.registers.d = cpu.registers.a;
        }
        (ArgKind::D, ArgKind::B) => {
            cpu.registers.d = cpu.registers.b;
        }
        (ArgKind::D, ArgKind::C) => {
            cpu.registers.d = cpu.registers.c;
        }
        (ArgKind::D, ArgKind::D) => {
            cpu.registers.d = cpu.registers.d;
        }
        (ArgKind::D, ArgKind::E) => {
            cpu.registers.d = cpu.registers.e;
        }
        (ArgKind::D, ArgKind::H) => {
            cpu.registers.d = cpu.registers.h;
        }
        (ArgKind::D, ArgKind::L) => {
            cpu.registers.d = cpu.registers.l;
        }
        (ArgKind::E, ArgKind::A) => {
            cpu.registers.e = cpu.registers.a;
        }
        (ArgKind::E, ArgKind::B) => {
            cpu.registers.e = cpu.registers.b;
        }
        (ArgKind::E, ArgKind::C) => {
            cpu.registers.e = cpu.registers.c;
        }
        (ArgKind::E, ArgKind::D) => {
            cpu.registers.e = cpu.registers.d;
        }
        (ArgKind::E, ArgKind::E) => {
            cpu.registers.e = cpu.registers.e;
        }
        (ArgKind::E, ArgKind::H) => {
            cpu.registers.e = cpu.registers.h;
        }
        (ArgKind::E, ArgKind::L) => {
            cpu.registers.e = cpu.registers.l;
        }
        (ArgKind::H, ArgKind::A) => {
            cpu.registers.h = cpu.registers.a;
        }
        (ArgKind::H, ArgKind::B) => {
            cpu.registers.h = cpu.registers.b;
        }
        (ArgKind::H, ArgKind::C) => {
            cpu.registers.h = cpu.registers.c;
        }
        (ArgKind::H, ArgKind::D) => {
            cpu.registers.h = cpu.registers.d;
        }
        (ArgKind::H, ArgKind::E) => {
            cpu.registers.h = cpu.registers.e;
        }
        (ArgKind::H, ArgKind::H) => {
            cpu.registers.h = cpu.registers.h;
        }
        (ArgKind::H, ArgKind::L) => {
            cpu.registers.h = cpu.registers.l;
        }
        (ArgKind::L, ArgKind::A) => {
            cpu.registers.l = cpu.registers.a;
        }
        (ArgKind::L, ArgKind::B) => {
            cpu.registers.l = cpu.registers.b;
        }
        (ArgKind::L, ArgKind::C) => {
            cpu.registers.l = cpu.registers.c;
        }
        (ArgKind::L, ArgKind::D) => {
            cpu.registers.l = cpu.registers.d;
        }
        (ArgKind::L, ArgKind::E) => {
            cpu.registers.l = cpu.registers.e;
        }
        (ArgKind::L, ArgKind::H) => {
            cpu.registers.l = cpu.registers.h;
        }
        (ArgKind::L, ArgKind::L) => {
            cpu.registers.l = cpu.registers.l;
        }
        // Special case: LD SP,HL
        (ArgKind::SP, ArgKind::HL) => {
            cpu.sp = cpu.registers.get_hl();
        }
        _ => panic!("Unsupported LD instruction variant"),
    }
    4 // Default cycles, will be overridden by instruction timing
}

fn execute_ldhl_sp_r8(cpu: &mut Cpu, offset: i8) -> u8 {
    // LD HL,SP+r8 - Load HL with SP plus signed 8-bit offset
    let sp_value = cpu.sp as i32;
    let offset_value = offset as i32;
    let result = sp_value.wrapping_add(offset_value) as u16;
    
    // Set flags based on 8-bit arithmetic (lower byte only)
    let sp_low = (cpu.sp & 0xFF) as u8;
    let offset_u8 = offset as u8;
    
    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    // Half carry is set if there's a carry from bit 3 to bit 4
    cpu.registers.f.half_carry = (sp_low & 0x0F) + (offset_u8 & 0x0F) > 0x0F;
    // Carry is set if there's a carry from bit 7 to bit 8
    cpu.registers.f.carry = (sp_low as u16) + (offset_u8 as u16) > 0xFF;
    
    cpu.registers.set_hl(result);
    12
}

fn execute_ld_sp_to_mem(cpu: &mut Cpu, address: u16) -> u8 {
    // LD (nn),SP - Store SP at 16-bit address (little-endian)
    let sp_low = (cpu.sp & 0xFF) as u8;
    let sp_high = ((cpu.sp >> 8) & 0xFF) as u8;
    
    // Store SP in little-endian format (low byte first, then high byte)
    cpu.mmap.write(address, sp_low);
    cpu.mmap.write(address.wrapping_add(1), sp_high);
    20
}

fn execute_ld_mem(cpu: &mut Cpu, addr_reg: &ArgKind, src: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::BC => cpu.registers.get_bc(),
        ArgKind::DE => cpu.registers.get_de(),
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported LD_MEM address register"),
    };
    let value = match src {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        ArgKind::Immediate(val) => *val,
        _ => panic!("Unsupported LD_MEM source"),
    };
    cpu.mmap.write(address, value);
    8
}

fn execute_ld_mem_dec(cpu: &mut Cpu, addr_reg: &ArgKind, src: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported LD_MEM_DEC address register"),
    };
    let value = match src {
        ArgKind::A => cpu.registers.a,
        _ => panic!("Unsupported LD_MEM_DEC source"),
    };
    cpu.mmap.write(address, value);
    let new_hl = address.wrapping_sub(1);
    cpu.registers.set_hl(new_hl);
    8
}

fn execute_ld_mem_inc(cpu: &mut Cpu, addr_reg: &ArgKind, src: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported LD_MEM_INC address register"),
    };
    let value = match src {
        ArgKind::A => cpu.registers.a,
        _ => panic!("Unsupported LD_MEM_INC source"),
    };
    cpu.mmap.write(address, value);
    let new_hl = address.wrapping_add(1);
    cpu.registers.set_hl(new_hl);
    8
}

fn execute_ld_from_mem(cpu: &mut Cpu, dest: &ArgKind, addr_reg: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::BC => cpu.registers.get_bc(),
        ArgKind::DE => cpu.registers.get_de(),
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported LD_FROM_MEM address register"),
    };
    let value = cpu.mmap.read(address);
    match dest {
        ArgKind::A => cpu.registers.a = value,
        ArgKind::B => cpu.registers.b = value,
        ArgKind::C => cpu.registers.c = value,
        ArgKind::D => cpu.registers.d = value,
        ArgKind::E => cpu.registers.e = value,
        ArgKind::H => cpu.registers.h = value,
        ArgKind::L => cpu.registers.l = value,
        _ => panic!("Unsupported LD_FROM_MEM destination"),
    }
    8
}

fn execute_ld_from_mem_dec(cpu: &mut Cpu, dest: &ArgKind, addr_reg: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported LD_FROM_MEM_DEC address register"),
    };
    let value = cpu.mmap.read(address);
    if let ArgKind::A = dest {
        cpu.registers.a = value;
    }
    let new_hl = address.wrapping_sub(1);
    cpu.registers.set_hl(new_hl);
    8
}

fn execute_ld_from_mem_inc(cpu: &mut Cpu, dest: &ArgKind, addr_reg: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported LD_FROM_MEM_INC address register"),
    };
    let value = cpu.mmap.read(address);
    if let ArgKind::A = dest {
        cpu.registers.a = value;
    }
    let new_hl = address.wrapping_add(1);
    cpu.registers.set_hl(new_hl);
    8
}

fn execute_ld_mem_16(cpu: &mut Cpu, addr: &ArgKind, src: &ArgKind) -> u8 {
    let address = match addr {
        ArgKind::Immediate16(addr) => *addr,
        _ => panic!("Unsupported LD_MEM_16 address type"),
    };
    let value = match src {
        ArgKind::A => cpu.registers.a,
        ArgKind::B => cpu.registers.b,
        ArgKind::C => cpu.registers.c,
        ArgKind::D => cpu.registers.d,
        ArgKind::E => cpu.registers.e,
        ArgKind::H => cpu.registers.h,
        ArgKind::L => cpu.registers.l,
        _ => panic!("Unsupported LD_MEM_16 source"),
    };
    cpu.mmap.write(address, value);
    16
}

fn execute_ld_from_mem_16(cpu: &mut Cpu, dest: &ArgKind, addr: &ArgKind) -> u8 {
    let address = match addr {
        ArgKind::Immediate16(addr) => *addr,
        _ => panic!("Unsupported LD_FROM_MEM_16 address type"),
    };
    let value = cpu.mmap.read(address);
    match dest {
        ArgKind::A => cpu.registers.a = value,
        ArgKind::B => cpu.registers.b = value,
        ArgKind::C => cpu.registers.c = value,
        ArgKind::D => cpu.registers.d = value,
        ArgKind::E => cpu.registers.e = value,
        ArgKind::H => cpu.registers.h = value,
        ArgKind::L => cpu.registers.l = value,
        _ => panic!("Unsupported LD_FROM_MEM_16 destination"),
    };
    16
}

fn execute_ldh_to_c(cpu: &mut Cpu) -> u8 {
    let address = 0xFF00 + (cpu.registers.c as u16);
    cpu.mmap.write(address, cpu.registers.a);
    8
}

fn execute_ldh_from_c(cpu: &mut Cpu) -> u8 {
    let address = 0xFF00 + (cpu.registers.c as u16);
    cpu.registers.a = cpu.mmap.read(address);
    8
}

fn execute_ldh_to_n(cpu: &mut Cpu, offset: u8) -> u8 {
    let address = 0xFF00 + (offset as u16);
    cpu.mmap.write(address, cpu.registers.a);
    12
}

fn execute_ldh_from_n(cpu: &mut Cpu, offset: u8) -> u8 {
    let address = 0xFF00 + (offset as u16);
    cpu.registers.a = cpu.mmap.read(address);
    12
}