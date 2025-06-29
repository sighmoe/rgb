use rgb::rgb::{cpu::Cpu, registers::Registers, memory::MemoryMap};

#[test]
fn test_ld_bc_d16() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    // Write the LD BC, d16 instruction to memory
    // Opcode 0x01, followed by low byte 0x34, high byte 0x12
    cpu.mmap.write(0x0000, 0x01);
    cpu.mmap.write(0x0001, 0x34);
    cpu.mmap.write(0x0002, 0x12);
    
    // Decode and execute the instruction
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    // Verify BC register contains 0x1234
    assert_eq!(cpu.registers.b, 0x12);
    assert_eq!(cpu.registers.c, 0x34);
    
    // Verify PC was incremented by 3
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_ld_de_d16() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.mmap.write(0x0000, 0x11);
    cpu.mmap.write(0x0001, 0xAB);
    cpu.mmap.write(0x0002, 0xCD);
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.d, 0xCD);
    assert_eq!(cpu.registers.e, 0xAB);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_ld_hl_d16() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.mmap.write(0x0000, 0x21);
    cpu.mmap.write(0x0001, 0x56);
    cpu.mmap.write(0x0002, 0x78);
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.h, 0x78);
    assert_eq!(cpu.registers.l, 0x56);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_ld_sp_d16() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.mmap.write(0x0000, 0x31);
    cpu.mmap.write(0x0001, 0xFF);
    cpu.mmap.write(0x0002, 0xFE);
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.sp, 0xFEFF);
    assert_eq!(cpu.pc, 3);
}

#[test]
fn test_ld_b_d8() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.mmap.write(0x0000, 0x06);
    cpu.mmap.write(0x0001, 0x42);
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.b, 0x42);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_ld_a_d8() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.mmap.write(0x0000, 0x3E);
    cpu.mmap.write(0x0001, 0xFF);
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.a, 0xFF);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_ld_b_c() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.c = 0x35;
    cpu.mmap.write(0x0000, 0x41);
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.b, 0x35);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_ld_a_h() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.h = 0x99;
    cpu.mmap.write(0x0000, 0x7C);
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.a, 0x99);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_ld_a_a() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.a = 0x77;
    cpu.mmap.write(0x0000, 0x7F);
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.a, 0x77);
    assert_eq!(cpu.pc, 1);
}