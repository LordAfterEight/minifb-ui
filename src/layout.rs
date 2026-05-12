/// Vertical stack layout helper.
/// Call `next()` repeatedly to get positions for each child element.
pub struct VStack {
    x: usize,
    y: usize,
    spacing: usize,
    count: usize,
}

impl VStack {
    pub fn new(x: usize, y: usize, spacing: usize) -> Self {
        Self { x, y, spacing, count: 0 }
    }

    /// Returns the (x, y) position for the next element, then advances by `height + spacing`.
    pub fn next(&mut self, height: usize) -> (usize, usize) {
        let pos = (self.x, self.y);
        self.y += height + self.spacing;
        self.count += 1;
        pos
    }

    /// Returns the current y position without advancing
    pub fn current_y(&self) -> usize {
        self.y
    }

    /// Returns how many items have been placed
    pub fn count(&self) -> usize {
        self.count
    }
}

/// Horizontal stack layout helper.
/// Call `next()` repeatedly to get positions for each child element.
pub struct HStack {
    x: usize,
    y: usize,
    spacing: usize,
    count: usize,
}

impl HStack {
    pub fn new(x: usize, y: usize, spacing: usize) -> Self {
        Self { x, y, spacing, count: 0 }
    }

    /// Returns the (x, y) position for the next element, then advances by `width + spacing`.
    pub fn next(&mut self, width: usize) -> (usize, usize) {
        let pos = (self.x, self.y);
        self.x += width + self.spacing;
        self.count += 1;
        pos
    }

    /// Returns the current x position without advancing
    pub fn current_x(&self) -> usize {
        self.x
    }

    /// Returns how many items have been placed
    pub fn count(&self) -> usize {
        self.count
    }
}
