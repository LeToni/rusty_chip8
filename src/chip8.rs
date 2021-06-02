use crate::{bus::Bus, cpu::CPU, display::Display, fonts::SPRITES};

const PROGRAM_OFFSET: usize = 0x200;
pub struct Chip8 {
    cpu: CPU,
    bus: Bus,
    pub display: Display,
}

impl<'a> Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            cpu: CPU::new(),
            bus: Bus::new(),
            display: Display::new(),
        };

        chip8.init_memory();

        chip8
    }

    fn init_memory(&mut self) {
        let font_sprites = SPRITES.clone();

        for (i, &sprite) in font_sprites.iter().flat_map(|r| r.iter()).enumerate() {
            self.bus.write_to_ram(i as u16, sprite)
        }
    }

    pub fn load_rom(&mut self, program: Vec<u8>) {
        let prog = program.clone();

        for (i, &data) in prog.iter().enumerate() {
            self.bus.write_to_ram((PROGRAM_OFFSET + i) as u16, data);
        }
    }

    pub fn execute(&mut self, delta_time: f64) {
        self.cpu.cycle(&mut self.bus, delta_time);
    }

    pub fn key_pressed(&mut self, key: Option<u8>) {
        self.bus.pressed_key(key)
    }

    pub fn key_released(&mut self, key: Option<u8>) {
        self.bus.released_key(key);
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
        assert_eq!(chip8.bus.read_from_ram(0), 0xF0);
        assert_eq!(chip8.bus.read_from_ram(4), 0xF0);
    }

    #[test]
    fn should_load_font_1_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.bus.read_from_ram(0x5), 0x20);
        assert_eq!(chip8.bus.read_from_ram(0x9), 0x70);
    }

    #[test]
    fn should_load_font_a_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.bus.read_from_ram(50), 0xF0);
        assert_eq!(chip8.bus.read_from_ram(54), 0x90);
    }

    #[test]
    fn should_load_font_b_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.bus.read_from_ram(55), 0xE0);
        assert_eq!(chip8.bus.read_from_ram(59), 0xE0);
    }

    #[test]
    fn should_load_font_f_correct_into_memory() {
        // Given
        let chip8 = Chip8::new();

        // When

        // Then
        assert_eq!(chip8.bus.read_from_ram(75), 0xF0);
        assert_eq!(chip8.bus.read_from_ram(79), 0x80);
    }

    #[test]
    fn should_load_programm_at_address_0x200() {
        // Given
        let mut chip8 = Chip8::new();
        let prog: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        // When
        chip8.load_rom(prog);

        // Then
        assert_eq!(chip8.bus.read_from_ram(0x199), 0);
        assert_eq!(chip8.bus.read_from_ram(0x200), 10);
        assert_eq!(chip8.bus.read_from_ram(0x209), 100);
    }
}
