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
        self.get_width_precise(size).ceil() as usize
    }

    /// Returns the precise rendered width using the layout engine
    pub fn get_width_precise(&self, size: f32) -> f32 {
        use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};

        if self.text.is_empty() {
            return 0.0;
        }

        let fonts = self.font.as_slice();
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: 0.0,
            y: 0.0,
            ..Default::default()
        });
        layout.append(&fonts, &TextStyle::new(&self.text, size, 0));

        let glyphs = layout.glyphs();
        if let Some(last) = glyphs.last() {
            // Use the last glyph's bitmap width for visual extent,
            // not advance_width which includes trailing space
            let (metrics, _) = self.font.font.rasterize_config(last.key);
            last.x + metrics.width as f32
        } else {
            0.0
        }
    }
}
