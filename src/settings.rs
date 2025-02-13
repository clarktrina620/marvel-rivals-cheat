pub struct Settings {
    pub is_cheat_active: bool,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            is_cheat_active: false,
        }
    }

    pub fn toggle_cheat(&mut self) {
        self.is_cheat_active = !self.is_cheat_active;
    }
}