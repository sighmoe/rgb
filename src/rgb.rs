use std::fs;
use std::path::Path;

pub mod cpu;
pub mod instructions;
pub mod memory;
pub mod ppu;
pub mod registers;

struct Rgb {}

impl Rgb {
    pub fn load_game(path: &Path) -> Self {
        let rom = fs::read(path).expect("expected valid file path");
        Rgb {}
    }
}
