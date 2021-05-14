mod bus;
mod chip8;
mod cpu;
mod display;
mod fonts;
mod instruction;
mod ram;

use std::{fs::File, io::Read};

use chip8::Chip8;
fn main() {
    let input_rom = "rom/Space_Invaders.ch8";

    let mut rom = File::open(input_rom).expect("Not ableto open rom");
    let mut program = Vec::<u8>::new();
    rom.read_to_end(&mut program).expect("Not able to read rom");

    let mut chip8 = Chip8::new();
    chip8.load_rom(program);
    chip8.execute_cycle();
}
