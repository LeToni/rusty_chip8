const MEMORY_SIZE: usize = 0xFFF;
const MEMORY_OFFSET: usize = 0x200;

pub struct Memory {
    pub ram: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new(program: Vec<u8>) -> Memory {
        let mut ram = [0; MEMORY_SIZE];

        for (i, data) in program.iter().enumerate() {
            ram[MEMORY_OFFSET + i] = *data;
        }

        Memory { ram }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_should_start_at_offset() {
        let test_program: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let memory = Memory::new(test_program);

        assert_eq!(memory.ram[MEMORY_OFFSET], 1);
        assert_eq!(memory.ram[MEMORY_OFFSET + 9], 10);
    }
}
