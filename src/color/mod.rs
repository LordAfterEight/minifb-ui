/// RGBA color struct
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl Color {
    /// Returns a u32 representing its color as RGB (ignores alpha)
    pub fn as_u32(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

    /// Creates a new opaque color from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Creates a new color with alpha
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Linearly interpolates between two colors by t (0.0 = self, 1.0 = other)
    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let inv = 1.0 - t;
        Color {
            r: (self.r as f32 * inv + other.r as f32 * t) as u8,
            g: (self.g as f32 * inv + other.g as f32 * t) as u8,
            b: (self.b as f32 * inv + other.b as f32 * t) as u8,
            a: (self.a as f32 * inv + other.a as f32 * t) as u8,
        }
    }

    /// Returns the complementary (inverse) color
    pub fn complement(&self) -> Color {
        Color { r: 255 - self.r, g: 255 - self.g, b: 255 - self.b, a: self.a }
    }

    /// Parses a hex color string like "#RRGGBB" or "RRGGBB"
    pub fn from_hex(s: &str) -> Option<Color> {
        let s = s.trim_start_matches('#');
        if s.len() != 6 { return None; }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Color::new(r, g, b))
    }

    /// Creates a color from HSV values (h: 0.0–360.0, s: 0.0–1.0, v: 0.0–1.0)
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Color {
        let s = s.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        let h = ((h % 360.0) + 360.0) % 360.0;
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        let (r, g, b) = match h as u32 {
            0..60 => (c, x, 0.0),
            60..120 => (x, c, 0.0),
            120..180 => (0.0, c, x),
            180..240 => (0.0, x, c),
            240..300 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        Color::new(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self {
            r: ((color >> 16) & 0xFF) as u8,
            g: ((color >> 8)  & 0xFF) as u8,
            b: (color & 0xFF) as u8,
            a: 255,
        }
    }
}
