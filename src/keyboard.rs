pub struct Keyboard {
    key: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { key: None }
    }

    pub fn pressed_key(&mut self, key_pressed: Option<u8>) {
        self.key = key_pressed;
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.key
    }

    pub fn released_key(&mut self, key_released: Option<u8>) {
        if self.key == key_released {
            self.key = None;
        }
    }
}
