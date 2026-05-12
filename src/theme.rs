use crate::color::Color;

/// Application-wide color theme with dark and light presets
#[derive(Clone, Copy)]
pub struct Theme {
    // Background
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_surface: Color,

    // Text
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_dim: Color,

    // Accent
    pub accent: Color,
    pub accent_soft: Color,

    // Borders / separators
    pub border: Color,
    pub separator: Color,
    pub highlight: Color,

    // Semantic
    pub danger: Color,
    pub warning: Color,
    pub success: Color,

    // Surfaces
    pub surface: Color,
    pub surface_hover: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            bg_primary: Color::from(0x0E1628),
            bg_secondary: Color::from(0x060A14),
            bg_surface: Color::from(0x1A1A2E),
            text_primary: Color::from(0xE8E8F0),
            text_secondary: Color::from(0x8888A0),
            text_dim: Color::from(0x505068),
            accent: Color::from(0x6C5CE7),
            accent_soft: Color::rgba(108, 92, 231, 40),
            border: Color::from(0x303050),
            separator: Color::from(0x2A2A44),
            highlight: Color::rgba(108, 92, 231, 25),
            danger: Color::from(0xE05555),
            warning: Color::from(0xEAB308),
            success: Color::from(0x34C759),
            surface: Color::from(0x1A1A2E),
            surface_hover: Color::from(0x242440),
        }
    }

    pub fn light() -> Self {
        Self {
            bg_primary: Color::from(0x6C9BCF),
            bg_secondary: Color::from(0x3B6FA0),
            bg_surface: Color::from(0xFFFFFF),
            text_primary: Color::from(0x1A1A2E),
            text_secondary: Color::from(0x5A5A70),
            text_dim: Color::from(0x9090A8),
            accent: Color::from(0x6C5CE7),
            accent_soft: Color::rgba(108, 92, 231, 30),
            border: Color::from(0xC8C8D8),
            separator: Color::from(0xD0D0DC),
            highlight: Color::rgba(108, 92, 231, 20),
            danger: Color::from(0xE05555),
            warning: Color::from(0xEAB308),
            success: Color::from(0x34C759),
            surface: Color::from(0xF0F0F8),
            surface_hover: Color::from(0xE0E0EC),
        }
    }
}
