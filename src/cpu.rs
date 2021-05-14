use crate::{bus::Bus, instruction::Instruction};

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

    pub fn cycle(&mut self, bus: &mut Bus) {
        let opcode = self.fetch_opcode(bus);
        let instruction = self.decode_opcode(opcode);

        match instruction {
            Instruction::Clear => {
                bus.clear_display();
                self.program_counter = self.program_counter + 2;
            }
            Instruction::Return => {
                let addr = self.stack[self.stack_pointer as usize];
                self.program_counter = addr;

                if self.stack_pointer > 0 {
                    self.stack_pointer = self.stack_pointer - 1;
                }
            }
            Instruction::Jump(addr) => {
                self.program_counter = addr;
            }
            Instruction::Call(addr) => {
                if (self.stack_pointer as usize) < STACK_SIZE {
                    self.stack_pointer = self.stack_pointer + 1;
                }
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.program_counter = addr;
            }
            Instruction::SkipEqVxByte(_, _) => {}
            Instruction::SkipNEqVxByte(_, _) => {}
            Instruction::SkipEqVxVy(_, _) => {}
            Instruction::LoadVxByte(_, _) => {}
            Instruction::AddVxByte(_, _) => {}
            Instruction::LoadVxVy(_, _) => {}
            Instruction::OrVxVy(_, _) => {}
            Instruction::AndVxVy(_, _) => {}
            Instruction::Xor(_, _) => {}
            Instruction::AddVxVy(_, _) => {}
            Instruction::SubVxVy(_, _) => {}
            Instruction::RShiftVx(_) => {}
            Instruction::SubnVxVy(_, _) => {}
            Instruction::LShiftVx(_) => {}
            Instruction::SkipNEqVxVy(_, _) => {}
            Instruction::LoadI(_) => {}
            Instruction::JumpV0(_) => {}
            Instruction::RndVxByte(_, _) => {}
            Instruction::Draw(_, _, _) => {}
            Instruction::SkipVx(_) => {}
            Instruction::NSkipVx(_) => {}
            Instruction::LoadVxTimer(_) => {}
            Instruction::LoadVxKey(_) => {}
            Instruction::LoadTimerVx(_) => {}
            Instruction::LoadSoundVx(_) => {}
            Instruction::AddI(_) => {}
            Instruction::LoadFVx(_) => {}
            Instruction::LoadBVx(_) => {}
            Instruction::StoreIVx(_) => {}
            Instruction::LoadIVx(_) => {}
        }
    }

    fn fetch_opcode(&self, bus: &mut Bus) -> u16 {
        let higher_byte: u16 = (bus.read_from_ram(self.program_counter) as u16) << 8;
        let lower_byte: u16 = bus.read_from_ram(self.program_counter + 1) as u16;

        let opcode = higher_byte | lower_byte;

        opcode
    }

    fn decode_opcode(&self, opcode: u16) -> Instruction {
        Instruction::translate_opcode(opcode).expect("Unknown opcode")
    }
}
