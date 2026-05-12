pub struct ProgressBar {
    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub height: usize,

    progress: f32,

    pub track_color: crate::color::Color,
    pub fill_color: crate::color::Color,
    pub border_color: crate::color::Color,
    pub border_size: usize,
    pub radius: usize,

    pub label: Option<crate::ui::text::Text>,
    pub label_size: f32,
    pub label_color: crate::color::Color,
    pub show_percentage: bool,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            width: 200,
            height: 16,
            progress: 0.0,
            track_color: crate::color::Color::new(40, 40, 50),
            fill_color: crate::color::Color::new(108, 92, 231),
            border_color: crate::color::Color::new(80, 80, 100),
            border_size: 1,
            radius: 4,
            label: None,
            label_size: 12.0,
            label_color: crate::color::Color::new(200, 200, 210),
            show_percentage: false,
        }
    }
}

impl ProgressBar {
    pub fn position(mut self, x: usize, y: usize) -> Self {
        self.pos_x = x;
        self.pos_y = y;
        self
    }

    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn default_progress(mut self, p: f32) -> Self {
        self.progress = p.clamp(0.0, 1.0);
        self
    }

    pub fn track_color(mut self, color: crate::color::Color) -> Self {
        self.track_color = color;
        self
    }

    pub fn fill_color(mut self, color: crate::color::Color) -> Self {
        self.fill_color = color;
        self
    }

    pub fn border_color(mut self, color: crate::color::Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn border(mut self, size: usize) -> Self {
        self.border_size = size;
        self
    }

    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub fn label(mut self, text: &str, font: crate::ttf::Font, size: f32) -> Self {
        self.label = Some(crate::ui::text::Text::new(text, font));
        self.label_size = size;
        self
    }

    pub fn label_color(mut self, color: crate::color::Color) -> Self {
        self.label_color = color;
        self
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn set_progress(&mut self, p: f32) {
        self.progress = p.clamp(0.0, 1.0);
    }

    pub fn draw(&mut self, window: &mut crate::window::Window) {
        // Border
        window.draw_rect_f(
            self.pos_x, self.pos_y, self.width, self.height,
            self.radius, &self.border_color, 0,
        );

        // Track
        let inner_x = self.pos_x + self.border_size;
        let inner_y = self.pos_y + self.border_size;
        let inner_w = self.width.saturating_sub(self.border_size * 2);
        let inner_h = self.height.saturating_sub(self.border_size * 2);
        let inner_r = self.radius.saturating_sub(self.border_size);
        window.draw_rect_f(inner_x, inner_y, inner_w, inner_h, inner_r, &self.track_color, 0);

        // Fill
        let fill_w = (inner_w as f32 * self.progress) as usize;
        if fill_w > 0 {
            window.draw_rect_f(inner_x, inner_y, fill_w, inner_h, inner_r, &self.fill_color, 0);
        }

        // Percentage text
        if self.show_percentage {
            if let Some(ref label) = self.label {
                let pct = format!("{}%", (self.progress * 100.0) as u32);
                let pct_text = crate::ui::text::Text::new(&pct, label.font.clone());
                window.draw_text_centered(
                    self.pos_x, self.pos_y, self.width, self.height,
                    &pct_text, self.label_size, &self.label_color,
                );
            }
        }
    }
}
