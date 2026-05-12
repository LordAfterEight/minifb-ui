pub struct ContextMenu {
    items: Vec<String>,
    pub open: bool,
    menu_x: usize,
    menu_y: usize,

    pub item_height: usize,
    pub width: usize,
    pub padding: usize,

    pub bg_color: crate::color::Color,
    pub hover_color: crate::color::Color,
    pub text_color: crate::color::Color,
    pub border_color: crate::color::Color,
    pub border_size: usize,
    pub radius: usize,

    pub font: Option<crate::ttf::Font>,
    pub font_size: f32,

    clicked_item: Option<usize>,
    lmb_was_down: bool,
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            open: false,
            menu_x: 0,
            menu_y: 0,
            item_height: 28,
            width: 160,
            padding: 4,
            bg_color: crate::color::Color::new(25, 25, 35),
            hover_color: crate::color::Color::rgba(108, 92, 231, 40),
            text_color: crate::color::Color::new(220, 220, 230),
            border_color: crate::color::Color::new(70, 70, 90),
            border_size: 1,
            radius: 6,
            font: None,
            font_size: 13.0,
            clicked_item: None,
            lmb_was_down: false,
        }
    }
}

impl ContextMenu {
    pub fn items(mut self, items: &[&str]) -> Self {
        self.items = items.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn width(mut self, w: usize) -> Self {
        self.width = w;
        self
    }

    pub fn item_height(mut self, h: usize) -> Self {
        self.item_height = h;
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

    /// Opens the context menu at the given position
    pub fn open(&mut self, x: usize, y: usize) {
        self.menu_x = x;
        self.menu_y = y;
        self.open = true;
        self.clicked_item = None;
    }

    /// Closes the context menu
    pub fn close(&mut self) {
        self.open = false;
        self.clicked_item = None;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the index of the item that was clicked this frame, if any
    pub fn clicked_item(&self) -> Option<usize> {
        self.clicked_item
    }

    pub fn draw(&mut self, window: &mut crate::window::Window) {
        self.clicked_item = None;

        if !self.open {
            self.lmb_was_down = window.get_mouse_state().lmb_clicked;
            return;
        }

        let font = match &self.font {
            Some(f) => f.clone(),
            None => return,
        };

        let mouse = window.get_mouse_state();
        let mx = mouse.pos_x;
        let my = mouse.pos_y;
        let lmb = mouse.lmb_clicked;
        let lmb_just = lmb && !self.lmb_was_down;

        let total_h = self.padding * 2 + self.items.len() * self.item_height;

        // Background
        window.draw_rect_f(
            self.menu_x, self.menu_y, self.width, total_h,
            self.radius, &self.bg_color, 0,
        );
        for i in 0..self.border_size {
            window.draw_rect(
                self.menu_x + i, self.menu_y + i,
                self.width - i * 2, total_h - i * 2,
                self.radius.saturating_sub(i), &self.border_color,
            );
        }

        // Items
        for (i, item) in self.items.iter().enumerate() {
            let iy = self.menu_y + self.padding + i * self.item_height;
            let hovered = mx >= self.menu_x as f32
                && mx < (self.menu_x + self.width) as f32
                && my >= iy as f32
                && my < (iy + self.item_height) as f32;

            if hovered {
                window.draw_rect_f(
                    self.menu_x + self.border_size, iy,
                    self.width - self.border_size * 2, self.item_height,
                    0, &self.hover_color, 0,
                );

                if lmb_just {
                    self.clicked_item = Some(i);
                    self.open = false;
                }
            }

            let text = crate::ui::text::Text::new(item, font.clone());
            let ty = iy as f32 + (self.item_height as f32 - self.font_size) / 2.0;
            window.draw_text(self.menu_x + 12, ty as usize, &text, self.font_size, &self.text_color);
        }

        // Click outside closes
        if lmb_just {
            let in_menu = mx >= self.menu_x as f32
                && mx < (self.menu_x + self.width) as f32
                && my >= self.menu_y as f32
                && my < (self.menu_y + total_h) as f32;
            if !in_menu {
                self.open = false;
            }
        }

        self.lmb_was_down = lmb;
    }
}
