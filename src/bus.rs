use crate::{display::Display, ram::Ram};

pub struct Bus {
    ram: Ram,
    display: Display,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
            display: Display::new(),
        }
    }

    pub fn write_to_ram(&mut self, address: u16, byte_data: u8) {
        self.ram.write_byte(address, byte_data);
    }

    pub fn read_from_ram(&self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn clear_display(&mut self) {
        self.display.clear();
    }
}
