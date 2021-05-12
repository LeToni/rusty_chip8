const PROGRAM_START: u16 = 0x200;
const NUM_GENERAL_PURPOSE_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;

pub struct CPU {
    general_registers: [u8; NUM_GENERAL_PURPOSE_REGISTERS],
    sound_register: u8,
    timer_register: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; STACK_SIZE],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            general_registers: [0; NUM_GENERAL_PURPOSE_REGISTERS],
            sound_register: 0,
            timer_register: 0,
            program_counter: PROGRAM_START,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn set_pc(&mut self, address: u16) {
        self.program_counter = address;
    }
}
