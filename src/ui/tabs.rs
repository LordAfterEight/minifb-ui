pub struct Tabs {
    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub tab_height: usize,

    labels: Vec<String>,
    selected: usize,

    pub font: Option<crate::ttf::Font>,
    pub font_size: f32,

    pub bg_color: crate::color::Color,
    pub active_color: crate::color::Color,
    pub text_color: crate::color::Color,
    pub active_text_color: crate::color::Color,
    pub indicator_color: crate::color::Color,
    pub border_color: crate::color::Color,
    pub indicator_height: usize,
    pub radius: usize,

    lmb_was_down: bool,
    changed_this_frame: bool,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            width: 400,
            tab_height: 36,
            labels: Vec::new(),
            selected: 0,
            font: None,
            font_size: 13.0,
            bg_color: crate::color::Color::new(20, 20, 30),
            active_color: crate::color::Color::new(30, 30, 45),
            text_color: crate::color::Color::new(140, 140, 160),
            active_text_color: crate::color::Color::new(230, 230, 240),
            indicator_color: crate::color::Color::new(108, 92, 231),
            border_color: crate::color::Color::new(50, 50, 70),
            indicator_height: 3,
            radius: 0,
            lmb_was_down: false,
            changed_this_frame: false,
        }
    }
}

impl Tabs {
    pub fn position(mut self, x: usize, y: usize) -> Self {
        self.pos_x = x;
        self.pos_y = y;
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn tab_height(mut self, h: usize) -> Self {
        self.tab_height = h;
        self
    }

    pub fn tabs(mut self, labels: &[&str]) -> Self {
        self.labels = labels.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn default_selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    pub fn font(mut self, font: crate::ttf::Font, size: f32) -> Self {
        self.font = Some(font);
        self.font_size = size;
        self
    }

    pub fn bg_color(mut self, color: crate::color::Color) -> Self {
        self.bg_color = color;
        self
    }

    pub fn active_color(mut self, color: crate::color::Color) -> Self {
        self.active_color = color;
        self
    }

    pub fn text_color(mut self, color: crate::color::Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn active_text_color(mut self, color: crate::color::Color) -> Self {
        self.active_text_color = color;
        self
    }

    pub fn indicator_color(mut self, color: crate::color::Color) -> Self {
        self.indicator_color = color;
        self
    }

    pub fn border_color(mut self, color: crate::color::Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn set_selected(&mut self, index: usize) {
        if index < self.labels.len() {
            self.selected = index;
        }
    }

    pub fn just_changed(&self) -> bool {
        self.changed_this_frame
    }

    pub fn tab_count(&self) -> usize {
        self.labels.len()
    }

    /// Returns the y position where tab content should begin
    pub fn content_y(&self) -> usize {
        self.pos_y + self.tab_height
    }

    pub fn draw(&mut self, window: &mut crate::window::Window) {
        self.changed_this_frame = false;

        let font = match &self.font {
            Some(f) => f.clone(),
            None => return,
        };

        let n = self.labels.len();
        if n == 0 {
            return;
        }

        let tab_w = self.width / n;
        let mouse = window.get_mouse_state();
        let mx = mouse.pos_x;
        let my = mouse.pos_y;
        let lmb = mouse.lmb_clicked;
        let lmb_just = lmb && !self.lmb_was_down;

        // Background
        window.draw_rect_f(
            self.pos_x, self.pos_y, self.width, self.tab_height,
            self.radius, &self.bg_color, 0,
        );

        for (i, label) in self.labels.iter().enumerate() {
            let tx = self.pos_x + i * tab_w;
            let is_active = i == self.selected;

            let hovered = mx >= tx as f32 && mx < (tx + tab_w) as f32
                && my >= self.pos_y as f32 && my < (self.pos_y + self.tab_height) as f32;

            if lmb_just && hovered && !is_active {
                self.selected = i;
                self.changed_this_frame = true;
            }

            // Active tab background
            if is_active {
                window.draw_rect_f(tx, self.pos_y, tab_w, self.tab_height, 0, &self.active_color, 0);
            }

            // Label
            let text = crate::ui::text::Text::new(label, font.clone());
            let color = if is_active { &self.active_text_color } else { &self.text_color };
            window.draw_text_centered(tx, self.pos_y, tab_w, self.tab_height, &text, self.font_size, color);

            // Active indicator
            if is_active {
                let ind_y = self.pos_y + self.tab_height - self.indicator_height;
                window.draw_rect_f(tx, ind_y, tab_w, self.indicator_height, 0, &self.indicator_color, 0);
            }
        }

        // Bottom border
        window.draw_rect_f(
            self.pos_x, self.pos_y + self.tab_height - 1,
            self.width, 1, 0, &self.border_color, 0,
        );

        self.lmb_was_down = lmb;
    }
}
