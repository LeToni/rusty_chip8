const MEMORY_SIZE: usize = 0xFFF;
pub struct Ram {
    pub memory: [u8; MEMORY_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        let memory = [0; MEMORY_SIZE];

        Ram { memory: memory }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
