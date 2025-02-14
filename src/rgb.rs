use std::fs;
use std::path::Path;

pub mod cpu;
pub mod instructions;
pub mod ram;
pub mod registers;

struct rgb {}

impl rgb {
    pub fn load_game(path: &Path) -> Self {
        let rom = fs::read(path).expect("expected valid file path");
        rgb {}
    }
}
