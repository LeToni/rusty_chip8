use crate::{cpu::CPU, ram::Ram};

pub struct Chip8 {
    cpu: CPU,
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut ram = Ram::new();
        let mut cpu = CPU::new();

        Chip8 { ram, cpu }
    }

    fn init_memory() {}

    pub fn load_rom(program: Vec<u8>) {}
}
