pub struct Timers {
    pub div: u16
}

impl Timers {
    pub fn update() {}
}

impl Default for Timers {
    fn default() -> Self {
        Self { 
            div: 0x0
        }
    }
}