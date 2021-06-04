pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub type Buffer = [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH];

pub struct Display {
    buffer: Buffer,
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];
    }

    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn draw_byte(&mut self, pos_x: u8, pos_y: u8, byte: u8) -> bool {
        let mut pixel_collision = false;
        let y = pos_y as usize % DISPLAY_HEIGHT;

        for bit_pos in 0..8 {
            let x = (pos_x as usize + bit_pos) % DISPLAY_WIDTH;

            let current_pixel = self.buffer[x][y] as u8;

            let current_bit = (byte >> (7 - bit_pos)) & 1;
            let new_pixel = current_bit ^ current_pixel;

            self.buffer[x][y] = new_pixel != 0;

            if current_pixel == 1 && new_pixel == 0 {
                pixel_collision = true;
            }

            // if self.buffer[x][y] {
            //     println!("{:?}", self.buffer[x][y])
            // }
        }

        pixel_collision
    }
}
