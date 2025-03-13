use std::fs;

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

    pub fn load_bootstrap(&mut self) {
        let buf = fs::read("~/Development/rgb/testr/dmg_boot.bin")
            .expect("expected valid file path for bootstrap rom");
        assert_eq!(
            buf.len(),
            256,
            "expected bootstrap rom to be exactly 256 bytes"
        );

        self.contents[0..(255 + 1)].copy_from_slice(&buf);
    }
}
