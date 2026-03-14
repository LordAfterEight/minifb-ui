pub struct Text<'a> {
    pub font: &'a crate::ttf::Font,
    pub text: String,
}

impl<'b> Text<'b> {
    pub fn new<'a: 'b>(text: &str, font: &'a crate::ttf::Font) -> Self {
        Self {
            font,
            text: text.to_string()
        }
    }
}
