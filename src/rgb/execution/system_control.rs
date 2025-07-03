use crate::rgb::cpu::Cpu;
use crate::rgb::instructions::InstructionKind;

pub fn execute(cpu: &mut Cpu, instruction: &InstructionKind) -> u8 {
    match instruction {
        InstructionKind::NOP => {
            execute_nop()
        }
        InstructionKind::HALT => {
            execute_halt(cpu)
        }
        InstructionKind::STOP => {
            execute_stop(cpu)
        }
        InstructionKind::EI => {
            execute_ei(cpu)
        }
        InstructionKind::DI => {
            execute_di(cpu)
        }
        _ => panic!("Invalid system control instruction"),
    }
}

fn execute_nop() -> u8 {
    // Do nothing
    4
}

fn execute_halt(cpu: &mut Cpu) -> u8 {
    // Game Boy HALT bug: if IME is false but interrupts are pending,
    // don't halt and cause next instruction to execute twice
    if !cpu.ime && cpu.check_pending_interrupts() {
        #[cfg(debug_assertions)]
        {
            static mut HALT_BUG_COUNT: u32 = 0;
            unsafe {
                HALT_BUG_COUNT += 1;
                if HALT_BUG_COUNT <= 5 {
                    println!("HALT bug triggered #{} at PC=0x{:04X}, setting halt_bug flag", HALT_BUG_COUNT, cpu.pc - 1);
                }
            }
        }
        cpu.halt_bug = true;
    } else {
        cpu.halted = true;
        #[cfg(debug_assertions)]
        {
            static mut HALT_COUNT: u32 = 0;
            unsafe {
                HALT_COUNT += 1;
                if HALT_COUNT <= 5 {
                    println!("CPU halted #{} at PC=0x{:04X}, IME={}", HALT_COUNT, cpu.pc - 1, cpu.ime);
                }
            }
        }
    }
    4
}

fn execute_stop(cpu: &mut Cpu) -> u8 {
    // STOP instruction - similar to HALT but stops CPU and LCD
    cpu.halted = true;
    4
}

fn execute_ei(cpu: &mut Cpu) -> u8 {
    // Enable interrupts after next instruction (1-instruction delay)
    #[cfg(debug_assertions)]
    {
        use log::debug;
        static mut EI_COUNT: u32 = 0;
        unsafe {
            EI_COUNT += 1;
            if EI_COUNT <= 10 {
                debug!("EI instruction executed at PC=0x{:04X}", cpu.pc - 1);
            }
        }
    }
    cpu.ei_delay = true;
    4
}

fn execute_di(cpu: &mut Cpu) -> u8 {
    // Disable interrupts immediately
    #[cfg(debug_assertions)]
    {
        use log::debug;
        static mut DI_COUNT: u32 = 0;
        unsafe {
            DI_COUNT += 1;
            if DI_COUNT <= 10 {
                debug!("DI instruction executed at PC=0x{:04X}", cpu.pc - 1);
            }
        }
    }
    cpu.ime = false;
    cpu.ei_delay = false; // Cancel any pending EI
    4
}