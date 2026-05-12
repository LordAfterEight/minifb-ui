pub struct ScrollArea {
    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub height: usize,

    pub content_height: usize,
    scroll_offset: f32,
    pub scroll_speed: f32,

    pub scrollbar_width: usize,
    pub scrollbar_color: crate::color::Color,
    pub scrollbar_track_color: crate::color::Color,
    pub scrollbar_radius: usize,

    dragging: bool,
    drag_start_y: f32,
    drag_start_offset: f32,
    lmb_was_down: bool,
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            width: 200,
            height: 300,
            content_height: 300,
            scroll_offset: 0.0,
            scroll_speed: 3.0,
            scrollbar_width: 6,
            scrollbar_color: crate::color::Color::rgba(200, 200, 210, 120),
            scrollbar_track_color: crate::color::Color::rgba(60, 60, 80, 40),
            scrollbar_radius: 3,
            dragging: false,
            drag_start_y: 0.0,
            drag_start_offset: 0.0,
            lmb_was_down: false,
        }
    }
}

impl ScrollArea {
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

    pub fn content_height(mut self, h: usize) -> Self {
        self.content_height = h;
        self
    }

    pub fn scroll_speed(mut self, speed: f32) -> Self {
        self.scroll_speed = speed;
        self
    }

    pub fn scrollbar_width(mut self, w: usize) -> Self {
        self.scrollbar_width = w;
        self
    }

    pub fn scrollbar_color(mut self, color: crate::color::Color) -> Self {
        self.scrollbar_color = color;
        self
    }

    pub fn scrollbar_track_color(mut self, color: crate::color::Color) -> Self {
        self.scrollbar_track_color = color;
        self
    }

    pub fn scrollbar_radius(mut self, r: usize) -> Self {
        self.scrollbar_radius = r;
        self
    }

    /// Sets the content height dynamically (call before draw each frame if content changes)
    pub fn set_content_height(&mut self, h: usize) {
        self.content_height = h;
    }

    /// Returns current scroll offset in pixels
    pub fn offset(&self) -> f32 {
        self.scroll_offset
    }

    /// Sets scroll offset directly
    pub fn set_offset(&mut self, offset: f32) {
        self.scroll_offset = offset.max(0.0);
        self.clamp_offset();
    }

    /// Returns true if content overflows the visible area
    pub fn can_scroll(&self) -> bool {
        self.content_height > self.height
    }

    fn max_offset(&self) -> f32 {
        (self.content_height as f32 - self.height as f32).max(0.0)
    }

    fn clamp_offset(&mut self) {
        self.scroll_offset = self.scroll_offset.clamp(0.0, self.max_offset());
    }

    /// Updates scroll state (call once per frame, before drawing content)
    /// Pushes a clip region. Call `end_draw` after drawing content.
    pub fn begin_draw(&mut self, window: &mut crate::window::Window) {
        let mouse = window.get_mouse_state();
        let mx = mouse.pos_x;
        let my = mouse.pos_y;
        let lmb = mouse.lmb_clicked;

        let in_bounds = mx >= self.pos_x as f32
            && mx < (self.pos_x + self.width) as f32
            && my >= self.pos_y as f32
            && my < (self.pos_y + self.height) as f32;

        // Mouse wheel scrolling
        if in_bounds {
            if let Some(scroll) = window.window.get_scroll_wheel() {
                self.scroll_offset += scroll.1 * self.scroll_speed;
                self.clamp_offset();
            }
        }

        // Scrollbar dragging
        if self.can_scroll() {
            let sb_x = self.pos_x + self.width - self.scrollbar_width - 2;
            let thumb_frac = self.height as f32 / self.content_height as f32;
            let thumb_h = (thumb_frac * self.height as f32).max(20.0);
            let track_range = self.height as f32 - thumb_h;
            let thumb_y = self.pos_y as f32 + (self.scroll_offset / self.max_offset()) * track_range;

            let lmb_just = lmb && !self.lmb_was_down;

            if lmb_just {
                let on_thumb = mx >= sb_x as f32
                    && mx < (sb_x + self.scrollbar_width) as f32
                    && my >= thumb_y
                    && my < thumb_y + thumb_h;
                if on_thumb {
                    self.dragging = true;
                    self.drag_start_y = my;
                    self.drag_start_offset = self.scroll_offset;
                }
            }

            if self.dragging {
                if lmb {
                    let delta = my - self.drag_start_y;
                    let ratio = delta / track_range;
                    self.scroll_offset = self.drag_start_offset + ratio * self.max_offset();
                    self.clamp_offset();
                } else {
                    self.dragging = false;
                }
            }
        }

        self.lmb_was_down = lmb;

        // Push clip
        window.push_clip(self.pos_x, self.pos_y, self.width, self.height);
    }

    /// Ends the scroll area draw (pops clip) and draws scrollbar
    pub fn end_draw(&self, window: &mut crate::window::Window) {
        window.pop_clip();

        // Draw scrollbar if content overflows
        if self.can_scroll() {
            let sb_x = self.pos_x + self.width - self.scrollbar_width - 2;
            let thumb_frac = self.height as f32 / self.content_height as f32;
            let thumb_h = (thumb_frac * self.height as f32).max(20.0) as usize;
            let track_range = self.height as f32 - thumb_h as f32;
            let thumb_y = self.pos_y + ((self.scroll_offset / self.max_offset()) * track_range) as usize;

            // Track
            window.draw_rect_f(
                sb_x, self.pos_y, self.scrollbar_width, self.height,
                self.scrollbar_radius, &self.scrollbar_track_color, 0,
            );

            // Thumb
            window.draw_rect_f(
                sb_x, thumb_y, self.scrollbar_width, thumb_h,
                self.scrollbar_radius, &self.scrollbar_color, 0,
            );
        }
    }
}
