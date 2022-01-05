use crate::screen::Draw;

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing selectbox: {}", self.width * self.height);
    }
}
