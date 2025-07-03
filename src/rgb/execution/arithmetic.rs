use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::{InstructionKind, ArgKind};

pub fn execute(cpu: &mut Cpu, instruction: &InstructionKind) -> u8 {
    match instruction {
        InstructionKind::ADD(dest, src) => {
            execute_add(cpu, dest, src)
        }
        InstructionKind::SUB(dest, src) => {
            execute_sub(cpu, dest, src)
        }
        InstructionKind::ADC(dest, src) => {
            execute_adc(cpu, dest, src)
        }
        InstructionKind::SBC(dest, src) => {
            execute_sbc(cpu, dest, src)
        }
        InstructionKind::INC(register) => {
            execute_inc(cpu, register)
        }
        InstructionKind::INC_MEM(addr_reg) => {
            execute_inc_mem(cpu, addr_reg)
        }
        InstructionKind::DEC(register) => {
            execute_dec(cpu, register)
        }
        InstructionKind::DEC_MEM(addr_reg) => {
            execute_dec_mem(cpu, addr_reg)
        }
        InstructionKind::INC16(register) => {
            execute_inc16(cpu, register)
        }
        InstructionKind::DEC16(register) => {
            execute_dec16(cpu, register)
        }
        InstructionKind::DAA => {
            execute_daa(cpu)
        }
        InstructionKind::ADD_SP_R8(offset) => {
            execute_add_sp_r8(cpu, *offset)
        }
        _ => panic!("Invalid arithmetic instruction"),
    }
}

fn execute_add(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    match (dest, src) {
        (ArgKind::A, ArgKind::A) => {
            cpu.registers.a = cpu.add(cpu.registers.a);
        }
        (ArgKind::A, ArgKind::B) => {
            cpu.registers.a = cpu.add(cpu.registers.b);
        }
        (ArgKind::A, ArgKind::C) => {
            cpu.registers.a = cpu.add(cpu.registers.c);
        }
        (ArgKind::A, ArgKind::D) => {
            cpu.registers.a = cpu.add(cpu.registers.d);
        }
        (ArgKind::A, ArgKind::E) => {
            cpu.registers.a = cpu.add(cpu.registers.e);
        }
        (ArgKind::A, ArgKind::H) => {
            cpu.registers.a = cpu.add(cpu.registers.h);
        }
        (ArgKind::A, ArgKind::L) => {
            cpu.registers.a = cpu.add(cpu.registers.l);
        }
        (ArgKind::A, ArgKind::HL) => {
            let addr = cpu.registers.get_hl();
            let value = cpu.mmap.read(addr);
            cpu.registers.a = cpu.add(value);
        }
        (ArgKind::A, ArgKind::Immediate(value)) => {
            cpu.registers.a = cpu.add(*value);
        }
        // 16-bit ADD instructions (ADD HL,reg16)
        (ArgKind::HL, ArgKind::BC) => {
            let hl_value = cpu.registers.get_hl();
            let bc_value = cpu.registers.get_bc();
            let result = cpu.add16(hl_value, bc_value);
            cpu.registers.set_hl(result);
        }
        (ArgKind::HL, ArgKind::DE) => {
            let hl_value = cpu.registers.get_hl();
            let de_value = cpu.registers.get_de();
            let result = cpu.add16(hl_value, de_value);
            cpu.registers.set_hl(result);
        }
        (ArgKind::HL, ArgKind::HL) => {
            let hl_value = cpu.registers.get_hl();
            let result = cpu.add16(hl_value, hl_value);
            cpu.registers.set_hl(result);
        }
        (ArgKind::HL, ArgKind::SP) => {
            let hl_value = cpu.registers.get_hl();
            let result = cpu.add16(hl_value, cpu.sp);
            cpu.registers.set_hl(result);
        }
        _ => panic!("Unsupported ADD instruction variant"),
    }
    4 // Default cycles
}

fn execute_sub(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    match (dest, src) {
        (ArgKind::A, ArgKind::A) => {
            cpu.registers.a = cpu.subtract(cpu.registers.a);
        }
        (ArgKind::A, ArgKind::B) => {
            cpu.registers.a = cpu.subtract(cpu.registers.b);
        }
        (ArgKind::A, ArgKind::C) => {
            cpu.registers.a = cpu.subtract(cpu.registers.c);
        }
        (ArgKind::A, ArgKind::D) => {
            cpu.registers.a = cpu.subtract(cpu.registers.d);
        }
        (ArgKind::A, ArgKind::E) => {
            cpu.registers.a = cpu.subtract(cpu.registers.e);
        }
        (ArgKind::A, ArgKind::H) => {
            cpu.registers.a = cpu.subtract(cpu.registers.h);
        }
        (ArgKind::A, ArgKind::L) => {
            cpu.registers.a = cpu.subtract(cpu.registers.l);
        }
        (ArgKind::A, ArgKind::HL) => {
            let addr = cpu.registers.get_hl();
            let value = cpu.mmap.read(addr);
            cpu.registers.a = cpu.subtract(value);
        }
        (ArgKind::A, ArgKind::Immediate(value)) => {
            cpu.registers.a = cpu.subtract(*value);
        }
        _ => panic!("Unsupported SUB instruction variant"),
    }
    4
}

fn execute_adc(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    let carry = if cpu.registers.f.carry { 1 } else { 0 };
    let (a_val, src_val) = match (dest, src) {
        (ArgKind::A, ArgKind::B) => (cpu.registers.a, cpu.registers.b),
        (ArgKind::A, ArgKind::C) => (cpu.registers.a, cpu.registers.c),
        (ArgKind::A, ArgKind::D) => (cpu.registers.a, cpu.registers.d),
        (ArgKind::A, ArgKind::E) => (cpu.registers.a, cpu.registers.e),
        (ArgKind::A, ArgKind::H) => (cpu.registers.a, cpu.registers.h),
        (ArgKind::A, ArgKind::L) => (cpu.registers.a, cpu.registers.l),
        (ArgKind::A, ArgKind::A) => (cpu.registers.a, cpu.registers.a),
        (ArgKind::A, ArgKind::HL) => (cpu.registers.a, cpu.mmap.read(cpu.registers.get_hl())),
        (ArgKind::A, ArgKind::Immediate(val)) => (cpu.registers.a, *val),
        _ => panic!("Invalid ADC operands"),
    };
    
    let result = a_val.wrapping_add(src_val).wrapping_add(carry);
    let half_carry = (a_val & 0x0F) + (src_val & 0x0F) + carry > 0x0F;
    let full_carry = (a_val as u16) + (src_val as u16) + (carry as u16) > 0xFF;
    
    cpu.registers.a = result;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = full_carry;
    4
}

fn execute_sbc(cpu: &mut Cpu, dest: &ArgKind, src: &ArgKind) -> u8 {
    match (dest, src) {
        (ArgKind::A, ArgKind::A) => {
            cpu.registers.a = cpu.subtract_with_carry(cpu.registers.a);
        }
        (ArgKind::A, ArgKind::B) => {
            cpu.registers.a = cpu.subtract_with_carry(cpu.registers.b);
        }
        (ArgKind::A, ArgKind::C) => {
            cpu.registers.a = cpu.subtract_with_carry(cpu.registers.c);
        }
        (ArgKind::A, ArgKind::D) => {
            cpu.registers.a = cpu.subtract_with_carry(cpu.registers.d);
        }
        (ArgKind::A, ArgKind::E) => {
            cpu.registers.a = cpu.subtract_with_carry(cpu.registers.e);
        }
        (ArgKind::A, ArgKind::H) => {
            cpu.registers.a = cpu.subtract_with_carry(cpu.registers.h);
        }
        (ArgKind::A, ArgKind::L) => {
            cpu.registers.a = cpu.subtract_with_carry(cpu.registers.l);
        }
        (ArgKind::A, ArgKind::HL) => {
            let addr = cpu.registers.get_hl();
            let value = cpu.mmap.read(addr);
            cpu.registers.a = cpu.subtract_with_carry(value);
        }
        (ArgKind::A, ArgKind::Immediate(value)) => {
            cpu.registers.a = cpu.subtract_with_carry(*value);
        }
        _ => panic!("Unsupported SBC instruction variant"),
    }
    4
}

fn execute_inc(cpu: &mut Cpu, register: &ArgKind) -> u8 {
    match register {
        ArgKind::A => {
            cpu.registers.a = cpu.increment(cpu.registers.a);
        }
        ArgKind::B => {
            cpu.registers.b = cpu.increment(cpu.registers.b);
        }
        ArgKind::C => {
            cpu.registers.c = cpu.increment(cpu.registers.c);
        }
        ArgKind::D => {
            cpu.registers.d = cpu.increment(cpu.registers.d);
        }
        ArgKind::E => {
            cpu.registers.e = cpu.increment(cpu.registers.e);
        }
        ArgKind::H => {
            cpu.registers.h = cpu.increment(cpu.registers.h);
        }
        ArgKind::L => {
            cpu.registers.l = cpu.increment(cpu.registers.l);
        }
        _ => panic!("Unsupported INC instruction variant"),
    }
    4
}

fn execute_inc_mem(cpu: &mut Cpu, addr_reg: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported INC_MEM address register"),
    };
    let value = cpu.mmap.read(address);
    let new_value = cpu.increment(value);
    cpu.mmap.write(address, new_value);
    12
}

fn execute_dec(cpu: &mut Cpu, register: &ArgKind) -> u8 {
    match register {
        ArgKind::A => {
            cpu.registers.a = cpu.decrement(cpu.registers.a);
        }
        ArgKind::B => {
            cpu.registers.b = cpu.decrement(cpu.registers.b);
        }
        ArgKind::C => {
            cpu.registers.c = cpu.decrement(cpu.registers.c);
        }
        ArgKind::D => {
            cpu.registers.d = cpu.decrement(cpu.registers.d);
        }
        ArgKind::E => {
            cpu.registers.e = cpu.decrement(cpu.registers.e);
        }
        ArgKind::H => {
            cpu.registers.h = cpu.decrement(cpu.registers.h);
        }
        ArgKind::L => {
            cpu.registers.l = cpu.decrement(cpu.registers.l);
        }
        _ => panic!("Unsupported DEC instruction variant"),
    }
    4
}

fn execute_dec_mem(cpu: &mut Cpu, addr_reg: &ArgKind) -> u8 {
    let address = match addr_reg {
        ArgKind::HL => cpu.registers.get_hl(),
        _ => panic!("Unsupported DEC_MEM address register"),
    };
    let value = cpu.mmap.read(address);
    let new_value = cpu.decrement(value);
    cpu.mmap.write(address, new_value);
    12
}

fn execute_inc16(cpu: &mut Cpu, register: &ArgKind) -> u8 {
    match register {
        ArgKind::BC => {
            let value = cpu.registers.get_bc().wrapping_add(1);
            cpu.registers.set_bc(value);
        }
        ArgKind::DE => {
            let value = cpu.registers.get_de().wrapping_add(1);
            cpu.registers.set_de(value);
        }
        ArgKind::HL => {
            let value = cpu.registers.get_hl().wrapping_add(1);
            cpu.registers.set_hl(value);
        }
        ArgKind::SP => {
            cpu.sp = cpu.sp.wrapping_add(1);
        }
        _ => panic!("Unsupported INC16 instruction variant"),
    }
    // 16-bit increment doesn't affect flags
    8
}

fn execute_dec16(cpu: &mut Cpu, register: &ArgKind) -> u8 {
    match register {
        ArgKind::BC => {
            let value = cpu.registers.get_bc().wrapping_sub(1);
            cpu.registers.set_bc(value);
        }
        ArgKind::DE => {
            let value = cpu.registers.get_de().wrapping_sub(1);
            cpu.registers.set_de(value);
        }
        ArgKind::HL => {
            let value = cpu.registers.get_hl().wrapping_sub(1);
            cpu.registers.set_hl(value);
        }
        ArgKind::SP => {
            cpu.sp = cpu.sp.wrapping_sub(1);
        }
        _ => panic!("Unsupported DEC16 instruction variant"),
    }
    // 16-bit decrement doesn't affect flags
    8
}

fn execute_daa(cpu: &mut Cpu) -> u8 {
    // Decimal Adjust Accumulator - adjusts A for BCD arithmetic
    let mut a = cpu.registers.a;
    let mut correction = 0;
    let mut carry = false;
    
    if cpu.registers.f.half_carry || (!cpu.registers.f.subtract && (a & 0x0F) > 0x09) {
        correction |= 0x06;
    }
    
    if cpu.registers.f.carry || (!cpu.registers.f.subtract && a > 0x99) {
        correction |= 0x60;
        carry = true;
    }
    
    if cpu.registers.f.subtract {
        a = a.wrapping_sub(correction);
    } else {
        a = a.wrapping_add(correction);
    }
    
    cpu.registers.a = a;
    cpu.registers.f.zero = a == 0;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;
    // Subtract flag is preserved
    4
}

fn execute_add_sp_r8(cpu: &mut Cpu, offset: i8) -> u8 {
    // ADD SP,r8 - Add signed 8-bit immediate to Stack Pointer
    let sp_value = cpu.sp as i32;
    let offset_value = offset as i32;
    let result = sp_value.wrapping_add(offset_value) as u16;
    
    // Calculate flags based on the low 8 bits only (Game Boy specific behavior)
    let sp_low = (cpu.sp & 0xFF) as u8;
    let offset_u8 = offset as u8;
    
    // Set flags - ADD SP,r8 has specific flag behavior
    cpu.registers.f.zero = false; // Always cleared
    cpu.registers.f.subtract = false; // Always cleared
    // Half carry is set if there's a carry from bit 3 to bit 4 in the low byte
    cpu.registers.f.half_carry = (sp_low & 0x0F) + (offset_u8 & 0x0F) > 0x0F;
    // Carry is set if there's a carry from bit 7 to bit 8 in the low byte
    cpu.registers.f.carry = (sp_low as u16) + (offset_u8 as u16) > 0xFF;
    
    cpu.sp = result;
    16 // Takes 16 cycles
}