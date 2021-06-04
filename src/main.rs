mod bus;
mod chip8;
mod cpu;
mod display;
mod fonts;
mod instruction;
mod keyboard;
mod ram;

use std::{fs::File, io::Read};

use chip8::Chip8;
use display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use graphics::rectangle::square;
use piston::{
    Button, Event, Key, PressEvent, ReleaseEvent, RenderEvent, Size, UpdateEvent, WindowSettings,
};
use piston_window::PistonWindow as Window;

const ENLARGEMENT_FACTOR: usize = 10;

fn main() {
    let input_rom = "rom/Space_Invaders.ch8";

    let mut rom = File::open(input_rom).expect("Not ableto open rom");
    let mut program = Vec::<u8>::new();
    rom.read_to_end(&mut program).expect("Not able to read rom");

    let mut chip8 = Chip8::new();
    chip8.load_rom(program);

    let window_width = DISPLAY_WIDTH * ENLARGEMENT_FACTOR;
    let window_height = DISPLAY_HEIGHT * ENLARGEMENT_FACTOR;

    let size = Size {
        width: window_width as f64,
        height: window_height as f64,
    };

    let mut window: Window = WindowSettings::new("Chip-8 emulator", size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        if let Some(button) = event.press_args() {
            chip8.key_pressed(get_emulator_keycode(button));
        }

        if let Some(button) = event.release_args() {
            chip8.key_released(get_emulator_keycode(button));
        }

        if let Some(update) = event.update_args() {
            chip8.execute(update.dt);
        }

        if let Some(_args) = event.render_args() {
            render_emulator(&chip8.display().get_buffer(), &mut window, &event)
        }
    }
}

fn render_emulator(display_buffer: &display::Buffer, window: &mut Window, event: &Event) {
    use graphics::*;

    window.draw_2d(event, |context, graphics, _device| {
        clear(color::BLACK, graphics);

        for (coord_x, row) in display_buffer.iter().enumerate() {
            for (coord_y, pixel_on) in row.iter().enumerate() {
                if *pixel_on {
                    let pixel = square(
                        (coord_x * ENLARGEMENT_FACTOR) as f64,
                        (coord_y * ENLARGEMENT_FACTOR) as f64,
                        ENLARGEMENT_FACTOR as f64,
                    );
                    Rectangle::new(color::WHITE).draw(
                        pixel,
                        &context.draw_state,
                        context.transform,
                        graphics,
                    );
                }
            }
        }
    });
}

fn get_emulator_keycode(button: Button) -> Option<u8> {
    if let Button::Keyboard(key) = button {
        match key {
            // first key row
            Key::NumPad1 => return Some(0x1),
            Key::NumPad2 => return Some(0x2),
            Key::NumPad3 => return Some(0x3),
            Key::NumPad4 => return Some(0xC),
            // second key row
            Key::Q => return Some(0x4),
            Key::W => {
                return Some(0x5);
            }
            Key::E => return Some(0x6),
            Key::R => return Some(0xD),
            // third key row
            Key::A => return Some(0x7),
            Key::S => return Some(0x8),
            Key::D => return Some(0x9),
            Key::F => return Some(0xE),
            // fourth key row
            Key::Y => return Some(0xA),
            Key::X => return Some(0x0),
            Key::C => return Some(0xB),
            Key::V => return Some(0xF),
            _ => return None,
        };
    }

    None
}
