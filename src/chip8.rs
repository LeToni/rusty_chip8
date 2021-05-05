use crate::{cpu::CPU, fonts::SPRITES, ram::Ram};

const PROGRAM_OFFSET: usize = 0x200;
pub struct Chip8 {
    cpu: CPU,
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let ram = Ram::new();
        let cpu = CPU::new();

        let mut chip8 = Chip8 { ram, cpu };
        chip8.init_memory();

        chip8
    }

    fn init_memory(&mut self) {
        let font_sprites = SPRITES.clone();

        for (i, &sprite) in font_sprites.iter().flat_map(|r| r.iter()).enumerate() {
            self.ram.memory[i] = sprite;
        }
    }

    pub fn load_rom(&mut self, program: Vec<u8>) {
        let prog = program.clone();

        for (i, &data) in prog.iter().enumerate() {
            self.ram.memory[PROGRAM_OFFSET + i] = data;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_load_font_0_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.ram.memory[0], 0xF0);
        assert_eq!(chip8.ram.memory[4], 0xF0);
    }

    #[test]
    fn should_load_font_1_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.ram.memory[5], 0x20);
        assert_eq!(chip8.ram.memory[9], 0x70);
    }

    #[test]
    fn should_load_font_a_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.ram.memory[50], 0xF0);
        assert_eq!(chip8.ram.memory[54], 0x90);
    }

    #[test]
    fn should_load_font_b_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.ram.memory[55], 0xE0);
        assert_eq!(chip8.ram.memory[59], 0xE0);
    }

    #[test]
    fn should_load_font_f_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.ram.memory[75], 0xF0);
        assert_eq!(chip8.ram.memory[79], 0x80);
    }

    #[test]
    fn should_load_programm_at_address_0x200() {
        // Given
        let mut chip8 = Chip8::new();
        let prog: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        // When
        chip8.load_rom(prog);

        // Then
        assert_eq!(chip8.ram.memory[0x199], 0);
        assert_eq!(chip8.ram.memory[0x200], 10);
        assert_eq!(chip8.ram.memory[0x209], 100);
    }
}
