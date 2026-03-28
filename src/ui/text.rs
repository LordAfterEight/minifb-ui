pub struct Text {
    pub font: crate::ttf::Font,
    pub text: String,
}

impl Text {
    pub fn new(text: &str, font: crate::ttf::Font) -> Self {
        Self {
            font,
            text: text.to_string()
        }
    }

    pub fn get_width(&self, size: f32) -> usize {
        let mut width = 0;
        for c in self.text.chars() {
            width += self.font.font.metrics(c, size).advance_width as usize;
        }
        return width;
    }
}
