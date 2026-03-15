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
}

impl Default for Text {
    fn default() -> Self {
        Self {
            font: crate::ttf::Font::new("assets/Dico.ttf"),
            text: "Default".to_string()
        }
    }
}
