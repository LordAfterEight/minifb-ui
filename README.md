# Minifb-UI

A lightweight UI library built on top of [minifb](https://crates.io/crates/minifb), providing high-level abstractions for creating windows and interactive UI elements with SDF-based anti-aliased rendering.

![docs.rs](https://img.shields.io/docsrs/minifb-ui)
![Crates.io Version](https://img.shields.io/crates/v/minifb-ui)
![GitHub License](https://img.shields.io/github/license/lordaftereight/minifb-ui)

## Features

- **Window management** with direct framebuffer access and configurable anti-aliasing
- **Drawing primitives**: rounded rectangles, circles, ellipses, lines, gradients, bezier curves -- all SDF-based with smooth AA
- **Alpha blending** and **background blur** (`blur_region`, `blur_region_rounded`) for frosted glass effects
- **Box shadows** with configurable offset, spread, and blur
- **TTF/OTF font rendering** via [fontdue](https://crates.io/crates/fontdue) with text measurement and centered drawing
- **UI components**:
  - `Button` -- push or toggle, 3-state (idle/hovered/clicked), builder pattern, inset shadows, rounded corners
  - `Slider` -- numeric range slider with customizable track, handle, and border
  - `TextInput` -- text entry with cursor, scrolling, keyboard input, placeholder text
  - `Switch` -- sliding toggle with animation
- **Color utilities**: RGBA, hex parsing, HSV conversion, lerp, complement

## Example Usage

```rust
use minifb_ui::{Color, Font, Window, Text, Button, Alignment};

fn main() {
    let mut window = Window::custom("Demo", 800, 600, false, false);
    let font = Font::new("assets/font.ttf").unwrap();
    let text = Text::new("Hello, world!", font.clone());

    let mut button = Button::default()
        .label("Click Me", font.clone(), 16.0)
        .label_alignment(Alignment::Center)
        .position(100, 100)
        .size(150, 40)
        .border(1).radius(10)
        .background(Color::from(0x6C5CE7))
        .hover_bg(Color::from(0x8B7CF7))
        .label_color(Color::from(0xFFFFFF));

    while window.window.is_open() {
        window.clear(&Color::from(0x0C0C18));
        window.draw_text(20, 20, &text, 24.0, &Color::from(0xE8E8F0));
        button.draw(&mut window);

        if button.is_left_clicked(&window) {
            println!("Clicked!");
        }

        window.update();
    }
}
```

## Demo

The included `main.rs` implements a full mock desktop environment showcasing all components:

- Frosted glass taskbar with start menu, calendar, and notification center
- Draggable app windows (Files, Terminal, Notes, Calculator, Settings) with background blur and transparency
- Light/dark mode toggle via the `Switch` component
- Functional calculator, file browser, and settings panel with live switches

Run it with:

```sh
cargo run --release
```

## AI Disclosure

The majority of the code in this crate was written by Claude, guided by detailed instructions specifying the exact architecture, component APIs, rendering approaches, and visual design. I directed what to build, how each component should work, and what the final result should look like so Claude could implement it.
