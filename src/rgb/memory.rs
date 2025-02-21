pub struct MemoryMap {
    contents: [u8; 65535],
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            contents: [0; 65535],
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.contents[addr as usize] = val;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.contents[addr as usize]
    }
}
