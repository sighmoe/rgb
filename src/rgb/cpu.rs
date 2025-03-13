use crate::rgb::instructions::Instruction;
use crate::rgb::memory::MemoryMap;
use crate::rgb::registers::Registers;

pub struct Cpu {
    registers: Registers,
    pc: u16,
    sp: u16,
    mmap: MemoryMap,
}

impl Cpu {
    pub fn new() -> Self {
        let registers = Registers::new();
        let mut mmap = MemoryMap::new();
        mmap.load_bootstrap();

        Cpu {
            registers,
            pc: 0,
            sp: 0,
            mmap,
        }
    }

    fn decode(&self) {
        let _bite = self.mmap.read(self.pc);
    }

    fn execute(&mut self, instruction: Instruction) {}

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}
