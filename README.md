# Minifb-UI
This is a crate aiming to make usage of the minifb crate easier than it already is, by providing abstractions and features for easy creation and management of windows and UI elements.

![docs.rs](https://img.shields.io/docsrs/minifb-ui)
![Crates.io Version](https://img.shields.io/crates/v/minifb-ui)
![GitHub License](https://img.shields.io/github/license/lordaftereight/minifb-ui)


# Example Usage
```rust
use minifb_ui;

fn main() {
    let mut window = minifb_ui::window::Window::custom("TestWindow", 1920, 1080, false, false);

    let font = minifb_ui::ttf::Font::new("assets/Dico.ttf");
    let text = minifb_ui::ui::text::Text::new(
        "The quick brown fox jumps over the lazy dog  !\"§$%&/()=?+~*#'-_.:,;<>|",
        font,
    );

    let mut button = minifb_ui::ui::button::Button::default()
        .label(
            "Press Me!",
            minifb_ui::ttf::Font::new("assets/whitrabt.ttf"),
            20.0
        )
        .idle_label_col(minifb_ui::color::Color::from(0xCCCCCC))
        .hover_label_col(minifb_ui::color::Color::from(0xDDDDDD))
        .click_label_col(minifb_ui::color::Color::from(0xAAFFAA))
        .label_alignment(minifb_ui::ui::button::Alignment::Center)
        .position(100, 100)
        .size(150, 33)
        .border(2)
        .idle_shadow(5, 10)
        .hover_shadow(7, 20)
        .click_shadow(10, 10)
        .border_color(minifb_ui::color::Color::from(0x444444))
        .background(minifb_ui::color::Color::from(0x222222))
        .hover_bg(minifb_ui::color::Color::from(0x333333));

    while window.window.is_open() {
        window.draw_text(
            10,
            10,
            &text,
            16.0,
            &minifb_ui::color::Color::from(0xFFFFFF),
        );
        button.draw(&mut window);
        window.update();
    }
}
```
### This is what this button looks like when idle, hovered and clicked!
<p align="center">
    <img align=center src="example/example1_idle.png">
</p>
<p align="center">
    <img align=center src="example/example1_hovered.png">
</p>
<p align="center">
    <img align=center src="example/example1_clicked.png">
</p>
