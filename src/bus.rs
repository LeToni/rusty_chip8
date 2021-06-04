use crate::{display::Display, keyboard::Keyboard, ram::Ram};

pub struct Bus {
    ram: Ram,
    display: Display,
    keyboard: Keyboard,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn write_to_ram(&mut self, address: u16, byte_data: u8) {
        self.ram.write_byte(address, byte_data);
    }

    pub fn read_from_ram(&self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn get_display(&self) -> &Display {
        &self.display
    }

    pub fn clear_display(&mut self) {
        self.display.clear();
    }

    pub fn pressed_key(&mut self, key: Option<u8>) {
        self.keyboard.pressed_key(key);
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.keyboard.get_pressed_key()
    }

    pub fn released_key(&mut self, key: Option<u8>) {
        self.keyboard.released_key(key);
    }

    pub fn draw_byte(&mut self, pos_x: u8, pos_y: u8, byte: u8) -> bool {
        self.display.draw_byte(pos_x, pos_y, byte)
    }
}
