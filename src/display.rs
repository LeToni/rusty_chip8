const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

type Buffer = [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT];

pub struct Display {
    buffer: Buffer,
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }
}
