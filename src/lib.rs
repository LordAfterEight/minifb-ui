//! # Minifb-UI
//!
//! This is a crate aiming to make usage of the minifb crate easier than it already is,
//! by providing abstractions and features for easy creation and management of windows
//! and UI elements.
//!
//! # Example Usage
//!
//! ```rust
//! use minifb_ui;
//!
//! fn main() {
//!     let mut window = minifb_ui::window::Window::custom("TestWindow", 1920, 1080, false, false);
//!
//!     let font = minifb_ui::ttf::Font::new("assets/Dico.ttf").unwrap();
//!     let text = minifb_ui::ui::text::Text::new(
//!         "The quick brown fox jumps over the lazy dog  !\"§$%&/()=?+~*#'-_.:,;<>|",
//!         font,
//!     );
//!
//!     let mut button = minifb_ui::ui::button::Button::default()
//!         .label(
//!             "Press Me!",
//!             minifb_ui::ttf::Font::new("assets/whitrabt.ttf").unwrap(),
//!             20.0
//!         )
//!         .idle_label_col(minifb_ui::color::Color::from(0xCCCCCC))
//!         .hover_label_col(minifb_ui::color::Color::from(0xDDDDDD))
//!         .click_label_col(minifb_ui::color::Color::from(0xAAFFAA))
//!         .label_alignment(minifb_ui::ui::button::Alignment::Center)
//!         .position(100, 100)
//!         .size(150, 33)
//!         .border(2)
//!         .idle_shadow(5, 10)
//!         .hover_shadow(7, 20)
//!         .click_shadow(10, 10)
//!         .border_color(minifb_ui::color::Color::from(0x444444))
//!         .background(minifb_ui::color::Color::from(0x222222))
//!         .hover_bg(minifb_ui::color::Color::from(0x333333));
//!
//!     while window.window.is_open() {
//!         window.clear(&minifb_ui::color::Color::from(0x0));
//!         window.draw_text(
//!             10,
//!             10,
//!             &text,
//!             16.0,
//!             &minifb_ui::color::Color::from(0xFFFFFF),
//!         );
//!         button.draw(&mut window);
//!         window.update();
//!     }
//! }
//! ```

/// Provides necessary things to work with ttf and otf fonts
pub mod ttf;
/// Provides the Window struct and gives you everything you need to create one and draw in it
pub mod window;
/// Provides the Color RGBA struct
pub mod color;
/// Provides UI elements you can draw in a Window
pub mod ui;
/// Provides a Theme struct with dark/light presets
pub mod theme;
/// Provides animation/tween utilities
pub mod anim;
/// Provides layout helpers (VStack, HStack)
pub mod layout;

pub use minifb::{Key, KeyRepeat};

pub use color::Color;
pub use ttf::Font;
pub use window::Window;
pub use theme::Theme;
pub use anim::{Tween, Easing};
pub use layout::{VStack, HStack};
pub use ui::{Text, Button, ButtonType, Alignment, Slider, TextInput, Switch,
    Checkbox, ProgressBar, Dropdown, Tooltip, ScrollArea, Tabs, ContextMenu};
