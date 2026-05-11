pub struct Switch {
    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub height: usize,

    pub on: bool,
    anim_t: f32,
    pub anim_speed: f32,

    pub track_color_off: crate::color::Color,
    pub track_color_on: crate::color::Color,
    pub thumb_color: crate::color::Color,
    pub thumb_padding: f32,

    pub label: Option<crate::ui::text::Text>,
    pub label_size: f32,
    pub label_color: crate::color::Color,
    pub label_gap: usize,

    lmb_was_down: bool,
    toggled_this_frame: bool,
}

impl Default for Switch {
    fn default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            width: 50,
            height: 26,
            on: false,
            anim_t: 0.0,
            anim_speed: 0.08,
            track_color_off: crate::color::Color::new(120, 120, 130),
            track_color_on: crate::color::Color::new(52, 199, 89),
            thumb_color: crate::color::Color::new(255, 255, 255),
            thumb_padding: 2.0,
            label: None,
            label_size: 14.0,
            label_color: crate::color::Color::new(200, 200, 200),
            label_gap: 10,
            lmb_was_down: false,
            toggled_this_frame: false,
        }
    }
}

impl Switch {
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

    pub fn default_on(mut self, on: bool) -> Self {
        self.on = on;
        self.anim_t = if on { 1.0 } else { 0.0 };
        self
    }

    pub fn anim_speed(mut self, speed: f32) -> Self {
        self.anim_speed = speed;
        self
    }

    pub fn track_color_off(mut self, color: crate::color::Color) -> Self {
        self.track_color_off = color;
        self
    }

    pub fn track_color_on(mut self, color: crate::color::Color) -> Self {
        self.track_color_on = color;
        self
    }

    pub fn thumb_color(mut self, color: crate::color::Color) -> Self {
        self.thumb_color = color;
        self
    }

    pub fn thumb_padding(mut self, padding: f32) -> Self {
        self.thumb_padding = padding;
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

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn set_on(&mut self, on: bool) {
        self.on = on;
    }

    pub fn just_toggled(&self) -> bool {
        self.toggled_this_frame
    }

    fn update(&mut self, window: &mut crate::window::Window) {
        self.toggled_this_frame = false;

        let mouse = window.get_mouse_state();
        let lmb = mouse.lmb_clicked;
        let lmb_just_pressed = lmb && !self.lmb_was_down;

        if lmb_just_pressed {
            let mx = mouse.pos_x;
            let my = mouse.pos_y;
            let in_bounds = mx >= self.pos_x as f32
                && mx < (self.pos_x + self.width) as f32
                && my >= self.pos_y as f32
                && my < (self.pos_y + self.height) as f32;

            if in_bounds {
                self.on = !self.on;
                self.toggled_this_frame = true;
            }
        }

        self.lmb_was_down = lmb;

        // Animate toward target
        let target = if self.on { 1.0 } else { 0.0 };
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

        let track_color = self.track_color_off.lerp(&self.track_color_on, self.anim_t);
        let radius = self.height / 2;

        // Draw track
        window.draw_rounded_rect_f(
            self.pos_x, self.pos_y,
            self.width, self.height,
            radius,
            &track_color,
        );

        // Compute thumb position
        let thumb_radius = (self.height as f32 / 2.0) - self.thumb_padding;
        let left_cx = self.pos_x as f32 + self.thumb_padding + thumb_radius;
        let right_cx = self.pos_x as f32 + self.width as f32 - self.thumb_padding - thumb_radius;
        let thumb_cx = left_cx + self.anim_t * (right_cx - left_cx);
        let thumb_cy = self.pos_y as f32 + self.height as f32 / 2.0;

        // Thumb shadow (subtle offset down)
        window.draw_circle_f(
            thumb_cx as isize,
            (thumb_cy + 1.0) as isize,
            thumb_radius as usize,
            &crate::color::Color::rgba(0, 0, 0, 30),
        );

        // Thumb
        window.draw_circle_f(
            thumb_cx as isize,
            thumb_cy as isize,
            thumb_radius as usize,
            &self.thumb_color,
        );

        // Label
        if let Some(ref label) = self.label {
            let label_x = self.pos_x + self.width + self.label_gap;
            let label_h = self.label_size;
            let label_y = self.pos_y as f32 + (self.height as f32 - label_h) / 2.0;
            window.draw_text(label_x, label_y as usize, label, self.label_size, &self.label_color);
        }
    }
}
