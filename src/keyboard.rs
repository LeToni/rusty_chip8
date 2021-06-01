pub struct Keyboard {
    key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { key_pressed: None }
    }

    pub fn pressed_key(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.key_pressed
    }

    pub fn released_key(&mut self, key: Option<u8>) {
        if self.key_pressed == key {
            self.key_pressed = None
        }
    }
}
