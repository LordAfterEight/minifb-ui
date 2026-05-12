pub struct Dropdown {
    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub height: usize,

    items: Vec<String>,
    selected: usize,
    pub open: bool,

    pub bg_color: crate::color::Color,
    pub bg_open_color: crate::color::Color,
    pub hover_color: crate::color::Color,
    pub text_color: crate::color::Color,
    pub border_color: crate::color::Color,
    pub border_size: usize,
    pub radius: usize,
    pub blur: usize,

    pub font: Option<crate::ttf::Font>,
    pub font_size: f32,

    pub item_height: usize,
    pub max_visible: usize,

    lmb_was_down: bool,
    changed_this_frame: bool,
}

impl Default for Dropdown {
    fn default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            width: 180,
            height: 30,
            items: Vec::new(),
            selected: 0,
            open: false,
            bg_color: crate::color::Color::new(30, 30, 40),
            bg_open_color: crate::color::Color::new(25, 25, 35),
            hover_color: crate::color::Color::rgba(108, 92, 231, 40),
            text_color: crate::color::Color::new(220, 220, 230),
            border_color: crate::color::Color::new(80, 80, 100),
            border_size: 1,
            radius: 6,
            blur: 0,
            font: None,
            font_size: 13.0,
            item_height: 28,
            max_visible: 8,
            lmb_was_down: false,
            changed_this_frame: false,
        }
    }
}

impl Dropdown {
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

    pub fn items(mut self, items: &[&str]) -> Self {
        self.items = items.iter().map(|s| s.to_string()).collect();
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

    pub fn bg_open_color(mut self, color: crate::color::Color) -> Self {
        self.bg_open_color = color;
        self
    }

    pub fn hover_color(mut self, color: crate::color::Color) -> Self {
        self.hover_color = color;
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

    pub fn blur(mut self, blur: usize) -> Self {
        self.blur = blur;
        self
    }

    pub fn item_height(mut self, h: usize) -> Self {
        self.item_height = h;
        self
    }

    pub fn max_visible(mut self, n: usize) -> Self {
        self.max_visible = n;
        self
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn selected_text(&self) -> &str {
        self.items.get(self.selected).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn set_selected(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected = index;
        }
    }

    pub fn just_changed(&self) -> bool {
        self.changed_this_frame
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    fn update(&mut self, window: &mut crate::window::Window) {
        self.changed_this_frame = false;
        let mouse = window.get_mouse_state();
        let lmb = mouse.lmb_clicked;
        let lmb_just = lmb && !self.lmb_was_down;
        let mx = mouse.pos_x;
        let my = mouse.pos_y;

        if lmb_just {
            // Click on header
            let header_hit = mx >= self.pos_x as f32
                && mx < (self.pos_x + self.width) as f32
                && my >= self.pos_y as f32
                && my < (self.pos_y + self.height) as f32;

            if header_hit {
                self.open = !self.open;
            } else if self.open {
                // Check click on dropdown items
                let visible = self.items.len().min(self.max_visible);
                let list_y = self.pos_y + self.height;
                let list_h = visible * self.item_height;

                let in_list = mx >= self.pos_x as f32
                    && mx < (self.pos_x + self.width) as f32
                    && my >= list_y as f32
                    && my < (list_y + list_h) as f32;

                if in_list {
                    let idx = ((my - list_y as f32) / self.item_height as f32) as usize;
                    if idx < self.items.len() && idx != self.selected {
                        self.selected = idx;
                        self.changed_this_frame = true;
                    }
                }
                self.open = false;
            }
        }

        self.lmb_was_down = lmb;
    }

    pub fn draw(&mut self, window: &mut crate::window::Window) {
        self.update(window);

        let font = match &self.font {
            Some(f) => f.clone(),
            None => return,
        };

        // Header
        window.draw_rect_f(
            self.pos_x, self.pos_y, self.width, self.height,
            self.radius, &self.bg_color, 0,
        );
        for i in 0..self.border_size {
            window.draw_rect(
                self.pos_x + i, self.pos_y + i,
                self.width - i * 2, self.height - i * 2,
                self.radius.saturating_sub(i), &self.border_color,
            );
        }

        // Selected text
        let selected_text = self.items.get(self.selected).cloned().unwrap_or_default();
        let text = crate::ui::text::Text::new(&selected_text, font.clone());
        let text_x = self.pos_x + 8;
        let text_y = self.pos_y as f32 + (self.height as f32 - self.font_size) / 2.0;
        window.draw_text(text_x, text_y as usize, &text, self.font_size, &self.text_color);

        // Arrow indicator
        let arrow = crate::ui::text::Text::new(if self.open { "^" } else { "v" }, font.clone());
        let arr_x = self.pos_x + self.width - 20;
        window.draw_text(arr_x, text_y as usize, &arrow, self.font_size, &self.text_color);

        // Dropdown list
        if self.open {
            let mouse = window.get_mouse_state();
            let mx = mouse.pos_x;
            let my = mouse.pos_y;
            let visible = self.items.len().min(self.max_visible);
            let list_y = self.pos_y + self.height;
            let list_h = visible * self.item_height;

            window.draw_rect_f(
                self.pos_x, list_y, self.width, list_h,
                self.radius, &self.bg_open_color, self.blur,
            );
            for i in 0..self.border_size {
                window.draw_rect(
                    self.pos_x + i, list_y + i,
                    self.width - i * 2, list_h - i * 2,
                    self.radius.saturating_sub(i), &self.border_color,
                );
            }

            for (i, item) in self.items.iter().take(visible).enumerate() {
                let iy = list_y + i * self.item_height;
                let hovered = mx >= self.pos_x as f32
                    && mx < (self.pos_x + self.width) as f32
                    && my >= iy as f32
                    && my < (iy + self.item_height) as f32;

                if hovered || i == self.selected {
                    window.draw_rect_f(
                        self.pos_x + self.border_size, iy,
                        self.width - self.border_size * 2, self.item_height,
                        0, &self.hover_color, 0,
                    );
                }

                let item_text = crate::ui::text::Text::new(item, font.clone());
                let item_ty = iy as f32 + (self.item_height as f32 - self.font_size) / 2.0;
                window.draw_text(self.pos_x + 8, item_ty as usize, &item_text, self.font_size, &self.text_color);
            }
        }
    }
}
