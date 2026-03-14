pub struct Font {
    pub font: fontdue::Font,
}

impl Font {
    pub fn new(path: &str) -> Self {
        Self {
            font: fontdue::Font::from_bytes(
                std::fs::read(path).expect(&format!("Could not find font file: {}", path)),
                fontdue::FontSettings::default()
            )
            .expect("Could not create font"),
        }
    }
}
