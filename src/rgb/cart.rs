use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Cart {
    rom: Vec<u8>,
}

impl Cart {
    pub fn new(path: &Path) -> Self {
        let buf = fs::read(path).expect("expected valid file path");
        Cart { rom: buf }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
