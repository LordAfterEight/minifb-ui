pub struct Checkbox {
    pub pos_x: usize,
    pub pos_y: usize,
    pub size: usize,

    pub checked: bool,
    anim_t: f32,
    pub anim_speed: f32,

    pub box_color: crate::color::Color,
    pub box_color_checked: crate::color::Color,
    pub check_color: crate::color::Color,
    pub border_color: crate::color::Color,
    pub border_size: usize,
    pub radius: usize,

    pub label: Option<crate::ui::text::Text>,
    pub label_size: f32,
    pub label_color: crate::color::Color,
    pub label_gap: usize,

    lmb_was_down: bool,
    toggled_this_frame: bool,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            size: 20,
            checked: false,
            anim_t: 0.0,
            anim_speed: 0.15,
            box_color: crate::color::Color::new(40, 40, 50),
            box_color_checked: crate::color::Color::new(108, 92, 231),
            check_color: crate::color::Color::new(255, 255, 255),
            border_color: crate::color::Color::new(100, 100, 120),
            border_size: 1,
            radius: 4,
            label: None,
            label_size: 14.0,
            label_color: crate::color::Color::new(200, 200, 210),
            label_gap: 8,
            lmb_was_down: false,
            toggled_this_frame: false,
        }
    }
}

impl Checkbox {
    pub fn position(mut self, x: usize, y: usize) -> Self {
        self.pos_x = x;
        self.pos_y = y;
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn default_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self.anim_t = if checked { 1.0 } else { 0.0 };
        self
    }

    pub fn box_color(mut self, color: crate::color::Color) -> Self {
        self.box_color = color;
        self
    }

    pub fn box_color_checked(mut self, color: crate::color::Color) -> Self {
        self.box_color_checked = color;
        self
    }

    pub fn check_color(mut self, color: crate::color::Color) -> Self {
        self.check_color = color;
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

    pub fn label(mut self, text: &str, font: crate::ttf::Font, size: f32) -> Self {
        self.label = Some(crate::ui::text::Text::new(text, font));
        self.label_size = size;
        self
    }

    pub fn label_color(mut self, color: crate::color::Color) -> Self {
        self.label_color = color;
        self
    }

    pub fn label_gap(mut self, gap: usize) -> Self {
        self.label_gap = gap;
        self
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    pub fn just_toggled(&self) -> bool {
        self.toggled_this_frame
    }

    fn update(&mut self, window: &mut crate::window::Window) {
        self.toggled_this_frame = false;

        let mouse = window.get_mouse_state();
        let lmb = mouse.lmb_clicked;
        let lmb_just = lmb && !self.lmb_was_down;

        if lmb_just {
            let mx = mouse.pos_x;
            let my = mouse.pos_y;
            let hit_w = self.size + self.label.as_ref().map_or(0, |l| {
                self.label_gap + l.get_width(self.label_size)
            });
            let in_bounds = mx >= self.pos_x as f32
                && mx < (self.pos_x + hit_w) as f32
                && my >= self.pos_y as f32
                && my < (self.pos_y + self.size) as f32;
            if in_bounds {
                self.checked = !self.checked;
                self.toggled_this_frame = true;
            }
        }

        self.lmb_was_down = lmb;

        let target = if self.checked { 1.0 } else { 0.0 };
        let diff = target - self.anim_t;
        if diff.abs() > 0.001 {
            self.anim_t += diff.signum() * self.anim_speed;
            self.anim_t = self.anim_t.clamp(0.0, 1.0);
        } else {
            self.anim_t = target;
        }
    }

    pub fn draw(&mut self, window: &mut crate::window::Window) {
        self.update(window);

        let bg = self.box_color.lerp(&self.box_color_checked, self.anim_t);

        // Border
        window.draw_rect_f(
            self.pos_x, self.pos_y, self.size, self.size,
            self.radius, &self.border_color, 0,
        );

        // Inner box
        let inner_x = self.pos_x + self.border_size;
        let inner_y = self.pos_y + self.border_size;
        let inner_s = self.size.saturating_sub(self.border_size * 2);
        let inner_r = self.radius.saturating_sub(self.border_size);
        window.draw_rect_f(inner_x, inner_y, inner_s, inner_s, inner_r, &bg, 0);

        // Checkmark (two lines forming a check)
        if self.anim_t > 0.1 {
            let s = self.size as f32;
            let ox = self.pos_x as f32;
            let oy = self.pos_y as f32;

            // Check mark points (relative to box, scaled)
            let p1 = (ox + s * 0.22, oy + s * 0.50);
            let p2 = (ox + s * 0.42, oy + s * 0.72);
            let p3 = (ox + s * 0.78, oy + s * 0.30);

            let th = (s * 0.12).max(1.5) as usize;
            let alpha_color = crate::color::Color::rgba(
                self.check_color.r, self.check_color.g, self.check_color.b,
                (self.anim_t * 255.0) as u8,
            );

            window.draw_line(p1.0 as isize, p1.1 as isize, p2.0 as isize, p2.1 as isize, th, alpha_color);
            window.draw_line(p2.0 as isize, p2.1 as isize, p3.0 as isize, p3.1 as isize, th, alpha_color);
        }

        // Label
        if let Some(ref label) = self.label {
            let label_x = self.pos_x + self.size + self.label_gap;
            let label_y = self.pos_y as f32 + (self.size as f32 - self.label_size) / 2.0;
            window.draw_text(label_x, label_y as usize, label, self.label_size, &self.label_color);
        }
    }
}
