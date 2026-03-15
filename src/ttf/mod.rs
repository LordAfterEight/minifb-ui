#[derive(Clone)]
pub struct Font {
    pub font: fontdue::Font,
}

impl Font {
    pub fn new(path: &str) -> Self {
        Self {
            font: fontdue::Font::from_bytes(
                std::fs::read(path).expect(&format!("Could not find font file: {}", path)),
                fontdue::FontSettings {
                    scale: 16.0,
                    load_substitutions: true,
                    ..Default::default()
                }
            )
            .expect("Could not create font"),
        }
    }
    pub fn as_slice(&self) -> [&fontdue::Font; 1] {
        [&self.font]
    }
}
