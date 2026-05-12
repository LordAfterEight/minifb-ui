#[derive(Default)]
pub struct Button {
    pub label: Option<crate::ui::text::Text>,
    pub label_size: f32,
    pub text_alignment: Alignment,

    /// The Button's horizontal position in pixels
    pub pos_x: usize,
    /// The Button's vertical position in pixels
    pub pos_y: usize,

    /// The Button's width in pixels
    pub width: usize,
    /// The Button's height in pixels
    pub height: usize,

    /// The Button's border size for when the Button is idle
    pub border_size_idle: usize,
    /// The Button's border size for when the Button is hovered
    pub border_size_hovered: usize,
    /// The Button's border size for when the Button is clicked
    pub border_size_clicked: usize,

    /// Button-wide shadow size
    pub shadow_size_idle: usize,
    /// Button-wide shadow size when hoverd
    pub shadow_size_hovered: usize,
    /// Button-wide shadow size when clicked
    pub shadow_size_clicked: usize,
    /// How intense the shadows should be
    pub shadow_intensity_idle: u8,
    /// How intense the shadows should be when hovered
    pub shadow_intensity_hovered: u8,
    /// How intense the shadows should be when clicked
    pub shadow_intensity_clicked: u8,

    /// The color of the Button's label
    pub label_col_idle: crate::color::Color,
    /// The color of the Button's border
    pub border_col_idle: crate::color::Color,
    /// The color of the Button's background
    pub bg_col_idle: crate::color::Color,

    /// The color of the Button's label when hovered
    pub label_col_hovered: crate::color::Color,
    /// The color of the Button's border when hovered
    pub border_col_hovered: crate::color::Color,
    /// The color of the Button's background when hovered
    pub bg_col_hovered: crate::color::Color,

    /// The color of the Button's label when clicked
    pub label_col_clicked: crate::color::Color,
    /// The color of the Button's border when clicked
    pub border_col_clicked: crate::color::Color,
    /// The color of the Button's background when clicked
    pub bg_col_clicked: crate::color::Color,

    /// Whether the Button is a push button or a toggle button
    pub button_type: ButtonType,

    /// The current state of the Button. Can be Idle, Hovered or Clicked
    pub state: ButtonState,

    /// Whether the toggle button is currently toggled on (only relevant for ButtonType::Toggle)
    pub toggled: bool,

    /// Corner radius for rounded edges
    pub radius: usize,
}

impl Button {
    /// Sets the text to be displayed inside the button
    pub fn label(mut self, text: &str, font: crate::ttf::Font, size: f32) -> Self {
        self.label = Some(crate::ui::text::Text::new(text, font));
        self.label_size = size;
        self
    }

    /// Determines whether the button label is left-aligned, right-aligned or centered
    pub fn label_alignment(mut self, alignment: Alignment) -> Self {
        self.text_alignment = alignment;
        self
    }

    /// Sets the position of the button inside the window in pixel coordinates
    pub fn position(mut self, x: usize, y: usize) -> Self {
        self.pos_x = x;
        self.pos_y = y;
        self
    }

    /// Sets the size of the button in pixels
    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the thickness of the Button's borders in pixels
    pub fn border(mut self, size: usize) -> Self {
        self.border_size_idle = size;
        self.border_size_hovered = size;
        self.border_size_clicked = size;
        self
    }

    /// Determines whether to draw shadows and how large and intense they should be
    /// Sets values for idle, hovered and clicked state
    pub fn shadow(mut self, size: usize, intensity: u8) -> Self {
        self.shadow_size_idle = size;
        self.shadow_size_hovered = size;
        self.shadow_size_clicked = size;
        self.shadow_intensity_idle = intensity;
        self.shadow_intensity_hovered = intensity;
        self.shadow_intensity_clicked = intensity;
        self
    }

    /// Specifically sets the idle shadow size and intensity
    pub fn idle_shadow(mut self, size: usize, intensity: u8) -> Self {
        self.shadow_size_idle = size;
        self.shadow_intensity_idle = intensity;
        self
    }

    /// Specifically sets the shadow size and intensity for when the Button is hovered
    pub fn hover_shadow(mut self, size: usize, intensity: u8) -> Self {
        self.shadow_size_hovered = size;
        self.shadow_intensity_hovered = intensity;
        self
    }

    /// Specifically sets the shadow size and intensity for when the Button is clicked
    pub fn click_shadow(mut self, size: usize, intensity: u8) -> Self {
        self.shadow_size_clicked = size;
        self.shadow_intensity_clicked = intensity;
        self
    }

    /// Sets the label text color for idle, hovered and clicked state
    pub fn label_color(mut self, color: crate::color::Color) -> Self {
        self.label_col_hovered = color.clone();
        self.label_col_clicked = color.clone();
        self.label_col_idle = color;
        self
    }

    /// Specifically sets the label color for when the Button is idle
    pub fn idle_label_col(mut self, color: crate::color::Color) -> Self {
        self.label_col_idle = color;
        self
    }

    /// Specifically sets the label color for when the Button is hovered
    pub fn hover_label_col(mut self, color: crate::color::Color) -> Self {
        self.label_col_hovered = color;
        self
    }

    /// Specifically sets the label color for when the Button is clicked
    pub fn click_label_col(mut self, color: crate::color::Color) -> Self {
        self.label_col_clicked = color;
        self
    }

    /// Sets the border color for idle, hovered and clicked state
    pub fn border_color(mut self, color: crate::color::Color) -> Self {
        self.border_col_hovered = color.clone();
        self.border_col_clicked = color.clone();
        self.border_col_idle = color;
        self
    }

    /// Sets the button's background color for idle, hovered and clicked state
    pub fn background(mut self, color: crate::color::Color) -> Self {
        self.bg_col_hovered = color.clone();
        self.bg_col_clicked = color.clone();
        self.bg_col_idle = color;
        self
    }

    /// Specifically sets the background color for when the Button is idle
    pub fn idle_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col_idle = color;
        self
    }

    /// Specifically sets the background color for when the Button is hovered
    pub fn hover_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col_hovered = color;
        self
    }

    /// Specifically sets the background color for when the Button is clicked
    pub fn click_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col_clicked = color;
        self
    }

    /// Determines whether the button is a push button or toggle button
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    /// Sets the corner radius for rounded edges
    pub fn radius(mut self, radius: usize) -> Self {
        self.radius = radius;
        self
    }

    fn draw_shadow(&self, window: &mut crate::window::Window) {
        let shadow_depth;
        let shadow_size;
        let border_size;

        match self.state {
            ButtonState::Idle => {
                shadow_depth = self.shadow_intensity_idle;
                shadow_size = self.shadow_size_idle;
                border_size = self.border_size_idle;
            },
            ButtonState::Hovered => {
                shadow_depth = self.shadow_intensity_hovered;
                shadow_size = self.shadow_size_hovered;
                border_size = self.border_size_hovered;
            },
            ButtonState::Clicked => {
                shadow_depth = self.shadow_intensity_clicked;
                shadow_size = self.shadow_size_clicked;
                border_size = self.border_size_clicked;
            }
        }

        // Draw inset shadow as concentric rounded-rect rings
        let inner_x = self.pos_x + border_size;
        let inner_y = self.pos_y + border_size;
        let inner_w = self.width.saturating_sub(border_size * 2);
        let inner_h = self.height.saturating_sub(border_size * 2);
        let inner_r = self.radius.saturating_sub(border_size) as f32;

        let cx = inner_x as f32 + inner_w as f32 / 2.0;
        let cy = inner_y as f32 + inner_h as f32 / 2.0;
        let hw = inner_w as f32 / 2.0;
        let hh = inner_h as f32 / 2.0;

        for py in inner_y..inner_y + inner_h {
            for px in inner_x..inner_x + inner_w {
                // Signed distance from the inner edge (negative = inside)
                let pfx = px as f32 + 0.5;
                let pfy = py as f32 + 0.5;
                let dx = (pfx - cx).abs() - (hw - inner_r);
                let dy = (pfy - cy).abs() - (hh - inner_r);
                let outside = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt();
                let inside = dx.max(dy).min(0.0);
                let dist = outside + inside - inner_r; // negative inside

                // dist is negative inside; the edge is at dist=0
                // Shadow starts at the edge and fades inward
                let depth = -dist; // positive near edge, grows toward center
                if depth > 0.0 && depth <= shadow_size as f32 {
                    let t = 1.0 - (depth / shadow_size as f32);
                    let blend = (shadow_depth as f32 * t) as i32;
                    if blend > 0 {
                        darken_pixel(window, px, py, blend);
                    }
                }
            }
        }
    }

    /// Internal helper
    fn draw_button(&self, window: &mut crate::window::Window) {
        let bg_col: &crate::color::Color;
        let border_col: &crate::color::Color;
        let label_col: &crate::color::Color;
        let border_size;

        match self.state {
            ButtonState::Idle => {
                bg_col = &self.bg_col_idle;
                border_col = &self.border_col_idle;
                label_col = &self.label_col_idle;
                border_size = self.border_size_idle;
            }
            ButtonState::Hovered =>{
                bg_col = &self.bg_col_hovered;
                border_col = &self.border_col_hovered;
                label_col = &self.label_col_hovered;
                border_size = self.border_size_hovered;
            },
            ButtonState::Clicked => {
                bg_col = &self.bg_col_clicked;
                border_col = &self.border_col_clicked;
                label_col = &self.label_col_clicked;
                border_size = self.border_size_clicked;
            },
        }

        window.draw_rect_f(
            self.pos_x,
            self.pos_y,
            self.width,
            self.height,
            self.radius,
            bg_col,
            0,
        );

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

        if let Some(label) = &self.label {
            let lm = label.font.font.horizontal_line_metrics(self.label_size).unwrap();

            let y_pos = (self.pos_y as f32 + (self.height as f32 / 2.0) - (lm.ascent / 2.0)
                + (lm.descent / 3.0))
                .max(0.0) as usize;

            let text_width = label.get_width_precise(self.label_size);

            match self.text_alignment {
                Alignment::Left => {
                    window.draw_text(
                        self.pos_x + border_size + 4,
                        y_pos,
                        &label,
                        self.label_size,
                        label_col,
                    );
                }
                Alignment::Right => {
                    let x = (self.pos_x + self.width) as f32 - text_width - 4.0;
                    window.draw_text(
                        x.max(0.0) as usize,
                        y_pos,
                        &label,
                        self.label_size,
                        label_col,
                    );
                }
                Alignment::Center => {
                    let x = self.pos_x as f32 + (self.width as f32 / 2.0) - (text_width / 2.0);
                    window.draw_text(
                        x.max(0.0) as usize,
                        y_pos,
                        &label,
                        self.label_size,
                        label_col,
                    );
                }
            }
        }

    }

    pub fn is_hovered(&self, window: &crate::window::Window) -> bool {
        let state = window.get_mouse_state();
        (state.pos_x as usize) > self.pos_x
            && (state.pos_y as usize) > self.pos_y
            && (state.pos_x as usize) < self.pos_x + self.width
            && (state.pos_y as usize) < self.pos_y + self.height
    }

    pub fn is_left_clicked(&self, window: &crate::window::Window) -> bool {
        let state = window.get_mouse_state();
        self.is_hovered(window) && state.lmb_clicked
    }

    pub fn is_right_clicked(&self, window: &crate::window::Window) -> bool {
        let state = window.get_mouse_state();
        self.is_hovered(window) && state.rmb_clicked
    }

    fn update(&mut self, window: &mut crate::window::Window) {
        let mouse = window.get_mouse_state();
        let hovered = (mouse.pos_x as usize) > self.pos_x
            && (mouse.pos_y as usize) > self.pos_y
            && (mouse.pos_x as usize) < self.pos_x + self.width
            && (mouse.pos_y as usize) < self.pos_y + self.height;
        let clicked = hovered && (mouse.lmb_clicked || mouse.rmb_clicked);

        match self.button_type {
            ButtonType::Push => {
                self.state = if clicked {
                    ButtonState::Clicked
                } else if hovered {
                    ButtonState::Hovered
                } else {
                    ButtonState::Idle
                };
            }
            ButtonType::Toggle => {
                if clicked && !self.toggled {
                    self.toggled = true;
                } else if clicked && self.toggled {
                    self.toggled = false;
                }
                self.state = if self.toggled {
                    ButtonState::Clicked
                } else if hovered {
                    ButtonState::Hovered
                } else {
                    ButtonState::Idle
                };
            }
        }
    }

    /// Draws the button to a window
    pub fn draw(&mut self, window: &mut crate::window::Window) {
        self.update(window);
        self.draw_button(window);
        self.draw_shadow(window);
    }
}

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Push,
    Toggle,
}

#[derive(Default)]
pub enum Alignment {
    Left,
    Right,
    #[default]
    Center,
}

#[derive(Default)]
pub enum ButtonState {
    #[default]
    Idle,
    Hovered,
    Clicked,
}

fn darken_pixel(window: &mut crate::window::Window, x: usize, y: usize, blend: i32) {
    let existing = window.get_pixel(x, y);
    let er = ((existing >> 16) & 0xFF) as i32;
    let eg = ((existing >> 8) & 0xFF) as i32;
    let eb = ((existing) & 0xFF) as i32;

    let r = (er - blend).clamp(0, 255) as u32;
    let g = (eg - blend).clamp(0, 255) as u32;
    let b = (eb - blend).clamp(0, 255) as u32;

    window.draw_pixel(
        x,
        y,
        &crate::color::Color::from(0xFF000000 | (r << 16) | (g << 8) | b),
    );
}
