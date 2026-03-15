#[derive(Default)]
pub struct Button {
    pub label: crate::ui::text::Text,
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

    /// The color of the Button's label
    pub label_col: crate::color::Color,
    /// The color of the Button's border
    pub border_col: crate::color::Color,
    /// The color of the Button's background
    pub bg_col: crate::color::Color,

    /// Whether the Button is a push button or a toggle button
    pub button_type: ButtonType,
}

impl Button {
    /// Sets the text to be displayed inside the button
    pub fn label(mut self, text: &str, font: crate::ttf::Font) -> Self {
        self.label = crate::ui::text::Text::new(text, font);
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

    /// Determines whether to draw shadows and how large they should be
    pub fn shadow(mut self, enable: bool, size: usize) -> Self {
        self.inner_shadow = enable;
        self.shadow_size = size;
        self
    }

    /// Sets the label text color
    pub fn label_color(mut self, color: crate::color::Color) -> Self {
        self.label_col = color;
        self
    }

    /// Sets the border color
    pub fn border_color(mut self, color: crate::color::Color) -> Self {
        self.border_col = color;
        self
    }

    /// Sets the button's background color
    pub fn bg_color(mut self, color: crate::color::Color) -> Self {
        self.bg_col = color;
        self
    }

    /// Determines whether the button is a push button or toggle button
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    /// Draws the button to a window
    pub fn draw(&self, window: &mut crate::window::Window) {
        window.draw_rect_f(
            self.pos_x,
            self.pos_y,
            self.width,
            self.height,
            &self.bg_col,
        );
        for i in 0..self.border_size {
            window.draw_rect(
                self.pos_x + i,
                self.pos_y + i,
                self.width - i * 2,
                self.height - i * 2,
                &self.border_col,
            );
        }
        let y_pos = self.pos_y + (self.height - self.label.font.font.metrics('A', 16.0).height) / 2;
        match self.text_alignment {
            Alignment::Left => {
                window.draw_text(
                    self.pos_x + self.border_size + 2,
                    y_pos,
                    &self.label,
                    16.0,
                    &self.label_col,
                );
            }
            Alignment::Right => {
                let mut offset = 0;
                for c in 0..self.label.text.len() {
                    offset += (self
                        .label
                        .font
                        .font
                        .metrics(self.label.text.as_bytes()[c] as char, 16.0)
                        .advance_width) as usize;
                }
                window.draw_text(
                    (self.pos_x + self.width) - offset - 2,
                    y_pos,
                    &self.label,
                    16.0,
                    &self.label_col,
                );
            }
            Alignment::Center => {
                let mut offset = 0;
                for c in 0..self.label.text.len() {
                    offset += (self
                        .label
                        .font
                        .font
                        .metrics(self.label.text.as_bytes()[c] as char, 16.0)
                        .advance_width) as usize;
                }
                window.draw_text(
                    (self.pos_x + self.width / 2) - offset / 2,
                    y_pos,
                    &self.label,
                    16.0,
                    &self.label_col,
                );
            }
        }
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
