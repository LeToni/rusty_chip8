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
    SubVxVy(u8, u8),
    RShiftVx(u8),
    SubnVxVy(u8, u8),
    LShiftVx(u8),
    SkipNEqVxVy(u8, u8),
    LoadI(u16),
    JumpV0(u16),
    RndVxByte(u8, u8),
    Draw(u8, u8, u8),
    SkipVx(u8),
    NSkipVx(u8),
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
        match xooo(opcode) {
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
            0x8 => match n(opcode) {
                0x0 => Some(Instruction::LoadVxVy(oxoo(opcode), ooxo(opcode))),
                0x1 => Some(Instruction::OrVxVy(oxoo(opcode), ooxo(opcode))),
                0x2 => Some(Instruction::AndVxVy(oxoo(opcode), ooxo(opcode))),
                0x3 => Some(Instruction::Xor(oxoo(opcode), ooxo(opcode))),
                0x4 => Some(Instruction::AddVxVy(oxoo(opcode), ooxo(opcode))),
                0x5 => Some(Instruction::SubVxVy(oxoo(opcode), ooxo(opcode))),
                0x6 => Some(Instruction::RShiftVx(oxoo(opcode))),
                0x7 => Some(Instruction::SubnVxVy(oxoo(opcode), ooxo(opcode))),
                0xE => Some(Instruction::LShiftVx(oxoo(opcode))),
                _ => None,
            },
            0x9 => match n(opcode) {
                0x0 => Some(Instruction::SkipNEqVxVy(oxoo(opcode), ooxo(opcode))),
                _ => None,
            },
            0xA => Some(Instruction::LoadI(nnn(opcode))),
            0xB => Some(Instruction::JumpV0(nnn(opcode))),
            0xC => Some(Instruction::RndVxByte(oxoo(opcode), kk(opcode))),
            0xD => Some(Instruction::Draw(oxoo(opcode), ooxo(opcode), n(opcode))),
            0xE => match kk(opcode) {
                0x9E => Some(Instruction::SkipVx(oxoo(opcode))),
                0xA1 => Some(Instruction::NSkipVx(oxoo(opcode))),
                _ => None,
            },
            0xF => match kk(opcode) {
                0x07 => Some(Instruction::LoadVxTimer(oxoo(opcode))),
                0x0A => Some(Instruction::LoadVxKey(oxoo(opcode))),
                0x15 => Some(Instruction::LoadTimerVx(oxoo(opcode))),
                0x18 => Some(Instruction::LoadSoundVx(oxoo(opcode))),
                0x1E => Some(Instruction::AddI(oxoo(opcode))),
                0x29 => Some(Instruction::LoadFVx(oxoo(opcode))),
                0x33 => Some(Instruction::LoadBVx(oxoo(opcode))),
                0x55 => Some(Instruction::StoreIVx(oxoo(opcode))),
                0x65 => Some(Instruction::LoadIVx(oxoo(opcode))),
                _ => None,
            },
            _ => None,
        }
    }
}

fn xooo(opcode: u16) -> u8 {
    let shifted_value = opcode >> 12;
    (shifted_value & 0xF) as u8
}

fn oxoo(opcode: u16) -> u8 {
    let shifted_value = opcode >> 8;
    (shifted_value & 0xF) as u8
}

fn ooxo(opcode: u16) -> u8 {
    let shifted_value = opcode >> 4;
    (shifted_value & 0xF) as u8
}

fn n(opcode: u16) -> u8 {
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
