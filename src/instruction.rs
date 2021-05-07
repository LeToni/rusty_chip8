#[derive(Debug)]
pub enum Instruction {
    Clear,
    Return,
    Jump(u16),
    Call(u16),
    SkipEqVxByte(u8, u8),
    SkipNEqVxByte(u8, u8),
    SkipEqVxVy(u8, u8),
    LoadVxByte(u8, u8),
    AddVxByte(u8, u8),
    LoadVxVy(u8, u8),
    OrVxVy(u8, u8),
    AndVxVy(u8, u8),
    Xor(u8, u8),
    AddVxVy(u8, u8),
    RShiftVx(u8),
    SubnVxVy(u8, u8),
    LShiftVx(u8),
    SkipNEqVxVy(u8, u8),
    LoadI(u8),
    JumpV0(u8),
    RndVxByte(u8, u8),
    Draw(u8, u8, u8),
    SkipVx(u8),
    LoadVxTimer(u8),
    LoadVxKey(u8),
    LoadTimerVx(u8),
    LoadSoundVx(u8),
    AddI(u8),
    LoadFVx(u8),
    LoadBVx(u8),
    StoreIVx(u8),
    LoadIVx(u8),
}

#[allow(dead_code)]
impl Instruction {
    pub fn translate_opcode(opcode: u16) -> Option<Instruction> {
        match oxoo(opcode) {
            0x0 => match kk(opcode) {
                0xE0 => Some(Instruction::Clear),
                0xEE => Some(Instruction::Return),
                _ => None,
            },
            0x1 => Some(Instruction::Jump(nnn(opcode))),
            0x2 => Some(Instruction::Call(nnn(opcode))),
            0x3 => Some(Instruction::SkipEqVxByte(oxoo(opcode), kk(opcode))),
            0x4 => Some(Instruction::SkipNEqVxByte(oxoo(opcode), kk(opcode))),
            0x5 => Some(Instruction::SkipEqVxVy(oxoo(opcode), ooxo(opcode))),
            0x6 => Some(Instruction::LoadVxByte(oxoo(opcode), kk(opcode))),
            0x7 => Some(Instruction::AddVxByte(oxoo(opcode), kk(opcode))),
            _ => None,
        }
    }
}

fn oxoo(opcode: u16) -> u8 {
    let shifted_value = opcode >> 12;
    (shifted_value) as u8
}

fn ooxo(opcode: u16) -> u8 {
    let shifted_value = opcode >> 4;
    (shifted_value & 0xF) as u8
}

fn ooox(opcode: u16) -> u8 {
    (opcode & 0xF) as u8
}

fn kk(opcode: u16) -> u8 {
    (opcode & 0xFF) as u8
}

fn nnn(opcode: u16) -> u16 {
    opcode & 0xFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_jmp_instruction_with_address() -> Result<(), String> {
        // Given
        let test_code: u16 = 0x1FFF;

        // When
        let test_instruction = Instruction::translate_opcode(test_code);

        // Then
        match test_instruction {
            Some(instruction) => match instruction {
                Instruction::Jump(0xFFF) => Ok(()),
                _ => Err(String::from("Failed to jump to correct address")),
            },
            None => Err(String::from("Failed to find JMP instruction")),
        }
    }
}
