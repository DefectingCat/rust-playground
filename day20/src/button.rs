use crate::screen::Draw;

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing button: {}", self.label);
    }
}
