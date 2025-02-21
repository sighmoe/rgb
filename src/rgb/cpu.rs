use crate::rgb::registers::Registers;
use crate::rgb::{instructions, memory, registers};

pub struct Cpu {
    rgstrs: Registers,
}
