#[derive(Default)]
pub struct Button {
    pub label: crate::ui::text::Text,
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

    /// Whether the Button has a border or not
    pub border: bool,
    /// The Button's border size
    pub border_size: usize,

    /// Whether the Button has an inner edge shadow
    pub inner_shadow: bool,
    /// Button-wide shadow size
    pub shadow_size: usize,
    /// Button-wide shadow size when hoverd
    pub shadow_size_hovered: usize,
    /// Button-wide shadow size when clicked
    pub shadow_size_clicked: usize,
    /// How intense the shadows should be
    pub shadow_intensity: u8,
    /// How intense the shadows should be when hovered
    pub shadow_intensity_hovered: u8,
    /// How intense the shadows should be when clicked
    pub shadow_intensity_clicked: u8,

    /// The color of the Button's label
    pub label_col: crate::color::Color,
    /// The color of the Button's border
    pub border_col: crate::color::Color,
    /// The color of the Button's background
    pub bg_col: crate::color::Color,

    /// The color of the Button's label when hovered
    pub label_hovered_col: crate::color::Color,
    /// The color of the Button's border when hovered
    pub border_hovered_col: crate::color::Color,
    /// The color of the Button's background when hovered
    pub bg_hovered_col: crate::color::Color,

    /// The color of the Button's label when clicked
    pub label_clicked_col: crate::color::Color,
    /// The color of the Button's border when clicker
    pub border_clicked_col: crate::color::Color,
    /// The color of the Button's background when clicked
    pub bg_clicked_col: crate::color::Color,

    /// Whether the Button is a push button or a toggle button
    pub button_type: ButtonType,

    /// The current state of the Button. Can be Idle, Hovered or Clicked
    pub state: ButtonState,
}

impl Button {
    /// Sets the text to be displayed inside the button
    pub fn label(mut self, text: &str, font: crate::ttf::Font, size: f32) -> Self {
        self.label = crate::ui::text::Text::new(text, font);
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

    /// Determines whether to draw borders and how thick they should be
    pub fn border(mut self, enable: bool, size: usize) -> Self {
        self.border = enable;
        self.border_size = size;
        self
    }

    /// Determines whether to draw shadows and how large and intense they should be
    /// Sets values for idle, hovered and clicked state
    pub fn shadow(mut self, enable: bool, size: usize, intensity: u8) -> Self {
        self.inner_shadow = enable;
        self.shadow_size = size;
        self.shadow_size_hovered = size;
        self.shadow_size_clicked = size;
        self.shadow_intensity = intensity;
        self.shadow_intensity_hovered = intensity;
        self.shadow_intensity_clicked = intensity;
        self
    }

    /// Specifically sets the idle shadow size and intensity
    pub fn idle_shadow(mut self, size: usize, intensity: u8) -> Self {
        self.shadow_size = size;
        self.shadow_intensity = intensity;
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
        self.label_hovered_col = color.clone();
        self.label_clicked_col = color.clone();
        self.label_col = color;
        self
    }

    /// Specifically sets the lable color for when the Button is idle
    pub fn idle_label_col(mut self, color: crate::color::Color) -> Self {
        self.label_col = color;
        self
    }

    /// Specifically sets the lable color for when the Button is hovered
    pub fn hover_label_col(mut self, color: crate::color::Color) -> Self {
        self.label_hovered_col = color;
        self
    }

    /// Specifically sets the lable color for when the Button is clicked
    pub fn click_label_col(mut self, color: crate::color::Color) -> Self {
        self.label_clicked_col = color;
        self
    }

    /// Sets the border color for idle, hovered and clicked state
    pub fn border_color(mut self, color: crate::color::Color) -> Self {
        self.border_hovered_col = color.clone();
        self.border_clicked_col = color.clone();
        self.border_col = color;
        self
    }

    /// Sets the button's background color for idle, hovered and clicked state
    pub fn background(mut self, color: crate::color::Color) -> Self {
        self.bg_hovered_col = color.clone();
        self.bg_clicked_col = color.clone();
        self.bg_col = color;
        self
    }

    /// Specifically sets the background color for when the Butoon is idle
    pub fn idle_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_col = color;
        self
    }

    /// Specifically sets the background color for when the Butoon is hovered
    pub fn hover_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_hovered_col = color;
        self
    }

    /// Specifically sets the background color for when the Butoon is clicked
    pub fn click_bg(mut self, color: crate::color::Color) -> Self {
        self.bg_clicked_col = color;
        self
    }

    /// Determines whether the button is a push button or toggle button
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    fn draw_shadow(&self, window: &mut crate::window::Window) {
        let shadow_depth = self.shadow_intensity as i32;
        let shadow_size;

        match self.state {
            ButtonState::Idle => shadow_size = self.shadow_size,
            ButtonState::Hovered => shadow_size = self.shadow_size_hovered,
            ButtonState::Clicked => shadow_size = self.shadow_size_clicked
        }

        for i in 0..shadow_size {
            let t_num = i as i32;
            let t_den = shadow_size as i32;

            let blend = shadow_depth - (shadow_depth * t_num) / t_den;

            let x = self.pos_x + self.border_size + i;
            let y = self.pos_y + self.border_size + i;
            let w = self.width - (i * 2) - self.border_size * 2;
            let h = self.height - (i * 2) - self.border_size * 2;

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

        match self.state {
            ButtonState::Idle => {
                bg_col = &self.bg_col;
                border_col = &self.border_col;
                label_col = &self.label_col;
            }
            ButtonState::Hovered =>{
                bg_col = &self.bg_hovered_col;
                border_col = &self.border_hovered_col;
                label_col = &self.label_hovered_col;
            },
            ButtonState::Clicked => {
                bg_col = &self.bg_clicked_col;
                border_col = &self.border_clicked_col;
                label_col = &self.label_clicked_col;
            },
        }

        window.draw_rect_f(
            self.pos_x,
            self.pos_y,
            self.width,
            self.height,
            bg_col,
        );

        for i in 0..self.border_size {
            window.draw_rect(
                self.pos_x + i,
                self.pos_y + i,
                self.width - i * 2,
                self.height - i * 2,
                border_col,
            );
        }

        let lm = self.label.font.font.horizontal_line_metrics(16.0).unwrap();

        let y_pos = (self.pos_y as f32 + (self.height as f32 / 2.0) - (lm.ascent / 2.0)
            + (lm.descent / 3.0))
            .max(0.0) as usize;

        match self.text_alignment {
            Alignment::Left => {
                window.draw_text(
                    self.pos_x + self.border_size + 4,
                    y_pos,
                    &self.label,
                    self.label_size,
                    label_col,
                );
            }
            Alignment::Right => {
                let offset: usize = self
                    .label
                    .text
                    .chars()
                    .map(|c| {
                        self.label
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
                    &self.label,
                    self.label_size,
                    label_col,
                );
            }
            Alignment::Center => {
                let offset: usize = self
                    .label
                    .text
                    .chars()
                    .map(|c| {
                        self.label
                            .font
                            .font
                            .metrics(c, self.label_size)
                            .advance_width as usize
                    })
                    .sum();
                window.draw_text(
                    (self.pos_x + self.width / 2).saturating_sub(offset / 2),
                    y_pos,
                    &self.label,
                    self.label_size,
                    label_col,
                );
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

    pub fn is_clicked(&mut self, window: &crate::window::Window) -> bool {
        let state = window.get_mouse_state();
        if self.is_hovered(window) && state.lmb_clicked {
            self.state = ButtonState::Clicked;
            true
        } else {
            self.state = ButtonState::Hovered;
            false
        }
    }

    fn update(&mut self, window: &mut crate::window::Window) {
        match self.is_hovered(window) {
            false => self.state = ButtonState::Idle,
            true => match self.is_clicked(window) {
                false => self.state = ButtonState::Hovered,
                true => self.state = ButtonState::Clicked
            }
        }
    }

    /// Draws the button to a window
    pub fn draw(&mut self, window: &mut crate::window::Window) {
        let state = window.get_mouse_state();
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
