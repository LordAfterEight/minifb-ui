/// Owns the window and the raw flat framebuffer to be rendered.
pub struct Window {
    pub window: minifb::Window,
    pub width: usize,
    pub height: usize,
    pub framebuffer_raw: Vec<u32>,
    pub is_fullscreen: bool,
}

impl Window {
    /// Creates a non-resizable window with a resolution of 1280 by 720 pixels
    pub fn default() -> Self {
        Self {
            window: minifb::Window::new(
                "minifb-ui",
                1280,
                720,
                minifb::WindowOptions {
                    resize: false,
                    scale: minifb::Scale::X1,
                    scale_mode: minifb::ScaleMode::AspectRatioStretch,
                    ..Default::default()
                },
            )
            .unwrap(),
            width: 1280,
            height: 720,
            framebuffer_raw: vec![0u32; 1280 * 720],
            is_fullscreen: false,
        }
    }

    /// Creates a window with custom resolution, can be borderless and resizable
    pub fn custom(name: &str, width: usize, height: usize, borders: bool, resizable: bool) -> Self {
        Self {
            window: minifb::Window::new(
                name,
                width,
                height,
                minifb::WindowOptions {
                    resize: resizable,
                    scale: minifb::Scale::X1,
                    scale_mode: minifb::ScaleMode::Center,
                    borderless: borders,
                    ..Default::default()
                },
            )
            .unwrap(),
            width: width,
            height: height,
            framebuffer_raw: vec![0u32; width * height],
            is_fullscreen: false,
        }
    }

    /// Draws a single pixel at given coordinates with color
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: &crate::color::Color) {
        self.framebuffer_raw[y * self.width + x] = color.as_u32()
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.framebuffer_raw[y * self.width + x]
    }

    pub fn clear(&mut self, color: &crate::color::Color) {
        self.draw_rect_f(0, 0, self.width, self.height, color);
    }

    /// Draws a straight line from coordinate to coordinate with color
    pub fn draw_line(
        &mut self,
        x0: isize,
        y0: isize,
        x1: isize,
        y1: isize,
        th: usize,
        color: crate::color::Color,
    ) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1isize } else { -1 };
        let sy = if y0 < y1 { 1isize } else { -1 };
        let value = color.as_u32();
        let half = (th / 2) as isize;

        let mut x = x0;
        let mut y = y0;
        let mut err = dx - dy;

        loop {
            for oy in -half..half {
                for ox in -half..half {
                    let px = x + ox;
                    let py = y + oy;
                    if px >= 0 && px < self.width as isize && py >= 0 && py < self.height as isize {
                        self.framebuffer_raw[py as usize * self.width + px as usize] = value;
                    }
                }
            }

            if x == x1 && y == y1 { break; }

            let e2 = err * 2;
            if e2 > -dy { err -= dy; x += sx; }
            if e2 <  dx { err += dx; y += sy; }
        }
    }

    /// Draws a filled rectangle at given coordinates with size and color
    pub fn draw_rect_f(&mut self, x: usize, y: usize, w: usize, h: usize, color: &crate::color::Color) {
        let value = color.as_u32();
        let start = y * self.width + x;
        for dy in 0..h {
            let row = &mut self.framebuffer_raw[start + dy * self.width..][..w];
            row.fill(value);
        }
    }

    /// Draws a hollow rectangle at given coordinates with size and color
    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: &crate::color::Color) {
        let value = color.as_u32();
        if h >= 1 {
            let top = y * self.width + x;
            self.framebuffer_raw[top..top + w].fill(value);
        }
        if h >= 2 {
            let bottom = (y + h - 1) * self.width + x;
            self.framebuffer_raw[bottom..bottom + w].fill(value);
        }
        if w >= 2 && h >= 2 {
            let left_start = (y + 1) * self.width + x;
            let right_start = (y + 1) * self.width + x + w - 1;

            for dy in 0..(h - 2) {
                let offset = dy * self.width;
                self.framebuffer_raw[left_start + offset] = value;
                self.framebuffer_raw[right_start + offset] = value;
            }
        }
    }

    /// Draws text using the passed font
    pub fn draw_text(&mut self, x: usize, y: usize, text: &crate::ui::text::Text, size: f32, color: &crate::color::Color) {
        use fontdue::layout::{Layout, LayoutSettings, CoordinateSystem, TextStyle};

        let fg_r = color.r as u32;
        let fg_g = color.g as u32;
        let fg_b = color.b as u32;

        let fonts = text.font.as_slice();

        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: x as f32,
            y: y as f32,
            ..Default::default()
        });

        layout.append(&fonts, &TextStyle::new(&text.text, size, 0));

        for glyph in layout.glyphs() {
            let (metrics, bitmap) = text.font.font.rasterize_config(glyph.key);

            let glyph_x = glyph.x as i32;
            let glyph_y = glyph.y as i32;

            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let px = glyph_x + col as i32;
                    let py = glyph_y + row as i32;

                    if px < 0 || py < 0 { continue; }
                    let (px, py) = (px as usize, py as usize);
                    if px >= self.width || py >= self.height { continue; }

                    let alpha = bitmap[row * metrics.width + col] as u32;
                    if alpha == 0 { continue; }

                    let idx = py * self.width + px;
                    let bg = self.framebuffer_raw[idx];
                    let bg_r = (bg >> 16) & 0xFF;
                    let bg_g = (bg >> 8)  & 0xFF;
                    let bg_b =  bg        & 0xFF;

                    let r = (fg_r * alpha + bg_r * (255 - alpha)) / 255;
                    let g = (fg_g * alpha + bg_g * (255 - alpha)) / 255;
                    let b = (fg_b * alpha + bg_b * (255 - alpha)) / 255;

                    self.framebuffer_raw[idx] = (r << 16) | (g << 8) | b;
                }
            }
        }
    }

    /// Updates the window. Should be called in a loop until the window is to be closed
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(self.framebuffer_raw.as_slice(), self.width, self.height)
            .unwrap();
    }

    pub fn get_mouse_state(&self) -> MouseState {
        let pos = self.window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
        let lmb = self.window.get_mouse_down(minifb::MouseButton::Left);
        let rmb = self.window.get_mouse_down(minifb::MouseButton::Right);
        MouseState {
            pos_x: pos.0,
            pos_y: pos.1,
            rmb_clicked: rmb,
            lmb_clicked: lmb,
        }
    }
}

pub struct MouseState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub rmb_clicked: bool,
    pub lmb_clicked: bool
}
