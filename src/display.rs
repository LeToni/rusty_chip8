pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub type Buffer = [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

pub struct Display {
    buffer: Buffer,
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn get_buffer(&self) -> Buffer {
        self.buffer
    }

    pub fn set_pixels(&mut self, screen_pos_x: u8, screen_pos_y: u8, memory: &[u8]) -> bool {
        let mut pixel_turned_off = false;

        for (byte_number, block) in memory.iter().enumerate() {
            let y = (screen_pos_y as usize + byte_number) % DISPLAY_HEIGHT;

            for bit_number in 0..8 {
                let x = (screen_pos_x as usize + bit_number) % DISPLAY_WIDTH;
                let current_pixel = self.buffer[y][x] as u8;

                let current_bit = (block >> (7 - bit_number)) & 1;
                let new_pixel = current_bit ^ current_pixel;

                self.buffer[y][x] = if new_pixel != 0 { true } else { false };

                if current_pixel == 1 && new_pixel == 0 {
                    pixel_turned_off = true;
                }
            }
        }
        pixel_turned_off
    }
}
