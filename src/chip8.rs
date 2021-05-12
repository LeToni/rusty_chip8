use crate::{cpu::CPU, fonts::SPRITES, instruction::Instruction, ram::Ram};

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

    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch_opcode();
        let instruction = self.decode_opcode(opcode);
        self.execute_opcode(instruction);
    }

    fn fetch_opcode(&self) -> u16 {
        let higher_byte: u16 = (self.ram.read_byte(self.cpu.get_pc()) as u16) << 8;
        let lower_byte: u16 = (self.ram.read_byte(self.cpu.get_pc() + 1)) as u16;

        let opcode = higher_byte | lower_byte;
        opcode
    }

    fn decode_opcode(&self, opcode: u16) -> Instruction {
        Instruction::translate_opcode(opcode).expect("Unknown opcode")
    }

    fn execute_opcode(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Clear => (),
            Instruction::Return => (),
            Instruction::Jump(address) => (),
            Instruction::Call(address) => (),
            Instruction::SkipEqVxByte(register, byte_value) => (),
            Instruction::SkipNEqVxByte(register, byte_value) => (),
            _ => (),
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
