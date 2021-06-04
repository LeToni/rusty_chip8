use rand::Rng;

use crate::{bus::Bus, instruction::Instruction};

const PROGRAM_START: u16 = 0x200;
const NUM_GENERAL_PURPOSE_REGISTERS: usize = 16;
const CLOCK_RATE: f64 = 600.0;

pub struct CPU {
    general_registers: [u8; NUM_GENERAL_PURPOSE_REGISTERS],
    sound_register: u8,
    timer_register: u8,
    program_counter: u16,
    stack: Vec<u16>,
    index_register: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            general_registers: [0; NUM_GENERAL_PURPOSE_REGISTERS],
            sound_register: 0,
            timer_register: 0,
            program_counter: PROGRAM_START,
            stack: Vec::<u16>::new(),
            index_register: 0,
        }
    }

    pub fn cycle(&mut self, bus: &mut Bus, delta_time: f64) {
        let executions = (delta_time * CLOCK_RATE) as u64;

        for _ in 1..executions {
            self.execute_instruction(bus);
        }
    }

    fn execute_instruction(&mut self, bus: &mut Bus) {
        let opcode = self.fetch_opcode(bus);
        let instruction = self.decode_opcode(opcode);

        match instruction {
            Instruction::Clear => {
                bus.clear_display();
                self.program_counter += 2;
            }
            Instruction::Return => {
                let addr = self.stack.pop().unwrap();
                self.program_counter = addr;
            }
            Instruction::Jump(addr) => {
                self.program_counter = addr;
            }
            Instruction::Call(addr) => {
                self.stack.push(self.program_counter + 2);
                self.program_counter = addr;
            }
            Instruction::SkipEqVxByte(register, data_byte) => {
                self.program_counter = if self.fetch_from_register(register) == data_byte {
                    self.program_counter + 4
                } else {
                    self.program_counter + 2
                }
            }
            Instruction::SkipNEqVxByte(register, data_byte) => {
                self.program_counter = if self.fetch_from_register(register) != data_byte {
                    self.program_counter + 4
                } else {
                    self.program_counter + 2
                }
            }
            Instruction::SkipEqVxVy(register1, register2) => {
                self.program_counter =
                    if self.fetch_from_register(register1) == self.fetch_from_register(register2) {
                        self.program_counter + 4
                    } else {
                        self.program_counter + 2
                    }
            }
            Instruction::LoadVxByte(register, data_byte) => {
                self.write_to_register(register, data_byte);

                self.program_counter += 2;
            }
            Instruction::AddVxByte(register, data_byte) => {
                let register_data = self.fetch_from_register(register);
                self.write_to_register(register, register_data.wrapping_add(data_byte));

                self.program_counter += 2;
            }
            Instruction::LoadVxVy(register1, register2) => {
                let register2_data = self.fetch_from_register(register2);
                self.write_to_register(register1, register2_data);

                self.program_counter += 2;
            }
            Instruction::OrVxVy(register1, register2) => {
                let vx = self.fetch_from_register(register1);
                let vy = self.fetch_from_register(register2);
                self.write_to_register(register1, vx | vy);

                self.program_counter += 2;
            }
            Instruction::AndVxVy(register1, register2) => {
                let vx = self.fetch_from_register(register1);
                let vy = self.fetch_from_register(register2);

                self.write_to_register(register1, vx & vy);

                self.program_counter += 2;
            }
            Instruction::Xor(register1, register2) => {
                let vx = self.fetch_from_register(register1);
                let vy = self.fetch_from_register(register2);

                self.write_to_register(register1, vx ^ vy);

                self.program_counter += 2;
            }
            Instruction::AddVxVy(register1, register2) => {
                let sum: u16 = self.fetch_from_register(register1) as u16
                    + self.fetch_from_register(register2) as u16;

                self.write_to_register(0xF, (sum > 0xFF) as u8);
                self.write_to_register(register1, (sum & 0xFF) as u8);

                self.program_counter += 2;
            }
            Instruction::SubVxVy(register1, register2) => {
                let vx = self.fetch_from_register(register1);
                let vy = self.fetch_from_register(register2);

                self.write_to_register(0xF, (vx > vy) as u8);

                let diff = vx.wrapping_sub(vy);
                self.write_to_register(register1, diff);

                self.program_counter += 2;
            }
            Instruction::RShiftVx(register) => {
                let data_byte = self.fetch_from_register(register);

                self.write_to_register(0xF, data_byte & 0x1);
                self.write_to_register(register, data_byte >> 1);

                self.program_counter += 2;
            }
            Instruction::SubnVxVy(register1, register2) => {
                let vx = self.fetch_from_register(register1);
                let vy = self.fetch_from_register(register2);

                self.write_to_register(0xF, (vx < vy) as u8);

                let diff = vy.wrapping_sub(vx);
                self.write_to_register(register1, diff);

                self.program_counter += 2;
            }
            Instruction::LShiftVx(register) => {
                let data_byte = self.fetch_from_register(register);

                self.write_to_register(0xF, data_byte >> 7);
                self.write_to_register(register, (data_byte) << 1);

                self.program_counter += 2;
            }
            Instruction::SkipNEqVxVy(register1, register2) => {
                let data_byte1 = self.fetch_from_register(register1);
                let data_byte2 = self.fetch_from_register(register2);

                self.program_counter = if data_byte1 != data_byte2 {
                    self.program_counter + 4
                } else {
                    self.program_counter + 2
                }
            }
            Instruction::LoadI(address) => {
                self.index_register = address;
                self.program_counter += 2;
            }
            Instruction::JumpV0(address) => {
                let data_byte = self.general_registers[0];
                let new_location = address + data_byte as u16;

                self.program_counter = new_location;
            }
            Instruction::RndVxByte(register, data_byte) => {
                let mut rng = rand::thread_rng();
                let random_value: u8 = rng.gen::<u8>();

                self.write_to_register(register, random_value & data_byte);

                self.program_counter += 2;
            }
            Instruction::Draw(register_x, register_y, n) => {
                let vx = self.fetch_from_register(register_x);
                let vy = self.fetch_from_register(register_y);

                self.draw_sprite(bus, vx, vy, n);
                self.program_counter += 2;
            }
            Instruction::SkipVx(register) => {
                let stored_value = self.fetch_from_register(register);

                if let Some(key) = bus.get_pressed_key() {
                    if key == stored_value {
                        self.program_counter += 2;
                    }
                }
                self.program_counter += 2;
            }
            Instruction::NSkipVx(register) => {
                let stored_value = self.fetch_from_register(register);

                match bus.get_pressed_key() {
                    Some(key) => {
                        if key != stored_value {
                            self.program_counter += 2;
                        }
                    }
                    _ => (),
                }
                self.program_counter += 2;
            }
            Instruction::LoadVxTimer(register) => {
                self.timer_register = self.fetch_from_register(register);

                self.program_counter += 2;
            }
            Instruction::LoadVxKey(register) => {
                if let Some(pressed_key) = bus.get_pressed_key() {
                    self.write_to_register(register, pressed_key);
                    self.program_counter += 2;
                }
            }
            Instruction::LoadTimerVx(register) => {
                self.write_to_register(register, self.timer_register);

                self.program_counter += 2;
            }
            Instruction::LoadSoundVx(register) => {
                self.sound_register = self.fetch_from_register(register);

                self.program_counter += 2;
            }
            Instruction::AddI(register) => {
                self.index_register += self.fetch_from_register(register) as u16;

                self.program_counter += 2;
            }
            Instruction::LoadFVx(register) => {
                let digit = self.fetch_from_register(register) as u16;

                self.index_register = digit * 5;
                self.program_counter += 2;
            }
            Instruction::LoadBVx(register) => {
                let mut data_byte = self.fetch_from_register(register);

                // ones digitat at location I+2
                bus.write_to_ram(self.index_register + 2, data_byte % 10);

                // tens digit at location I+1
                data_byte /= 10;
                bus.write_to_ram(self.index_register + 1, data_byte % 10);

                // hundred digit at location I
                data_byte /= 10;
                bus.write_to_ram(self.index_register, data_byte % 10);

                self.program_counter += 2;
            }
            Instruction::StoreIVx(register) => {
                for i in 0..=register {
                    let data_byte = self.fetch_from_register(i);
                    bus.write_to_ram(self.index_register + i as u16, data_byte);
                }

                self.program_counter += 2;
            }
            Instruction::LoadIVx(register) => {
                for i in 0..=register {
                    let data_byte = bus.read_from_ram(self.index_register + i as u16);
                    self.write_to_register(i, data_byte);
                }

                self.program_counter += 2;
            }
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

    fn fetch_from_register(&self, register_number: u8) -> u8 {
        self.general_registers[register_number as usize]
    }

    fn write_to_register(&mut self, register_number: u8, data_byte: u8) {
        self.general_registers[register_number as usize] = data_byte;
    }

    fn draw_sprite(&mut self, bus: &mut Bus, starting_x: u8, starting_y: u8, n: u8) {
        let mut pixel_collision = false;

        for sprite_y in 0..n {
            let b = bus.read_from_ram(self.index_register + sprite_y as u16);

            pixel_collision |= bus.draw_byte(starting_x, starting_y + sprite_y, b);
        }

        self.write_to_register(0xF, pixel_collision as u8);
    }
}
