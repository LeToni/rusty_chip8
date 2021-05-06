#[derive(Debug)]
pub enum Instruction {
    JMP(u16),
}

pub fn translate_opcode(opcode: u16) -> Option<Instruction> {
    match xoo(opcode) {
        0x1 => Some(Instruction::JMP(nnn(opcode))),
        _ => None,
    }
}

fn xoo(opcode: u16) -> u8 {
    let shifted_value = opcode >> 12;
    (shifted_value) as u8
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
        let test_instruction = translate_opcode(test_code);

        // Then
        match test_instruction {
            Some(instruction) => match instruction {
                Instruction::JMP(0xFFF) => Ok(()),
                Instruction::JMP(_) => Err(String::from("Failed to jump to correct address")),
            },
            None => Err(String::from("Failed to find JMP instruction")),
        }
    }
}
