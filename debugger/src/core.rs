#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebuggerState {
    Running,
    Paused,
    Stepping,
}

#[derive(Debug, Clone)]
pub struct CpuSnapshot {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub zero_flag: bool,
    pub subtract_flag: bool,
    pub half_carry_flag: bool,
    pub carry_flag: bool,
    pub ime: bool,
    pub halted: bool,
}

#[derive(Debug, Clone)]
pub struct MemoryInspection {
    pub address: u16,
    pub value: u8,
    pub timestamp: std::time::Instant,
}

pub struct Debugger {
    pub state: DebuggerState,
    pub step_count: u64,
    pub instructions_to_run: Option<u64>,
    pub current_snapshot: Option<CpuSnapshot>,
    pub memory_watches: Vec<u16>,
    pub memory_values: Vec<MemoryInspection>,
    pub breakpoints: Vec<u16>,
    pub instruction_history: Vec<(u16, u8)>, // (PC, opcode)
    pub history_size: usize,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            state: DebuggerState::Paused,
            step_count: 0,
            instructions_to_run: None,
            current_snapshot: None,
            memory_watches: Vec::new(),
            memory_values: Vec::new(),
            breakpoints: Vec::new(),
            instruction_history: Vec::new(),
            history_size: 50,
        }
    }
    
    pub fn step_one(&mut self) {
        self.state = DebuggerState::Stepping;
        self.instructions_to_run = Some(1);
        self.step_count += 1;
    }
    
    pub fn step_multiple(&mut self, count: u64) {
        self.state = DebuggerState::Stepping;
        self.instructions_to_run = Some(count);
    }
    
    pub fn resume(&mut self) {
        self.state = DebuggerState::Running;
        self.instructions_to_run = None;
    }
    
    pub fn pause(&mut self) {
        self.state = DebuggerState::Paused;
        self.instructions_to_run = None;
    }
    
    pub fn should_execute(&mut self) -> bool {
        match self.state {
            DebuggerState::Running => true,
            DebuggerState::Paused => false,
            DebuggerState::Stepping => {
                if let Some(count) = self.instructions_to_run {
                    if count > 0 {
                        self.instructions_to_run = Some(count - 1);
                        true
                    } else {
                        self.state = DebuggerState::Paused;
                        self.instructions_to_run = None;
                        false
                    }
                } else {
                    self.state = DebuggerState::Paused;
                    false
                }
            }
        }
    }
    
    pub fn add_breakpoint(&mut self, address: u16) {
        if !self.breakpoints.contains(&address) {
            self.breakpoints.push(address);
        }
    }
    
    pub fn remove_breakpoint(&mut self, address: u16) {
        self.breakpoints.retain(|&bp| bp != address);
    }
    
    pub fn check_breakpoint(&self, pc: u16) -> bool {
        self.breakpoints.contains(&pc)
    }
    
    pub fn add_memory_watch(&mut self, address: u16) {
        if !self.memory_watches.contains(&address) {
            self.memory_watches.push(address);
        }
    }
    
    pub fn remove_memory_watch(&mut self, address: u16) {
        self.memory_watches.retain(|&addr| addr != address);
    }
    
    pub fn update_memory_watches<F>(&mut self, read_memory: F) 
    where F: Fn(u16) -> u8 
    {
        for &address in &self.memory_watches {
            let value = read_memory(address);
            self.memory_values.push(MemoryInspection {
                address,
                value,
                timestamp: std::time::Instant::now(),
            });
        }
        
        // Keep only recent values to prevent unbounded growth
        if self.memory_values.len() > 1000 {
            self.memory_values.drain(0..500);
        }
    }
    
    pub fn get_memory_value<F>(&self, address: u16, read_memory: F) -> u8
    where F: Fn(u16) -> u8
    {
        read_memory(address)
    }
    
    pub fn record_instruction(&mut self, pc: u16, opcode: u8) {
        self.instruction_history.push((pc, opcode));
        if self.instruction_history.len() > self.history_size {
            self.instruction_history.remove(0);
        }
    }
    
    pub fn get_instruction_history(&self) -> &[(u16, u8)] {
        &self.instruction_history
    }
}