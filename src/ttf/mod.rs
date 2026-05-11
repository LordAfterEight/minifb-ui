use std::sync::Arc;

#[derive(Clone)]
pub struct Font {
    pub font: Arc<fontdue::Font>,
}

impl Font {
    pub fn new(path: &str) -> Result<Self, String> {
        let bytes = std::fs::read(path)
            .map_err(|_| format!("Could not find font file: {}", path))?;
        let font = fontdue::Font::from_bytes(
            bytes,
            fontdue::FontSettings {
                scale: 16.0,
                load_substitutions: true,
                ..Default::default()
            },
        )
        .map_err(|e| format!("Could not create font: {}", e))?;
        Ok(Self { font: Arc::new(font) })
    }
    pub fn as_slice(&self) -> [&fontdue::Font; 1] {
        [&self.font]
    }
}
