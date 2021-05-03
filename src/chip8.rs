use crate::fonts::SPRITES;

use crate::{cpu::CPU, ram::Ram};

pub struct Chip8 {
    cpu: CPU,
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let ram = Ram::new();
        let cpu = CPU::new();

        Chip8 { ram, cpu }
    }

    fn init_memory(&mut self) {
        let font_sprites = SPRITES.clone();

        for (i, &sprite) in font_sprites.iter().flat_map(|r| r.iter()).enumerate() {
            self.ram.memory[i] = sprite;
        }
    }

    pub fn load_rom(program: Vec<u8>) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_load_font_0_correct_into_memory() {
        // Given
        let mut chip8 = Chip8::new();

        // When
        chip8.init_memory();

        // Then
        assert_eq!(chip8.ram.memory[0], 0xF0);
        assert_eq!(chip8.ram.memory[4], 0xF0);
    }

    #[test]
    fn should_load_font_1_correct_into_memory() {
        // Given
        let mut chip8 = Chip8::new();

        // When
        chip8.init_memory();

        // Then
        assert_eq!(chip8.ram.memory[5], 0x20);
        assert_eq!(chip8.ram.memory[9], 0x70);
    }

    #[test]
    fn should_load_font_a_correct_into_memory() {
        // Given
        let mut chip8 = Chip8::new();

        // When
        chip8.init_memory();

        // Then
        assert_eq!(chip8.ram.memory[50], 0xF0);
        assert_eq!(chip8.ram.memory[54], 0x90);
    }

    #[test]
    fn should_load_font_b_correct_into_memory() {
        // Given
        let mut chip8 = Chip8::new();

        // When
        chip8.init_memory();

        // Then
        assert_eq!(chip8.ram.memory[55], 0xE0);
        assert_eq!(chip8.ram.memory[59], 0xE0);
    }

    #[test]
    fn should_load_font_f_correct_into_memory() {
        // Given
        let mut chip8 = Chip8::new();

        // When
        chip8.init_memory();

        // Then
        assert_eq!(chip8.ram.memory[75], 0xF0);
        assert_eq!(chip8.ram.memory[79], 0x80);
    }
}
