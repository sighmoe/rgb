enum ArgKind {
    Immediate(u8),
    SP,
}

enum InstructionKind {
    LD(ArgKind, ArgKind),
    NOP,
}

impl InstructionKind {
    pub fn from(instr: u8) -> Self {
        match instr {
            0x00 => Self::NOP,
            b => panic!("No instruction mapping for byte {}", b),
        }
    }
}

pub struct Instruction {
    kind: InstructionKind,
    instr: u8,
    arg1: Option<ArgKind>,
    arg2: Option<ArgKind>,
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            kind: InstructionKind::NOP,
            instr: 0,
            arg1: None,
            arg2: None,
        }
    }
}

#[derive(Debug)]
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
