mod bus;
mod chip8;
mod cpu;
mod display;
mod fonts;
mod instruction;
mod ram;

use std::{fs::File, io::Read};

use chip8::Chip8;
use piston::{
    EventSettings, Events, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent, WindowSettings,
};
use piston_window::PistonWindow as Window;

fn main() {
    let input_rom = "rom/Space_Invaders.ch8";

    let mut rom = File::open(input_rom).expect("Not ableto open rom");
    let mut program = Vec::<u8>::new();
    rom.read_to_end(&mut program).expect("Not able to read rom");

    let mut chip8 = Chip8::new();
    chip8.load_rom(program);
    chip8.execute_cycle();

    let mut window: Window = WindowSettings::new("Chip-8 emulator", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());

    while let Some(event) = window.next() {
        if let Some(key) = event.press_args() {}

        if let Some(key) = event.release_args() {}

        if let Some(update) = event.update_args() {}

        if let Some(args) = event.render_args() {}
    }
}
