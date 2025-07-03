use rgb::rgb::{cpu::Cpu, registers::Registers, memory::MemoryMap};

#[test]
fn test_inc_a() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.a = 0x0F;
    cpu.mmap.write(0x0000, 0x3C); // INC A
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.a, 0x10);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.half_carry); // 0x0F + 1 causes half carry
    assert!(!cpu.registers.f.subtract);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_inc_b_zero_flag() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.b = 0xFF;
    cpu.mmap.write(0x0000, 0x04); // INC B
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.registers.b, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.subtract);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_bit_0_b_set() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.b = 0x01; // Bit 0 is set
    cpu.mmap.write(0x0000, 0xCB); // CB prefix
    cpu.mmap.write(0x0001, 0x40); // BIT 0, B
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert!(!cpu.registers.f.zero); // Bit is set, so Z=0
    assert!(!cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_bit_0_b_clear() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.b = 0xFE; // Bit 0 is clear
    cpu.mmap.write(0x0000, 0xCB); // CB prefix
    cpu.mmap.write(0x0001, 0x40); // BIT 0, B
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert!(cpu.registers.f.zero); // Bit is clear, so Z=1
    assert!(!cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    assert_eq!(cpu.pc, 2);
}

#[test]
fn test_jp_always() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.mmap.write(0x0000, 0xC3); // JP a16
    cpu.mmap.write(0x0001, 0x34); // Low byte
    cpu.mmap.write(0x0002, 0x12); // High byte
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.pc, 0x1234);
}

#[test]
fn test_jp_zero_condition_true() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.f.zero = true;
    cpu.mmap.write(0x0000, 0xCA); // JP Z, a16
    cpu.mmap.write(0x0001, 0x78); // Low byte
    cpu.mmap.write(0x0002, 0x56); // High byte
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.pc, 0x5678);
}

#[test]
fn test_jp_zero_condition_false() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.registers.f.zero = false;
    cpu.mmap.write(0x0000, 0xCA); // JP Z, a16
    cpu.mmap.write(0x0001, 0x78); // Low byte
    cpu.mmap.write(0x0002, 0x56); // High byte
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.pc, 3); // Should continue to next instruction
}

#[test]
fn test_call_and_ret() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0x100,
        sp: 0xFFFE,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    // Test CALL
    cpu.mmap.write(0x0100, 0xCD); // CALL a16
    cpu.mmap.write(0x0101, 0x00); // Low byte
    cpu.mmap.write(0x0102, 0x02); // High byte
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.pc, 0x0200); // Should jump to called address
    assert_eq!(cpu.sp, 0xFFFC); // Stack pointer should decrease by 2
    
    // Test RET
    cpu.mmap.write(0x0200, 0xC9); // RET
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert_eq!(cpu.pc, 0x103); // Should return to address after CALL
    assert_eq!(cpu.sp, 0xFFFE); // Stack pointer should be restored
}

#[test]
fn test_halt() {
    let mut cpu = Cpu {
        registers: Registers::new(),
        pc: 0,
        sp: 0,
        mmap: MemoryMap::new(),
        halted: false,
    };
    
    cpu.mmap.write(0x0000, 0x76); // HALT
    
    let instruction = cpu.decode();
    cpu.execute(instruction);
    
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 1);
}