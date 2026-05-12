pub struct Tooltip {
    pub text: crate::ui::text::Text,
    pub font_size: f32,

    pub bg_color: crate::color::Color,
    pub text_color: crate::color::Color,
    pub border_color: crate::color::Color,
    pub border_size: usize,
    pub radius: usize,
    pub padding: usize,

    /// Vertical offset from target (negative = above)
    pub offset_y: isize,
}

impl Tooltip {
    pub fn new(text: &str, font: crate::ttf::Font, size: f32) -> Self {
        Self {
            text: crate::ui::text::Text::new(text, font),
            font_size: size,
            bg_color: crate::color::Color::new(20, 20, 30),
            text_color: crate::color::Color::new(220, 220, 230),
            border_color: crate::color::Color::new(70, 70, 90),
            border_size: 1,
            radius: 6,
            padding: 6,
            offset_y: -8,
        }
    }

    pub fn bg_color(mut self, color: crate::color::Color) -> Self {
        self.bg_color = color;
        self
    }

    pub fn text_color(mut self, color: crate::color::Color) -> Self {
        self.text_color = color;
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

    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    pub fn offset_y(mut self, offset: isize) -> Self {
        self.offset_y = offset;
        self
    }

    /// Draws the tooltip if the mouse is hovering over the target area
    pub fn draw_if_hovered(
        &self,
        window: &mut crate::window::Window,
        target_x: usize,
        target_y: usize,
        target_w: usize,
        target_h: usize,
    ) {
        let mouse = window.get_mouse_state();
        let mx = mouse.pos_x;
        let my = mouse.pos_y;
        let in_target = mx >= target_x as f32
            && mx < (target_x + target_w) as f32
            && my >= target_y as f32
            && my < (target_y + target_h) as f32;

        if !in_target {
            return;
        }

        let text_w = self.text.get_width_precise(self.font_size);
        let lm = self.text.font.font.horizontal_line_metrics(self.font_size).unwrap();
        let text_h = (lm.ascent - lm.descent).ceil();

        let tip_w = text_w.ceil() as usize + self.padding * 2;
        let tip_h = text_h as usize + self.padding * 2;

        // Center horizontally on target, position above/below
        let tip_x = (target_x + target_w / 2).saturating_sub(tip_w / 2);
        let tip_y = if self.offset_y < 0 {
            target_y.saturating_sub(tip_h).saturating_sub((-self.offset_y) as usize)
        } else {
            target_y + target_h + self.offset_y as usize
        };

        // Background
        window.draw_rect_f(tip_x, tip_y, tip_w, tip_h, self.radius, &self.bg_color, 0);

        // Border
        for i in 0..self.border_size {
            window.draw_rect(
                tip_x + i, tip_y + i,
                tip_w - i * 2, tip_h - i * 2,
                self.radius.saturating_sub(i), &self.border_color,
            );
        }

        // Text
        window.draw_text(
            tip_x + self.padding,
            tip_y + self.padding,
            &self.text, self.font_size, &self.text_color,
        );
    }
}
