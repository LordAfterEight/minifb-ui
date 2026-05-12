use crate::Key;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};

pub struct TextInput {
    pub text: String,
    pub font: Option<crate::ttf::Font>,
    pub font_size: f32,

    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub height: usize,

    pub bg_col_idle: crate::color::Color,
    pub border_col_idle: crate::color::Color,
    pub text_col_idle: crate::color::Color,
    pub border_size_idle: usize,

    pub bg_col_editing: crate::color::Color,
    pub border_col_editing: crate::color::Color,
    pub text_col_editing: crate::color::Color,
    pub border_size_editing: usize,

    pub cursor_col: crate::color::Color,
    pub cursor_width: usize,

    /// Corner radius for rounded edges
    pub radius: usize,

    pub state: TextInputState,
    pub cursor_pos: usize,
    pub scroll_offset: f32,
    lmb_was_down: bool,
}

#[derive(Default, PartialEq)]
pub enum TextInputState {
    #[default]
    Idle,
    Editing,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            text: String::new(),
            font: None,
            font_size: 16.0,
            pos_x: 0,
            pos_y: 0,
            width: 200,
            height: 24,
            bg_col_idle: crate::color::Color::new(30, 30, 30),
            border_col_idle: crate::color::Color::new(100, 100, 100),
            text_col_idle: crate::color::Color::new(200, 200, 200),
            border_size_idle: 1,
            bg_col_editing: crate::color::Color::new(40, 40, 40),
            border_col_editing: crate::color::Color::new(100, 160, 255),
            text_col_editing: crate::color::Color::new(255, 255, 255),
            border_size_editing: 2,
            cursor_col: crate::color::Color::new(255, 255, 255),
            cursor_width: 2,
            radius: 0,
            state: TextInputState::Idle,
            cursor_pos: 0,
            scroll_offset: 0.0,
            lmb_was_down: false,
        }
    }
}

impl TextInput {
    pub fn font(mut self, font: crate::ttf::Font, size: f32) -> Self {
        self.font = Some(font);
        self.font_size = size;
        self
    }

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

    pub fn placeholder(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self.cursor_pos = text.len();
        self
    }

    pub fn background(mut self, color: crate::color::Color) -> Self {
        self.bg_col_editing = color.clone();
        self.bg_col_idle = color;
        self
    }

    pub fn idle_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col_idle = color;
        self
    }

    pub fn editing_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col_editing = color;
        self
    }

    pub fn border_color(mut self, color: crate::color::Color) -> Self {
        self.border_col_editing = color.clone();
        self.border_col_idle = color;
        self
    }

    pub fn idle_border_col(mut self, color: crate::color::Color) -> Self {
        self.border_col_idle = color;
        self
    }

    pub fn editing_border_col(mut self, color: crate::color::Color) -> Self {
        self.border_col_editing = color;
        self
    }

    pub fn border(mut self, size: usize) -> Self {
        self.border_size_idle = size;
        self.border_size_editing = size;
        self
    }

    pub fn text_color(mut self, color: crate::color::Color) -> Self {
        self.text_col_editing = color.clone();
        self.text_col_idle = color;
        self
    }

    pub fn idle_text_col(mut self, color: crate::color::Color) -> Self {
        self.text_col_idle = color;
        self
    }

    pub fn editing_text_col(mut self, color: crate::color::Color) -> Self {
        self.text_col_editing = color;
        self
    }

    pub fn cursor_color(mut self, color: crate::color::Color) -> Self {
        self.cursor_col = color;
        self
    }

    /// Sets the corner radius for rounded edges
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }

    /// Returns the current text content
    pub fn value(&self) -> &str {
        &self.text
    }

    /// Returns whether the input is currently being edited
    pub fn is_editing(&self) -> bool {
        self.state == TextInputState::Editing
    }

    /// Uses fontdue Layout to compute the x offset (in pixels) for each glyph,
    /// returning the x position where character at `index` starts.
    /// If index == text.len(), returns the position after the last glyph.
    fn cursor_x_offset(&self, font: &crate::ttf::Font, index: usize) -> f32 {
        if self.text.is_empty() || index == 0 {
            return 0.0;
        }

        let fonts = font.as_slice();
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: 0.0,
            y: 0.0,
            ..Default::default()
        });
        layout.append(&fonts, &TextStyle::new(&self.text, self.font_size, 0));

        let glyphs = layout.glyphs();

        // Map cursor_pos (byte index) to glyph index
        let mut byte_offset = 0;
        let mut glyph_index = 0;
        for (i, c) in self.text.chars().enumerate() {
            if byte_offset >= index {
                glyph_index = i;
                break;
            }
            byte_offset += c.len_utf8();
            glyph_index = i + 1;
        }

        if glyph_index >= glyphs.len() {
            // Cursor is at the end: use last glyph's x + its advance width
            if let Some(last) = glyphs.last() {
                let metrics = font.font.metrics(
                    self.text.chars().last().unwrap(),
                    self.font_size,
                );
                return last.x + metrics.advance_width;
            }
            return 0.0;
        }

        glyphs[glyph_index].x
    }

    fn update(&mut self, window: &mut crate::window::Window) {
        let mouse = window.get_mouse_state();
        let lmb_down = mouse.lmb_clicked;
        let click_edge = lmb_down && !self.lmb_was_down;

        let mx = mouse.pos_x as usize;
        let my = mouse.pos_y as usize;
        let in_bounds = mx >= self.pos_x
            && mx < self.pos_x + self.width
            && my >= self.pos_y
            && my < self.pos_y + self.height;

        if click_edge {
            match self.state {
                TextInputState::Idle => {
                    if in_bounds {
                        self.state = TextInputState::Editing;
                        self.cursor_pos = self.text.len();
                    }
                }
                TextInputState::Editing => {
                    self.state = TextInputState::Idle;
                }
            }
        }

        if self.state == TextInputState::Editing {
            // Use OS-level InputCallback for character input.
            // This correctly handles Shift, Caps Lock, dead keys, compose, etc.
            let typed = window.get_typed_chars();
            for c in typed {
                if c >= ' ' && c != '\x7f' {
                    self.text.insert(self.cursor_pos, c);
                    self.cursor_pos += 1;
                }
            }

            // Use get_keys_pressed only for control/navigation keys
            let keys = window.window.get_keys_pressed(crate::KeyRepeat::Yes);
            for key in keys {
                match key {
                    Key::Enter => {
                        self.state = TextInputState::Idle;
                        break;
                    }
                    Key::Escape => {
                        self.state = TextInputState::Idle;
                        break;
                    }
                    Key::Backspace => {
                        if self.cursor_pos > 0 {
                            self.text.remove(self.cursor_pos - 1);
                            self.cursor_pos -= 1;
                        }
                    }
                    Key::Delete => {
                        if self.cursor_pos < self.text.len() {
                            self.text.remove(self.cursor_pos);
                        }
                    }
                    Key::Left => {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                        }
                    }
                    Key::Right => {
                        if self.cursor_pos < self.text.len() {
                            self.cursor_pos += 1;
                        }
                    }
                    Key::Home => {
                        self.cursor_pos = 0;
                    }
                    Key::End => {
                        self.cursor_pos = self.text.len();
                    }
                    _ => {}
                }
            }
        }

        self.lmb_was_down = lmb_down;
    }

    /// Draws and updates the text input
    pub fn draw(&mut self, window: &mut crate::window::Window) {
        self.update(window);

        let (bg_col, border_col, text_col, border_size) = match self.state {
            TextInputState::Idle => (
                &self.bg_col_idle,
                &self.border_col_idle,
                &self.text_col_idle,
                self.border_size_idle,
            ),
            TextInputState::Editing => (
                &self.bg_col_editing,
                &self.border_col_editing,
                &self.text_col_editing,
                self.border_size_editing,
            ),
        };

        // Background
        window.draw_rect_f(self.pos_x, self.pos_y, self.width, self.height, self.radius, bg_col, 0);

        // Border
        for i in 0..border_size {
            window.draw_rect(
                self.pos_x + i,
                self.pos_y + i,
                self.width - i * 2,
                self.height - i * 2,
                self.radius.saturating_sub(i),
                border_col,
            );
        }

        // Text and cursor
        if let Some(font) = &self.font {
            let padding = 4;
            let text_area_x = self.pos_x + border_size + padding;
            let text_area_w = self.width.saturating_sub((border_size + padding) * 2);

            // Compute cursor x offset using layout engine
            let cursor_offset = self.cursor_x_offset(font, self.cursor_pos);

            // Adjust scroll_offset so cursor is always visible
            let cursor_in_view = cursor_offset - self.scroll_offset;
            if cursor_in_view < 0.0 {
                self.scroll_offset = cursor_offset;
            } else if cursor_in_view > text_area_w as f32 {
                self.scroll_offset = cursor_offset - text_area_w as f32;
            }

            // Vertical centering
            let lm = font.font.horizontal_line_metrics(self.font_size).unwrap();
            let text_y = (self.pos_y as f32 + (self.height as f32 / 2.0) - (lm.ascent / 2.0)
                + (lm.descent / 3.0))
                .max(0.0) as usize;

            // Render text with scroll offset using clipped drawing
            let text_render_x = text_area_x as f32 - self.scroll_offset;
            self.draw_text_clipped(
                window,
                text_render_x,
                text_y,
                font,
                text_col,
                text_area_x,
                text_area_x + text_area_w,
            );

            // Cursor
            if self.state == TextInputState::Editing {
                let cursor_screen_x = text_area_x as f32 + cursor_offset - self.scroll_offset;
                let cx = cursor_screen_x as usize;
                // Only draw cursor if it's within the visible area
                if cx >= text_area_x && cx < text_area_x + text_area_w {
                    let cursor_y = self.pos_y + border_size + 2;
                    let cursor_h = self.height.saturating_sub(border_size * 2 + 4);
                    window.draw_rect_f(cx, cursor_y, self.cursor_width, cursor_h, 0, &self.cursor_col, 0);
                }
            }
        }
    }

    /// Renders text clipped to [clip_left, clip_right) horizontally
    fn draw_text_clipped(
        &self,
        window: &mut crate::window::Window,
        x: f32,
        y: usize,
        font: &crate::ttf::Font,
        color: &crate::color::Color,
        clip_left: usize,
        clip_right: usize,
    ) {
        let fonts = font.as_slice();
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x,
            y: y as f32,
            ..Default::default()
        });
        layout.append(&fonts, &TextStyle::new(&self.text, self.font_size, 0));

        let fg_r = color.r as u32;
        let fg_g = color.g as u32;
        let fg_b = color.b as u32;

        for glyph in layout.glyphs() {
            let (metrics, bitmap) = font.font.rasterize_config(glyph.key);

            let glyph_x = glyph.x as i32;
            let glyph_y = glyph.y as i32;

            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let px = glyph_x + col as i32;
                    let py = glyph_y + row as i32;

                    if px < clip_left as i32 || px >= clip_right as i32 {
                        continue;
                    }
                    if py < 0 || py >= window.height as i32 {
                        continue;
                    }

                    let (px, py) = (px as usize, py as usize);

                    let alpha = bitmap[row * metrics.width + col] as u32;
                    if alpha == 0 {
                        continue;
                    }

                    let idx = py * window.width + px;
                    let bg = window.framebuffer_raw[idx];
                    let bg_r = (bg >> 16) & 0xFF;
                    let bg_g = (bg >> 8) & 0xFF;
                    let bg_b = bg & 0xFF;

                    let r = (fg_r * alpha + bg_r * (255 - alpha)) / 255;
                    let g = (fg_g * alpha + bg_g * (255 - alpha)) / 255;
                    let b = (fg_b * alpha + bg_b * (255 - alpha)) / 255;

                    window.framebuffer_raw[idx] = (r << 16) | (g << 8) | b;
                }
            }
        }
    }
}

