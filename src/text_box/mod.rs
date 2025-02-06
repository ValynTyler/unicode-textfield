pub struct TextBox {
    pub size: (u16, u16),
    pub contents: String,
}

impl TextBox {
    pub fn width(&self) -> u16 {
        self.size.0
    }

    pub fn height(&self) -> u16 {
        self.size.1
    }
}
