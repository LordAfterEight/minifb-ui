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
    /// The color of the Button's border when clicker
    pub border_col_clicked: crate::color::Color,
    /// The color of the Button's background when clicked
    pub bg_col_clicked: crate::color::Color,

    /// Whether the Button is a push button or a toggle button
    pub button_type: ButtonType,

    /// The current state of the Button. Can be Idle, Hovered or Clicked
    pub state: ButtonState,
}

impl Button {
    /// Sets the text to be displayed inside the button
    pub fn label(mut self, text: &str, font: crate::ttf::Font, size: f32) -> Self {
        self.label = Some(crate::ui::text::Text::new(text, font));
        self.label_size = size;
        self
    }

    /// Determines whether the button lable is left-aligned, right-aligned or centered
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

    /// Specifically sets the lable color for when the Button is idle
    pub fn idle_label_col(mut self, color: crate::color::Color) -> Self {
        self.label_col_idle = color;
        self
    }

    /// Specifically sets the lable color for when the Button is hovered
    pub fn hover_label_col(mut self, color: crate::color::Color) -> Self {
        self.label_col_hovered = color;
        self
    }

    /// Specifically sets the lable color for when the Button is clicked
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

    /// Specifically sets the background color for when the Butoon is idle
    pub fn idle_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col_idle = color;
        self
    }

    /// Specifically sets the background color for when the Butoon is hovered
    pub fn hover_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col_hovered = color;
        self
    }

    /// Specifically sets the background color for when the Butoon is clicked
    pub fn click_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col_clicked = color;
        self
    }

    /// Determines whether the button is a push button or toggle button
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
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

        for i in 0..shadow_size {
            let t_num = i as i32;
            let t_den = shadow_size as i32;

            let blend = (shadow_depth as i32) - ((shadow_depth as i32) * t_num) / t_den;

            let x = self.pos_x + border_size + i;
            let y = self.pos_y + border_size + i;
            let w = self.width - (i * 2) - border_size * 2;
            let h = self.height - (i * 2) - border_size * 2;

            for px in x..x + w {
                for py in [y, y + h - 1] {
                    darken_pixel(window, px, py, blend);
                }
            }
            for py in y + 1..y + h - 1 {
                for px in [x, x + w - 1] {
                    darken_pixel(window, px, py, blend);
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
            bg_col,
        );

        for i in 0..border_size {
            window.draw_rect(
                self.pos_x + i,
                self.pos_y + i,
                self.width - i * 2,
                self.height - i * 2,
                border_col,
            );
        }

        if let Some(label) = &self.label {
            let lm = label.font.font.horizontal_line_metrics(16.0).unwrap();

            let y_pos = (self.pos_y as f32 + (self.height as f32 / 2.0) - (lm.ascent / 2.0)
                + (lm.descent / 3.0))
                .max(0.0) as usize;

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
                    let offset: usize = label
                        .text
                        .chars()
                        .map(|c| {
                            label
                                .font
                                .font
                                .metrics(c, self.label_size)
                                .advance_width as usize
                        })
                        .sum();
                    window.draw_text(
                        (self.pos_x + self.width)
                            .saturating_sub(offset)
                            .saturating_sub(4),
                        y_pos,
                        &label,
                        self.label_size,
                        label_col,
                    );
                }
                Alignment::Center => {
                    let offset: usize = label
                        .text
                        .chars()
                        .map(|c| {
                            label
                                .font
                                .font
                                .metrics(c, self.label_size)
                                .advance_width as usize
                        })
                        .sum();
                    window.draw_text(
                        (self.pos_x + self.width / 2).saturating_sub(offset / 2),
                        y_pos,
                        &label,
                        self.label_size,
                        label_col,
                    );
                }
            }
        }

    }

    pub fn is_hovered(&mut self, window: &crate::window::Window) -> bool {
        let state = window.get_mouse_state();
        if (state.pos_x as usize) > self.pos_x
            && (state.pos_y as usize) > self.pos_y
            && (state.pos_x as usize) < self.pos_x + self.width
            && (state.pos_y as usize) < self.pos_y + self.height
        {
            self.state = ButtonState::Hovered;
            true
        } else {
            self.state = ButtonState::Idle;
            false
        }
    }

    pub fn is_left_clicked(&mut self, window: &crate::window::Window) -> bool {
        let state = window.get_mouse_state();
        if self.is_hovered(window) && state.lmb_clicked {
            self.state = ButtonState::Clicked;
            return true
        }
        false
    }

    pub fn is_right_clicked(&mut self, window: &crate::window::Window) -> bool {
        let state = window.get_mouse_state();
        if self.is_hovered(window) && state.rmb_clicked {
            self.state = ButtonState::Clicked;
            return true
        }
        false
    }

    fn update(&mut self, window: &mut crate::window::Window) {
        match self.is_hovered(window) {
            false => self.state = ButtonState::Idle,
            true => match self.is_left_clicked(window) || self.is_right_clicked(window) {
                false => self.state = ButtonState::Hovered,
                true => self.state = ButtonState::Clicked
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
