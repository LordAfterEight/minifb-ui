pub struct Slider {
    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub height: usize,

    pub min: f32,
    pub max: f32,
    normalized: f32,

    pub handle_width: usize,
    pub handle_color: crate::color::Color,
    pub handle_color_active: crate::color::Color,
    /// Corner radius for the handle
    pub handle_radius: usize,

    pub track_color: crate::color::Color,
    pub track_filled_color: crate::color::Color,
    pub border_color: crate::color::Color,
    pub border_size: usize,
    /// Corner radius for the track
    pub radius: usize,

    /// Whether the handle draws over or under the border
    pub handle_overlay: bool,

    pub dragging: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            width: 200,
            height: 20,
            min: 0.0,
            max: 1.0,
            normalized: 0.0,
            handle_width: 10,
            handle_color: crate::color::Color::new(200, 200, 200),
            handle_color_active: crate::color::Color::new(255, 255, 255),
            handle_radius: 0,
            handle_overlay: false,
            track_color: crate::color::Color::new(50, 50, 50),
            track_filled_color: crate::color::Color::new(80, 130, 200),
            border_color: crate::color::Color::new(100, 100, 100),
            border_size: 1,
            radius: 0,
            dragging: false,
        }
    }
}

impl Slider {
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

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn default_value(mut self, val: f32) -> Self {
        self.normalized = ((val - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
        self
    }

    pub fn handle_width(mut self, width: usize) -> Self {
        self.handle_width = width;
        self
    }

    pub fn handle_color(mut self, color: crate::color::Color) -> Self {
        self.handle_color = color;
        self
    }

    pub fn handle_color_active(mut self, color: crate::color::Color) -> Self {
        self.handle_color_active = color;
        self
    }

    /// Sets the corner radius for the handle/thumb
    pub fn handle_radius(mut self, radius: usize) -> Self {
        self.handle_radius = radius;
        self
    }

    /// If true, the handle draws over the border; if false, it goes underneath
    pub fn handle_overlay(mut self, overlay: bool) -> Self {
        self.handle_overlay = overlay;
        self
    }

    pub fn track_color(mut self, color: crate::color::Color) -> Self {
        self.track_color = color;
        self
    }

    pub fn track_filled_color(mut self, color: crate::color::Color) -> Self {
        self.track_filled_color = color;
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

    /// Sets the corner radius for the track
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }

    /// Returns the current value mapped to [min, max]
    pub fn value(&self) -> f32 {
        self.min + self.normalized * (self.max - self.min)
    }

    /// Sets the value programmatically, clamping to the configured range
    pub fn set_value(&mut self, val: f32) {
        self.normalized = ((val - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
    }

    fn update(&mut self, window: &mut crate::window::Window) {
        let mouse = window.get_mouse_state();
        let mx = mouse.pos_x as usize;
        let my = mouse.pos_y as usize;
        let in_bounds = mx >= self.pos_x
            && mx < self.pos_x + self.width
            && my >= self.pos_y
            && my < self.pos_y + self.height;

        if mouse.lmb_clicked {
            if in_bounds || self.dragging {
                self.dragging = true;
                let inner_x = self.pos_x + self.border_size;
                let inner_w = self.width.saturating_sub(self.border_size * 2);
                let usable_w = inner_w.saturating_sub(self.handle_width);
                if usable_w > 0 {
                    let handle_half = self.handle_width / 2;
                    let track_start = (inner_x + handle_half) as f32;
                    let track_end = (inner_x + handle_half + usable_w) as f32;
                    self.normalized =
                        ((mouse.pos_x - track_start) / (track_end - track_start)).clamp(0.0, 1.0);
                }
            }
        } else {
            self.dragging = false;
        }
    }

    /// Draws and updates the slider
    pub fn draw(&mut self, window: &mut crate::window::Window) {
        self.update(window);

        let inner_x = self.pos_x + self.border_size;
        let inner_y = self.pos_y + self.border_size;
        let inner_w = self.width.saturating_sub(self.border_size * 2);
        let inner_h = self.height.saturating_sub(self.border_size * 2);
        let inner_radius = self.radius.saturating_sub(self.border_size);

        // Track background
        window.draw_rounded_rect_f(self.pos_x, self.pos_y, self.width, self.height, self.radius, &self.border_color);
        window.draw_rounded_rect_f(inner_x, inner_y, inner_w, inner_h, inner_radius, &self.track_color);

        // Handle position
        let usable_w = inner_w.saturating_sub(self.handle_width);
        let handle_x = inner_x + (self.normalized * usable_w as f32) as usize;

        // Filled portion of track
        let filled_w = (handle_x + self.handle_width / 2).saturating_sub(inner_x);
        if filled_w > 0 {
            window.draw_rounded_rect_f(inner_x, inner_y, filled_w, inner_h, inner_radius, &self.track_filled_color);
        }

        let handle_col = if self.dragging {
            &self.handle_color_active
        } else {
            &self.handle_color
        };

        if self.handle_overlay {
            // Handle on top of everything at full outer height
            window.draw_rounded_rect_f(handle_x, self.pos_y, self.handle_width, self.height, self.handle_radius, handle_col);
        } else {
            // Handle within inner area, then solid border ring on top
            window.draw_rounded_rect_f(handle_x, inner_y, self.handle_width, inner_h, self.handle_radius, handle_col);
            self.draw_border_ring(window);
        }
    }

    /// Draws the border as a solid filled ring (outer minus inner rounded rect).
    /// Pixels inside the outer shape but outside the inner shape get border color
    /// at full opacity, fully occluding anything underneath.
    fn draw_border_ring(&self, window: &mut crate::window::Window) {
        if self.border_size == 0 {
            return;
        }
        let aa = window.aa;
        let ox = self.pos_x as f32;
        let oy = self.pos_y as f32;
        let ow = self.width as f32;
        let oh = self.height as f32;
        let or = (self.radius as f32).min(ow / 2.0).min(oh / 2.0);

        let ix = (self.pos_x + self.border_size) as f32;
        let iy = (self.pos_y + self.border_size) as f32;
        let iw = self.width.saturating_sub(self.border_size * 2) as f32;
        let ih = self.height.saturating_sub(self.border_size * 2) as f32;
        let ir = (self.radius.saturating_sub(self.border_size) as f32).min(iw / 2.0).min(ih / 2.0);

        let border_val = self.border_color.as_u32();

        let min_px = (ox - aa).floor().max(0.0) as usize;
        let max_px = ((ox + ow + aa).ceil() as usize).min(window.width - 1);
        let min_py = (oy - aa).floor().max(0.0) as usize;
        let max_py = ((oy + oh + aa).ceil() as usize).min(window.height - 1);

        for py in min_py..=max_py {
            for px in min_px..=max_px {
                let pfx = px as f32 + 0.5;
                let pfy = py as f32 + 0.5;

                let outer_dist = Self::sdf(pfx, pfy, ox, oy, ow, oh, or);
                let inner_dist = Self::sdf(pfx, pfy, ix, iy, iw, ih, ir);

                // Coverage from outer shape (1 inside, 0 outside)
                let outer_cov = ((aa * 0.5 - outer_dist) / aa).clamp(0.0, 1.0);
                // Coverage from inner shape (1 inside, 0 outside)
                let inner_cov = ((aa * 0.5 - inner_dist) / aa).clamp(0.0, 1.0);

                // Border ring = inside outer but outside inner
                let ring_cov = (outer_cov - inner_cov).clamp(0.0, 1.0);

                if ring_cov >= 1.0 {
                    window.framebuffer_raw[py * window.width + px] = border_val;
                } else if ring_cov > 0.0 {
                    // Blend at AA edges
                    let idx = py * window.width + px;
                    let bg = window.framebuffer_raw[idx];
                    let bg_r = (bg >> 16) & 0xFF;
                    let bg_g = (bg >> 8) & 0xFF;
                    let bg_b = bg & 0xFF;
                    let a = (ring_cov * 255.0) as u32;
                    let inv = 255 - a;
                    let r = (self.border_color.r as u32 * a + bg_r * inv) / 255;
                    let g = (self.border_color.g as u32 * a + bg_g * inv) / 255;
                    let b = (self.border_color.b as u32 * a + bg_b * inv) / 255;
                    window.framebuffer_raw[idx] = (r << 16) | (g << 8) | b;
                }
            }
        }
    }

    #[inline]
    fn sdf(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32, r: f32) -> f32 {
        let cx = x + w / 2.0;
        let cy = y + h / 2.0;
        let dx = (px - cx).abs() - (w / 2.0 - r);
        let dy = (py - cy).abs() - (h / 2.0 - r);
        let outside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt();
        let inside = dx.max(dy).min(0.0);
        outside + inside - r
    }
}
