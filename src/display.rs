const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

type Buffer = [bool; WIDTH * HEIGHT];

pub struct Display {
    buffer: Buffer,
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }
}
