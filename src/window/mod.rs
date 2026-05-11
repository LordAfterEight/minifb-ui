use std::sync::{Arc, Mutex};

struct CharCallback {
    buffer: Arc<Mutex<Vec<char>>>,
}

impl minifb::InputCallback for CharCallback {
    fn add_char(&mut self, uni_char: u32) {
        if let Some(c) = char::from_u32(uni_char) {
            if let Ok(mut buf) = self.buffer.lock() {
                buf.push(c);
            }
        }
    }
}

/// Owns the window and the raw flat framebuffer to be rendered.
pub struct Window {
    pub window: minifb::Window,
    pub width: usize,
    pub height: usize,
    pub framebuffer_raw: Vec<u32>,
    pub is_fullscreen: bool,
    /// Anti-aliasing softness in pixels. 0.0 = hard edges (no AA), 1.0 = standard AA.
    /// Higher values produce softer/blurrier edges.
    pub aa: f32,
    input_buffer: Arc<Mutex<Vec<char>>>,
}

impl Window {
    /// Creates a non-resizable window with a resolution of 1280 by 720 pixels
    pub fn default() -> Self {
        let input_buffer = Arc::new(Mutex::new(Vec::new()));
        let mut window = minifb::Window::new(
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
        .unwrap();
        window.set_input_callback(Box::new(CharCallback {
            buffer: Arc::clone(&input_buffer),
        }));
        Self {
            window,
            width: 1280,
            height: 720,
            framebuffer_raw: vec![0u32; 1280 * 720],
            is_fullscreen: false,
            aa: 1.0,
            input_buffer,
        }
    }

    /// Creates a window with custom resolution, can be borderless and resizable
    pub fn custom(name: &str, width: usize, height: usize, borders: bool, resizable: bool) -> Self {
        let input_buffer = Arc::new(Mutex::new(Vec::new()));
        let mut window = minifb::Window::new(
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
        .unwrap();
        window.set_input_callback(Box::new(CharCallback {
            buffer: Arc::clone(&input_buffer),
        }));
        Self {
            window,
            width,
            height,
            framebuffer_raw: vec![0u32; width * height],
            is_fullscreen: false,
            aa: 1.0,
            input_buffer,
        }
    }

    /// Drains and returns all characters typed since the last call.
    pub fn get_typed_chars(&self) -> Vec<char> {
        if let Ok(mut buf) = self.input_buffer.lock() {
            buf.drain(..).collect()
        } else {
            Vec::new()
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
        self.framebuffer_raw.fill(color.as_u32());
    }

    // ─── Lines ──────────────────────────────────────────────────────────

    /// Draws a straight line. Uses AA when `self.aa > 0`.
    pub fn draw_line(
        &mut self,
        x0: isize,
        y0: isize,
        x1: isize,
        y1: isize,
        th: usize,
        color: crate::color::Color,
    ) {
        if self.aa > 0.0 {
            self.draw_line_aa_impl(x0 as f32, y0 as f32, x1 as f32, y1 as f32, th as f32, &color);
        } else {
            self.draw_line_aliased(x0, y0, x1, y1, th, &color);
        }
    }

    /// Draws a dashed line. `dash_len` is pixels on, `gap_len` is pixels off.
    pub fn draw_line_dashed(
        &mut self,
        x0: isize,
        y0: isize,
        x1: isize,
        y1: isize,
        th: usize,
        dash_len: usize,
        gap_len: usize,
        color: crate::color::Color,
    ) {
        let dx = (x1 - x0) as f32;
        let dy = (y1 - y0) as f32;
        let total_len = (dx * dx + dy * dy).sqrt();
        if total_len < 0.5 {
            return;
        }
        let dir_x = dx / total_len;
        let dir_y = dy / total_len;
        let pattern_len = (dash_len + gap_len) as f32;

        let mut dist = 0.0f32;
        while dist < total_len {
            let seg_end = (dist + dash_len as f32).min(total_len);
            let sx = x0 as f32 + dir_x * dist;
            let sy = y0 as f32 + dir_y * dist;
            let ex = x0 as f32 + dir_x * seg_end;
            let ey = y0 as f32 + dir_y * seg_end;

            if self.aa > 0.0 {
                self.draw_line_aa_impl(sx, sy, ex, ey, th as f32, &color);
            } else {
                self.draw_line_aliased(sx as isize, sy as isize, ex as isize, ey as isize, th, &color);
            }

            dist += pattern_len;
        }
    }

    /// Draws a dotted line (dash=1, gap=spacing)
    pub fn draw_line_dotted(
        &mut self,
        x0: isize,
        y0: isize,
        x1: isize,
        y1: isize,
        th: usize,
        spacing: usize,
        color: crate::color::Color,
    ) {
        self.draw_line_dashed(x0, y0, x1, y1, th, 1, spacing, color);
    }

    // ─── Rectangles ─────────────────────────────────────────────────────

    /// Draws a filled rectangle (axis-aligned, no AA needed)
    pub fn draw_rect_f(&mut self, x: usize, y: usize, w: usize, h: usize, color: &crate::color::Color) {
        let value = color.as_u32();
        let start = y * self.width + x;
        for dy in 0..h {
            let row = &mut self.framebuffer_raw[start + dy * self.width..][..w];
            row.fill(value);
        }
    }

    /// Draws a hollow rectangle (axis-aligned, no AA needed)
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

    // ─── Rounded Rectangles ─────────────────────────────────────────────

    /// Draws a filled rounded rectangle
    pub fn draw_rounded_rect_f(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        radius: usize,
        color: &crate::color::Color,
    ) {
        if self.aa > 0.0 {
            self.draw_rounded_rect_f_sdf(x as f32, y as f32, w as f32, h as f32, radius as f32, color);
        } else {
            self.draw_rounded_rect_f_aliased(x, y, w, h, radius, color);
        }
    }

    /// Draws a hollow rounded rectangle
    pub fn draw_rounded_rect(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        radius: usize,
        color: &crate::color::Color,
    ) {
        if self.aa > 0.0 {
            self.draw_rounded_rect_sdf(x as f32, y as f32, w as f32, h as f32, radius as f32, 1.0, color);
        } else {
            self.draw_rounded_rect_aliased(x, y, w, h, radius, color);
        }
    }

    // ─── Circles / Ellipses ─────────────────────────────────────────────

    /// Draws a filled circle
    pub fn draw_circle_f(
        &mut self,
        cx: isize,
        cy: isize,
        radius: usize,
        color: &crate::color::Color,
    ) {
        if self.aa > 0.0 {
            self.draw_circle_f_sdf(cx as f32, cy as f32, radius as f32, color);
        } else {
            self.draw_circle_f_aliased(cx, cy, radius, color);
        }
    }

    /// Draws a hollow circle
    pub fn draw_circle(
        &mut self,
        cx: isize,
        cy: isize,
        radius: usize,
        color: &crate::color::Color,
    ) {
        if self.aa > 0.0 {
            self.draw_circle_sdf(cx as f32, cy as f32, radius as f32, 1.0, color);
        } else {
            self.draw_circle_aliased(cx, cy, radius, color);
        }
    }

    /// Draws a filled ellipse
    pub fn draw_ellipse_f(
        &mut self,
        cx: isize,
        cy: isize,
        rx: usize,
        ry: usize,
        color: &crate::color::Color,
    ) {
        if self.aa > 0.0 {
            self.draw_ellipse_f_sdf(cx as f32, cy as f32, rx as f32, ry as f32, color);
        } else {
            self.draw_ellipse_f_aliased(cx, cy, rx, ry, color);
        }
    }

    /// Draws a hollow ellipse
    pub fn draw_ellipse(
        &mut self,
        cx: isize,
        cy: isize,
        rx: usize,
        ry: usize,
        color: &crate::color::Color,
    ) {
        if self.aa > 0.0 {
            self.draw_ellipse_sdf(cx as f32, cy as f32, rx as f32, ry as f32, 1.0, color);
        } else {
            self.draw_ellipse_aliased(cx, cy, rx, ry, color);
        }
    }

    // ─── Gradients ──────────────────────────────────────────────────────

    /// Draws a filled rectangle with a horizontal linear gradient (left to right)
    pub fn draw_gradient_h(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color_left: &crate::color::Color,
        color_right: &crate::color::Color,
    ) {
        if w == 0 || h == 0 {
            return;
        }
        for dy in 0..h {
            let row = (y + dy) * self.width;
            for dx in 0..w {
                let t = dx as f32 / (w - 1).max(1) as f32;
                let r = (color_left.r as f32 * (1.0 - t) + color_right.r as f32 * t) as u32;
                let g = (color_left.g as f32 * (1.0 - t) + color_right.g as f32 * t) as u32;
                let b = (color_left.b as f32 * (1.0 - t) + color_right.b as f32 * t) as u32;
                self.framebuffer_raw[row + x + dx] = (r << 16) | (g << 8) | b;
            }
        }
    }

    /// Draws a filled rectangle with a vertical linear gradient (top to bottom)
    pub fn draw_gradient_v(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color_top: &crate::color::Color,
        color_bottom: &crate::color::Color,
    ) {
        if w == 0 || h == 0 {
            return;
        }
        for dy in 0..h {
            let t = dy as f32 / (h - 1).max(1) as f32;
            let r = (color_top.r as f32 * (1.0 - t) + color_bottom.r as f32 * t) as u32;
            let g = (color_top.g as f32 * (1.0 - t) + color_bottom.g as f32 * t) as u32;
            let b = (color_top.b as f32 * (1.0 - t) + color_bottom.b as f32 * t) as u32;
            let value = (r << 16) | (g << 8) | b;
            let row = (y + dy) * self.width + x;
            self.framebuffer_raw[row..row + w].fill(value);
        }
    }

    /// Draws a filled rounded rectangle with a vertical linear gradient
    pub fn draw_rounded_gradient_v(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        radius: usize,
        color_top: &crate::color::Color,
        color_bottom: &crate::color::Color,
    ) {
        if w == 0 || h == 0 {
            return;
        }
        let aa = self.aa;
        let fx = x as f32;
        let fy = y as f32;
        let fw = w as f32;
        let fh = h as f32;
        let r = (radius as f32).min(fw / 2.0).min(fh / 2.0);

        if aa > 0.0 {
            let min_px = ((fx - 1.0).floor() as isize).max(0);
            let max_px = ((fx + fw + 1.0).ceil() as isize).min(self.width as isize - 1);
            let min_py = ((fy - 1.0).floor() as isize).max(0);
            let max_py = ((fy + fh + 1.0).ceil() as isize).min(self.height as isize - 1);

            for py in min_py..=max_py {
                let t = (py as f32 - fy) / (fh - 1.0).max(1.0);
                let t = t.clamp(0.0, 1.0);
                let row_color = color_top.lerp(color_bottom, t);
                for px in min_px..=max_px {
                    let pfx = px as f32 + 0.5;
                    let pfy = py as f32 + 0.5;
                    let dist = rounded_rect_sdf(pfx, pfy, fx, fy, fw, fh, r);
                    let coverage = (aa * 0.5 - dist).clamp(0.0, 1.0) / aa;
                    if coverage > 0.0 {
                        self.blend_pixel(px, py, &row_color, coverage);
                    }
                }
            }
        } else {
            let ri = radius.min(w / 2).min(h / 2);
            for dy in 0..h {
                let t = dy as f32 / (h - 1).max(1) as f32;
                let cr = (color_top.r as f32 * (1.0 - t) + color_bottom.r as f32 * t) as u32;
                let cg = (color_top.g as f32 * (1.0 - t) + color_bottom.g as f32 * t) as u32;
                let cb = (color_top.b as f32 * (1.0 - t) + color_bottom.b as f32 * t) as u32;
                let value = (cr << 16) | (cg << 8) | cb;

                let (row_start, row_end) = if dy < ri {
                    let cy = ri - dy;
                    let dx = ri - isqrt(ri * ri - cy * cy);
                    (x + dx, x + w - dx)
                } else if dy >= h - ri {
                    let cy = dy - (h - 1 - ri);
                    let dx = ri - isqrt(ri * ri - cy * cy);
                    (x + dx, x + w - dx)
                } else {
                    (x, x + w)
                };

                let start = (y + dy) * self.width + row_start;
                let len = row_end - row_start;
                self.framebuffer_raw[start..start + len].fill(value);
            }
        }
    }

    // ─── Alpha Blending ─────────────────────────────────────────────────

    /// Draws a single pixel with alpha blending
    pub fn draw_pixel_alpha(&mut self, x: usize, y: usize, color: &crate::color::Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let alpha = color.a as u32;
        if alpha == 0 {
            return;
        }
        let idx = y * self.width + x;
        if alpha == 255 {
            self.framebuffer_raw[idx] = color.as_u32();
            return;
        }
        let bg = self.framebuffer_raw[idx];
        let bg_r = (bg >> 16) & 0xFF;
        let bg_g = (bg >> 8) & 0xFF;
        let bg_b = bg & 0xFF;
        let inv = 255 - alpha;
        let r = (color.r as u32 * alpha + bg_r * inv) / 255;
        let g = (color.g as u32 * alpha + bg_g * inv) / 255;
        let b = (color.b as u32 * alpha + bg_b * inv) / 255;
        self.framebuffer_raw[idx] = (r << 16) | (g << 8) | b;
    }

    /// Draws a filled rectangle with alpha blending
    pub fn draw_rect_f_alpha(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: &crate::color::Color,
    ) {
        let alpha = color.a as u32;
        if alpha == 0 {
            return;
        }
        if alpha == 255 {
            self.draw_rect_f(x, y, w, h, color);
            return;
        }
        let fg_r = color.r as u32;
        let fg_g = color.g as u32;
        let fg_b = color.b as u32;
        let inv = 255 - alpha;

        for dy in 0..h {
            let row = (y + dy) * self.width + x;
            for dx in 0..w {
                let idx = row + dx;
                let bg = self.framebuffer_raw[idx];
                let bg_r = (bg >> 16) & 0xFF;
                let bg_g = (bg >> 8) & 0xFF;
                let bg_b = bg & 0xFF;
                let r = (fg_r * alpha + bg_r * inv) / 255;
                let g = (fg_g * alpha + bg_g * inv) / 255;
                let b = (fg_b * alpha + bg_b * inv) / 255;
                self.framebuffer_raw[idx] = (r << 16) | (g << 8) | b;
            }
        }
    }

    /// Draws a filled rounded rectangle with alpha blending
    pub fn draw_rounded_rect_f_alpha(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        radius: usize,
        color: &crate::color::Color,
    ) {
        let aa = self.aa;
        let fx = x as f32;
        let fy = y as f32;
        let fw = w as f32;
        let fh = h as f32;
        let r = (radius as f32).min(fw / 2.0).min(fh / 2.0);
        let base_alpha = color.a as f32 / 255.0;

        let min_px = ((fx - 1.0).floor() as isize).max(0);
        let max_px = ((fx + fw + 1.0).ceil() as isize).min(self.width as isize - 1);
        let min_py = ((fy - 1.0).floor() as isize).max(0);
        let max_py = ((fy + fh + 1.0).ceil() as isize).min(self.height as isize - 1);

        for py in min_py..=max_py {
            for px in min_px..=max_px {
                let pfx = px as f32 + 0.5;
                let pfy = py as f32 + 0.5;
                let dist = rounded_rect_sdf(pfx, pfy, fx, fy, fw, fh, r);
                let shape_coverage = if aa > 0.0 {
                    (aa * 0.5 - dist).clamp(0.0, 1.0) / aa
                } else {
                    if dist <= 0.0 { 1.0 } else { 0.0 }
                };
                let final_alpha = shape_coverage * base_alpha;
                if final_alpha > 0.0 {
                    self.blend_pixel(px, py, color, final_alpha);
                }
            }
        }
    }

    // ─── Box Shadow ─────────────────────────────────────────────────────

    /// Draws a rectangular drop shadow.
    pub fn draw_box_shadow(
        &mut self,
        x: isize,
        y: isize,
        w: usize,
        h: usize,
        offset_x: isize,
        offset_y: isize,
        spread: isize,
        blur: usize,
        color: &crate::color::Color,
    ) {
        let sx = x + offset_x - spread;
        let sy = y + offset_y - spread;
        let sw = (w as isize + spread * 2) as usize;
        let sh = (h as isize + spread * 2) as usize;
        let blur_f = blur as f32;

        let total_w = sw + blur * 2;
        let total_h = sh + blur * 2;
        let draw_x = sx - blur as isize;
        let draw_y = sy - blur as isize;

        for dy in 0..total_h {
            let py = draw_y + dy as isize;
            if py < 0 || py >= self.height as isize {
                continue;
            }
            for dx in 0..total_w {
                let px = draw_x + dx as isize;
                if px < 0 || px >= self.width as isize {
                    continue;
                }

                let dist_x = if dx < blur {
                    (blur - dx) as f32
                } else if dx >= blur + sw {
                    (dx - blur - sw + 1) as f32
                } else {
                    0.0
                };
                let dist_y = if dy < blur {
                    (blur - dy) as f32
                } else if dy >= blur + sh {
                    (dy - blur - sh + 1) as f32
                } else {
                    0.0
                };

                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
                if dist >= blur_f {
                    continue;
                }

                let alpha = (color.a as f32 / 255.0) * (1.0 - dist / blur_f);
                if alpha > 0.0 {
                    self.blend_pixel(px, py, color, alpha);
                }
            }
        }
    }

    // ─── Bezier Curves ──────────────────────────────────────────────────

    /// Draws a quadratic bezier curve from p0 to p2 with control point p1
    pub fn draw_bezier_quad(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        th: usize,
        color: &crate::color::Color,
    ) {
        let chord = ((x2 - x0).powi(2) + (y2 - y0).powi(2)).sqrt();
        let control_net = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt()
            + ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        let steps = ((chord + control_net) * 2.0).ceil() as usize;
        let steps = steps.max(32);

        let mut prev_x = x0;
        let mut prev_y = y0;

        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let inv = 1.0 - t;
            let cur_x = inv * inv * x0 + 2.0 * inv * t * x1 + t * t * x2;
            let cur_y = inv * inv * y0 + 2.0 * inv * t * y1 + t * t * y2;

            self.draw_line_aa_impl(prev_x, prev_y, cur_x, cur_y, th as f32, color);

            prev_x = cur_x;
            prev_y = cur_y;
        }
    }

    /// Draws a cubic bezier curve from p0 to p3 with control points p1 and p2
    pub fn draw_bezier_cubic(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        th: usize,
        color: &crate::color::Color,
    ) {
        let chord = ((x3 - x0).powi(2) + (y3 - y0).powi(2)).sqrt();
        let control_net = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt()
            + ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
            + ((x3 - x2).powi(2) + (y3 - y2).powi(2)).sqrt();
        let steps = ((chord + control_net) * 2.0).ceil() as usize;
        let steps = steps.max(32);

        let mut prev_x = x0;
        let mut prev_y = y0;

        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let inv = 1.0 - t;
            let cur_x = inv * inv * inv * x0
                + 3.0 * inv * inv * t * x1
                + 3.0 * inv * t * t * x2
                + t * t * t * x3;
            let cur_y = inv * inv * inv * y0
                + 3.0 * inv * inv * t * y1
                + 3.0 * inv * t * t * y2
                + t * t * t * y3;

            self.draw_line_aa_impl(prev_x, prev_y, cur_x, cur_y, th as f32, color);

            prev_x = cur_x;
            prev_y = cur_y;
        }
    }

    // ─── Text ───────────────────────────────────────────────────────────

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

    // ─── Window Management ──────────────────────────────────────────────

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

    // ─── Text Centering ─────────────────────────────────────────────────

    /// Draws text centered both horizontally and vertically within a bounding box
    pub fn draw_text_centered(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        text: &crate::ui::text::Text,
        size: f32,
        color: &crate::color::Color,
    ) {
        let text_w = text.get_width_precise(size);
        let lm = text.font.font.horizontal_line_metrics(size).unwrap();
        let tx = (x as f32 + (w as f32 / 2.0) - (text_w / 2.0)).max(0.0) as usize;
        let ty = (y as f32 + (h as f32 / 2.0) - (lm.ascent / 2.0) + (lm.descent / 3.0)).max(0.0) as usize;
        self.draw_text(tx, ty, text, size, color);
    }

    // ─── Blur ───────────────────────────────────────────────────────────

    /// Applies a separable box blur to a rectangular region of the framebuffer.
    /// Uses an O(n) sliding window — performance is independent of blur radius.
    pub fn blur_region(&mut self, rx: usize, ry: usize, rw: usize, rh: usize, radius: usize) {
        if radius == 0 || rw == 0 || rh == 0 { return; }
        let x0 = rx.min(self.width);
        let y0 = ry.min(self.height);
        let x1 = (rx + rw).min(self.width);
        let y1 = (ry + rh).min(self.height);
        let w = x1 - x0;
        let h = y1 - y0;
        if w == 0 || h == 0 { return; }

        let r = radius as i32;
        let diam = (2 * r + 1) as u32;
        let buf = &mut self.framebuffer_raw;
        let fb_w = self.width;
        let mut temp = vec![0u32; w * h];

        // Pass 1: horizontal — framebuffer → temp
        for row in 0..h {
            let (mut sr, mut sg, mut sb) = (0u32, 0u32, 0u32);
            let buf_row = (y0 + row) * fb_w + x0;
            for dx in -r..=r {
                let sx = dx.clamp(0, w as i32 - 1) as usize;
                let p = buf[buf_row + sx];
                sr += (p >> 16) & 0xFF; sg += (p >> 8) & 0xFF; sb += p & 0xFF;
            }
            for col in 0..w {
                temp[row * w + col] = ((sr / diam) << 16) | ((sg / diam) << 8) | (sb / diam);
                let old = (col as i32 - r).clamp(0, w as i32 - 1) as usize;
                let p = buf[buf_row + old];
                sr -= (p >> 16) & 0xFF; sg -= (p >> 8) & 0xFF; sb -= p & 0xFF;
                let new = (col as i32 + r + 1).clamp(0, w as i32 - 1) as usize;
                let p = buf[buf_row + new];
                sr += (p >> 16) & 0xFF; sg += (p >> 8) & 0xFF; sb += p & 0xFF;
            }
        }

        // Pass 2: vertical — temp → framebuffer
        for col in 0..w {
            let (mut sr, mut sg, mut sb) = (0u32, 0u32, 0u32);
            for dy in -r..=r {
                let sy = dy.clamp(0, h as i32 - 1) as usize;
                let p = temp[sy * w + col];
                sr += (p >> 16) & 0xFF; sg += (p >> 8) & 0xFF; sb += p & 0xFF;
            }
            for row in 0..h {
                buf[(y0 + row) * fb_w + x0 + col] = ((sr / diam) << 16) | ((sg / diam) << 8) | (sb / diam);
                let old = (row as i32 - r).clamp(0, h as i32 - 1) as usize;
                let p = temp[old * w + col];
                sr -= (p >> 16) & 0xFF; sg -= (p >> 8) & 0xFF; sb -= p & 0xFF;
                let new = (row as i32 + r + 1).clamp(0, h as i32 - 1) as usize;
                let p = temp[new * w + col];
                sr += (p >> 16) & 0xFF; sg += (p >> 8) & 0xFF; sb += p & 0xFF;
            }
        }
    }

    /// Applies a box blur to a region, preserving pixels outside rounded corners.
    /// This prevents the rectangular blur from leaking past rounded card edges.
    pub fn blur_region_rounded(
        &mut self,
        rx: usize, ry: usize, rw: usize, rh: usize,
        corner_radius: usize, blur_radius: usize,
    ) {
        if blur_radius == 0 || rw == 0 || rh == 0 { return; }
        let r2 = (corner_radius * corner_radius) as isize;
        let corners = [
            (rx, ry, rx + corner_radius, ry + corner_radius, rx + corner_radius, ry + corner_radius),
            (rx + rw - corner_radius, ry, rx + rw, ry + corner_radius, rx + rw - corner_radius, ry + corner_radius),
            (rx, ry + rh - corner_radius, rx + corner_radius, ry + rh, rx + corner_radius, ry + rh - corner_radius),
            (rx + rw - corner_radius, ry + rh - corner_radius, rx + rw, ry + rh, rx + rw - corner_radius, ry + rh - corner_radius),
        ];

        // Save corner pixels outside the rounded curve
        let mut saved: Vec<(usize, u32)> = Vec::new();
        for &(x0, y0, x1, y1, cx, cy) in &corners {
            for py in y0..y1 {
                for px in x0..x1 {
                    let dx = px as isize - cx as isize;
                    let dy = py as isize - cy as isize;
                    if dx * dx + dy * dy > r2 {
                        let idx = py * self.width + px;
                        if idx < self.framebuffer_raw.len() {
                            saved.push((idx, self.framebuffer_raw[idx]));
                        }
                    }
                }
            }
        }

        self.blur_region(rx, ry, rw, rh, blur_radius);

        // Restore saved corner pixels
        for &(idx, val) in &saved {
            if idx < self.framebuffer_raw.len() {
                self.framebuffer_raw[idx] = val;
            }
        }
    }

    // ─── Private: AA Implementations ────────────────────────────────────

    #[inline]
    fn blend_pixel(&mut self, x: isize, y: isize, color: &crate::color::Color, alpha: f32) {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return;
        }
        let a = (alpha * 255.0) as u32;
        if a == 0 {
            return;
        }
        let idx = y as usize * self.width + x as usize;
        if a >= 255 {
            self.framebuffer_raw[idx] = color.as_u32();
            return;
        }
        let bg = self.framebuffer_raw[idx];
        let bg_r = (bg >> 16) & 0xFF;
        let bg_g = (bg >> 8) & 0xFF;
        let bg_b = bg & 0xFF;
        let inv = 255 - a;
        let r = (color.r as u32 * a + bg_r * inv) / 255;
        let g = (color.g as u32 * a + bg_g * inv) / 255;
        let b = (color.b as u32 * a + bg_b * inv) / 255;
        self.framebuffer_raw[idx] = (r << 16) | (g << 8) | b;
    }

    #[inline]
    fn set_pixel_safe(&mut self, x: isize, y: isize, value: u32) {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.framebuffer_raw[y as usize * self.width + x as usize] = value;
        }
    }

    /// Line with AA: uses Wu's algorithm for thin lines, SDF for thick
    fn draw_line_aa_impl(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: f32, color: &crate::color::Color) {
        if thickness <= 1.5 && self.aa > 0.0 {
            self.draw_line_wu(x0, y0, x1, y1, color);
        } else if self.aa > 0.0 {
            self.draw_line_thick_sdf(x0, y0, x1, y1, thickness, color);
        } else {
            self.draw_line_aliased(x0 as isize, y0 as isize, x1 as isize, y1 as isize, thickness as usize, color);
        }
    }

    /// Xiaolin Wu's line algorithm
    fn draw_line_wu(&mut self, mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32, color: &crate::color::Color) {
        let steep = (y1 - y0).abs() > (x1 - x0).abs();
        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let gradient = if dx < 0.001 { 1.0 } else { dy / dx };

        let xend = x0.round();
        let yend = y0 + gradient * (xend - x0);
        let xgap = 1.0 - (x0 + 0.5).fract();
        let xpxl1 = xend as isize;
        let ypxl1 = yend.floor() as isize;
        if steep {
            self.blend_pixel(ypxl1, xpxl1, color, (1.0 - yend.fract()) * xgap);
            self.blend_pixel(ypxl1 + 1, xpxl1, color, yend.fract() * xgap);
        } else {
            self.blend_pixel(xpxl1, ypxl1, color, (1.0 - yend.fract()) * xgap);
            self.blend_pixel(xpxl1, ypxl1 + 1, color, yend.fract() * xgap);
        }
        let mut intery = yend + gradient;

        let xend2 = x1.round();
        let yend2 = y1 + gradient * (xend2 - x1);
        let xgap2 = (x1 + 0.5).fract();
        let xpxl2 = xend2 as isize;
        let ypxl2 = yend2.floor() as isize;
        if steep {
            self.blend_pixel(ypxl2, xpxl2, color, (1.0 - yend2.fract()) * xgap2);
            self.blend_pixel(ypxl2 + 1, xpxl2, color, yend2.fract() * xgap2);
        } else {
            self.blend_pixel(xpxl2, ypxl2, color, (1.0 - yend2.fract()) * xgap2);
            self.blend_pixel(xpxl2, ypxl2 + 1, color, yend2.fract() * xgap2);
        }

        for x in (xpxl1 + 1)..xpxl2 {
            let ipart = intery.floor() as isize;
            let fpart = intery.fract();
            if steep {
                self.blend_pixel(ipart, x, color, 1.0 - fpart);
                self.blend_pixel(ipart + 1, x, color, fpart);
            } else {
                self.blend_pixel(x, ipart, color, 1.0 - fpart);
                self.blend_pixel(x, ipart + 1, color, fpart);
            }
            intery += gradient;
        }
    }

    /// Thick line using distance-to-segment SDF
    fn draw_line_thick_sdf(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: f32, color: &crate::color::Color) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len_sq = dx * dx + dy * dy;
        if len_sq < 0.001 {
            return;
        }
        let half = thickness / 2.0;
        let aa = self.aa;

        let min_x = (x0.min(x1) - half - aa).floor() as isize;
        let max_x = (x0.max(x1) + half + aa).ceil() as isize;
        let min_y = (y0.min(y1) - half - aa).floor() as isize;
        let max_y = (y0.max(y1) + half + aa).ceil() as isize;

        let min_x = min_x.max(0);
        let max_x = max_x.min(self.width as isize - 1);
        let min_y = min_y.max(0);
        let max_y = max_y.min(self.height as isize - 1);

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let fx = px as f32 + 0.5;
                let fy = py as f32 + 0.5;

                let t = ((fx - x0) * dx + (fy - y0) * dy) / len_sq;
                let t = t.clamp(0.0, 1.0);
                let closest_x = x0 + t * dx;
                let closest_y = y0 + t * dy;
                let dist = ((fx - closest_x).powi(2) + (fy - closest_y).powi(2)).sqrt();

                let coverage = ((half + aa * 0.5 - dist) / aa).clamp(0.0, 1.0);
                if coverage > 0.0 {
                    self.blend_pixel(px, py, color, coverage);
                }
            }
        }
    }

    /// Bresenham line (no AA)
    fn draw_line_aliased(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, th: usize, color: &crate::color::Color) {
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
            for oy in -half..=half {
                for ox in -half..=half {
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

    /// Filled circle with SDF AA
    fn draw_circle_f_sdf(&mut self, cx: f32, cy: f32, radius: f32, color: &crate::color::Color) {
        let aa = self.aa;
        let min_x = ((cx - radius - aa).floor() as isize).max(0);
        let max_x = ((cx + radius + aa).ceil() as isize).min(self.width as isize - 1);
        let min_y = ((cy - radius - aa).floor() as isize).max(0);
        let max_y = ((cy + radius + aa).ceil() as isize).min(self.height as isize - 1);

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let fx = px as f32 + 0.5;
                let fy = py as f32 + 0.5;
                let dist = ((fx - cx).powi(2) + (fy - cy).powi(2)).sqrt() - radius;
                let coverage = ((aa * 0.5 - dist) / aa).clamp(0.0, 1.0);
                if coverage > 0.0 {
                    self.blend_pixel(px, py, color, coverage);
                }
            }
        }
    }

    /// Hollow circle (ring) with SDF AA
    fn draw_circle_sdf(&mut self, cx: f32, cy: f32, radius: f32, thickness: f32, color: &crate::color::Color) {
        let aa = self.aa;
        let half = thickness / 2.0;
        let min_x = ((cx - radius - half - aa).floor() as isize).max(0);
        let max_x = ((cx + radius + half + aa).ceil() as isize).min(self.width as isize - 1);
        let min_y = ((cy - radius - half - aa).floor() as isize).max(0);
        let max_y = ((cy + radius + half + aa).ceil() as isize).min(self.height as isize - 1);

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let fx = px as f32 + 0.5;
                let fy = py as f32 + 0.5;
                let dist_from_center = ((fx - cx).powi(2) + (fy - cy).powi(2)).sqrt();
                let dist = (dist_from_center - radius).abs() - half;
                let coverage = ((aa * 0.5 - dist) / aa).clamp(0.0, 1.0);
                if coverage > 0.0 {
                    self.blend_pixel(px, py, color, coverage);
                }
            }
        }
    }

    /// Filled circle without AA
    fn draw_circle_f_aliased(&mut self, cx: isize, cy: isize, radius: usize, color: &crate::color::Color) {
        let r = radius as isize;
        let value = color.as_u32();
        for dy in -r..=r {
            let half_w = isqrt_i(r * r - dy * dy);
            let py = cy + dy;
            if py < 0 || py >= self.height as isize { continue; }
            let start_x = (cx - half_w).max(0) as usize;
            let end_x = ((cx + half_w + 1) as usize).min(self.width);
            if start_x >= end_x { continue; }
            let row = py as usize * self.width;
            self.framebuffer_raw[row + start_x..row + end_x].fill(value);
        }
    }

    /// Hollow circle without AA (midpoint algorithm)
    fn draw_circle_aliased(&mut self, cx: isize, cy: isize, radius: usize, color: &crate::color::Color) {
        let value = color.as_u32();
        let mut x = radius as isize;
        let mut y: isize = 0;
        let mut err: isize = 1 - x;

        while x >= y {
            self.set_pixel_safe(cx + x, cy + y, value);
            self.set_pixel_safe(cx - x, cy + y, value);
            self.set_pixel_safe(cx + x, cy - y, value);
            self.set_pixel_safe(cx - x, cy - y, value);
            self.set_pixel_safe(cx + y, cy + x, value);
            self.set_pixel_safe(cx - y, cy + x, value);
            self.set_pixel_safe(cx + y, cy - x, value);
            self.set_pixel_safe(cx - y, cy - x, value);

            y += 1;
            if err < 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x) + 1;
            }
        }
    }

    /// Filled ellipse with SDF AA
    fn draw_ellipse_f_sdf(&mut self, cx: f32, cy: f32, rx: f32, ry: f32, color: &crate::color::Color) {
        let aa = self.aa;
        let min_x = ((cx - rx - aa).floor() as isize).max(0);
        let max_x = ((cx + rx + aa).ceil() as isize).min(self.width as isize - 1);
        let min_y = ((cy - ry - aa).floor() as isize).max(0);
        let max_y = ((cy + ry + aa).ceil() as isize).min(self.height as isize - 1);

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let fx = px as f32 + 0.5;
                let fy = py as f32 + 0.5;
                let nx = (fx - cx) / rx;
                let ny = (fy - cy) / ry;
                let d = (nx * nx + ny * ny).sqrt();
                let r_min = rx.min(ry);
                let dist = (d - 1.0) * r_min;
                let coverage = ((aa * 0.5 - dist) / aa).clamp(0.0, 1.0);
                if coverage > 0.0 {
                    self.blend_pixel(px, py, color, coverage);
                }
            }
        }
    }

    /// Hollow ellipse with SDF AA
    fn draw_ellipse_sdf(&mut self, cx: f32, cy: f32, rx: f32, ry: f32, thickness: f32, color: &crate::color::Color) {
        let aa = self.aa;
        let half = thickness / 2.0;
        let min_x = ((cx - rx - half - aa).floor() as isize).max(0);
        let max_x = ((cx + rx + half + aa).ceil() as isize).min(self.width as isize - 1);
        let min_y = ((cy - ry - half - aa).floor() as isize).max(0);
        let max_y = ((cy + ry + half + aa).ceil() as isize).min(self.height as isize - 1);

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let fx = px as f32 + 0.5;
                let fy = py as f32 + 0.5;
                let nx = (fx - cx) / rx;
                let ny = (fy - cy) / ry;
                let d = (nx * nx + ny * ny).sqrt();
                let r_min = rx.min(ry);
                let dist = ((d - 1.0) * r_min).abs() - half;
                let coverage = ((aa * 0.5 - dist) / aa).clamp(0.0, 1.0);
                if coverage > 0.0 {
                    self.blend_pixel(px, py, color, coverage);
                }
            }
        }
    }

    /// Filled ellipse without AA
    fn draw_ellipse_f_aliased(&mut self, cx: isize, cy: isize, rx: usize, ry: usize, color: &crate::color::Color) {
        let value = color.as_u32();
        let rx_i = rx as isize;
        let ry_i = ry as isize;
        for dy in -ry_i..=ry_i {
            let py = cy + dy;
            if py < 0 || py >= self.height as isize { continue; }
            let half_w = (rx_i as f32 * (1.0 - (dy * dy) as f32 / (ry_i * ry_i) as f32).sqrt()) as isize;
            let start_x = (cx - half_w).max(0) as usize;
            let end_x = ((cx + half_w + 1) as usize).min(self.width);
            if start_x >= end_x { continue; }
            let row = py as usize * self.width;
            self.framebuffer_raw[row + start_x..row + end_x].fill(value);
        }
    }

    /// Hollow ellipse without AA (parametric)
    fn draw_ellipse_aliased(&mut self, cx: isize, cy: isize, rx: usize, ry: usize, color: &crate::color::Color) {
        let value = color.as_u32();
        let a = rx as f32;
        let b = ry as f32;
        let steps = ((rx + ry) * 4).max(64);
        let step_angle = std::f32::consts::TAU / steps as f32;

        for i in 0..steps {
            let angle = i as f32 * step_angle;
            let px = (cx as f32 + a * angle.cos()) as isize;
            let py = (cy as f32 + b * angle.sin()) as isize;
            self.set_pixel_safe(px, py, value);
        }
    }

    /// Filled rounded rect with SDF AA
    fn draw_rounded_rect_f_sdf(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: &crate::color::Color) {
        let r = radius.min(w / 2.0).min(h / 2.0);
        let aa = self.aa;
        let min_px = ((x - aa).floor() as isize).max(0);
        let max_px = ((x + w + aa).ceil() as isize).min(self.width as isize - 1);
        let min_py = ((y - aa).floor() as isize).max(0);
        let max_py = ((y + h + aa).ceil() as isize).min(self.height as isize - 1);

        let value = color.as_u32();

        for py in min_py..=max_py {
            for px in min_px..=max_px {
                let pfx = px as f32 + 0.5;
                let pfy = py as f32 + 0.5;
                let dist = rounded_rect_sdf(pfx, pfy, x, y, w, h, r);
                let coverage = ((aa * 0.5 - dist) / aa).clamp(0.0, 1.0);
                if coverage >= 1.0 {
                    self.framebuffer_raw[py as usize * self.width + px as usize] = value;
                } else if coverage > 0.0 {
                    self.blend_pixel(px, py, color, coverage);
                }
            }
        }
    }

    /// Hollow rounded rect with SDF AA
    fn draw_rounded_rect_sdf(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, thickness: f32, color: &crate::color::Color) {
        let r = radius.min(w / 2.0).min(h / 2.0);
        let aa = self.aa;
        let half = thickness / 2.0;
        let min_px = ((x - half - aa).floor() as isize).max(0);
        let max_px = ((x + w + half + aa).ceil() as isize).min(self.width as isize - 1);
        let min_py = ((y - half - aa).floor() as isize).max(0);
        let max_py = ((y + h + half + aa).ceil() as isize).min(self.height as isize - 1);

        for py in min_py..=max_py {
            for px in min_px..=max_px {
                let pfx = px as f32 + 0.5;
                let pfy = py as f32 + 0.5;
                let dist = rounded_rect_sdf(pfx, pfy, x, y, w, h, r).abs() - half;
                let coverage = ((aa * 0.5 - dist) / aa).clamp(0.0, 1.0);
                if coverage > 0.0 {
                    self.blend_pixel(px, py, color, coverage);
                }
            }
        }
    }

    /// Filled rounded rect without AA
    fn draw_rounded_rect_f_aliased(&mut self, x: usize, y: usize, w: usize, h: usize, radius: usize, color: &crate::color::Color) {
        let r = radius.min(w / 2).min(h / 2);
        let value = color.as_u32();

        for dy in 0..h {
            let (row_start, row_end) = if dy < r {
                let cy = r - dy;
                let dx = r - isqrt(r * r - cy * cy);
                (x + dx, x + w - dx)
            } else if dy >= h - r {
                let cy = dy - (h - 1 - r);
                let dx = r - isqrt(r * r - cy * cy);
                (x + dx, x + w - dx)
            } else {
                (x, x + w)
            };

            let start = (y + dy) * self.width + row_start;
            let len = row_end - row_start;
            self.framebuffer_raw[start..start + len].fill(value);
        }
    }

    /// Hollow rounded rect without AA
    fn draw_rounded_rect_aliased(&mut self, x: usize, y: usize, w: usize, h: usize, radius: usize, color: &crate::color::Color) {
        let r = radius.min(w / 2).min(h / 2);
        let value = color.as_u32();

        for px in (x + r)..(x + w - r) {
            self.framebuffer_raw[y * self.width + px] = value;
            self.framebuffer_raw[(y + h - 1) * self.width + px] = value;
        }
        for py in (y + r)..(y + h - r) {
            self.framebuffer_raw[py * self.width + x] = value;
            self.framebuffer_raw[py * self.width + x + w - 1] = value;
        }

        let cx_tl = x + r;
        let cy_tl = y + r;
        let cx_tr = x + w - 1 - r;
        let cy_tr = y + r;
        let cx_bl = x + r;
        let cy_bl = y + h - 1 - r;
        let cx_br = x + w - 1 - r;
        let cy_br = y + h - 1 - r;

        let mut ix = r as isize;
        let mut iy: isize = 0;
        let mut err: isize = 1 - ix;

        while ix >= iy {
            self.set_pixel_safe(cx_tl as isize - ix, cy_tl as isize - iy, value);
            self.set_pixel_safe(cx_tl as isize - iy, cy_tl as isize - ix, value);
            self.set_pixel_safe(cx_tr as isize + ix, cy_tr as isize - iy, value);
            self.set_pixel_safe(cx_tr as isize + iy, cy_tr as isize - ix, value);
            self.set_pixel_safe(cx_bl as isize - ix, cy_bl as isize + iy, value);
            self.set_pixel_safe(cx_bl as isize - iy, cy_bl as isize + ix, value);
            self.set_pixel_safe(cx_br as isize + ix, cy_br as isize + iy, value);
            self.set_pixel_safe(cx_br as isize + iy, cy_br as isize + ix, value);

            iy += 1;
            if err < 0 {
                err += 2 * iy + 1;
            } else {
                ix -= 1;
                err += 2 * (iy - ix) + 1;
            }
        }
    }
}

pub struct MouseState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub rmb_clicked: bool,
    pub lmb_clicked: bool
}

/// Integer square root (usize)
#[inline]
fn isqrt(n: usize) -> usize {
    (n as f32).sqrt() as usize
}

/// Integer square root (isize)
#[inline]
fn isqrt_i(n: isize) -> isize {
    (n as f32).sqrt() as isize
}

/// Signed distance field for a rounded rectangle.
/// Returns negative inside, positive outside, zero on the boundary.
#[inline]
fn rounded_rect_sdf(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32, r: f32) -> f32 {
    let cx = x + w / 2.0;
    let cy = y + h / 2.0;
    let dx = (px - cx).abs() - (w / 2.0 - r);
    let dy = (py - cy).abs() - (h / 2.0 - r);
    let outside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt();
    let inside = dx.max(dy).min(0.0);
    outside + inside - r
}
