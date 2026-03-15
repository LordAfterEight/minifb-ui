# Minifb-UI
This is a crate aiming to make usage of the minifb crate easier than it already is, by providing abstractions and features for easy creation and management of windows and UI elements.

# Example Usage
```Rust
use minifb_ui;

fn main() {
    // Create a custom window with resolution and title, and determine whether it's
    // borderless and resizable
    let mut window = minifb_ui::window::Window::custom(
        "TestWindow",
        1920,
        1080,
        false,
        false,
    );

    // You can load both otf and ttf fonts, some might work better than others
    let font = minifb_ui::ttf::Font::new("assets/Dico.ttf");

    // Create an example text and give it a font. A Text instance owns a Font instance
    let text = minifb_ui::ui::text::Text::new(
        "The quick brown fox jumps over the lazy dog  !\"§$%&/()=?+~*#'-_.:,;<>|",
        font.clone()
    );

    // Create a button. Chaining of methods only works at creation, they cannot be used afterwards.
    // Values can still be modified using the fields of the struct instance
    let button = minifb_ui::ui::button::Button::default()
        .label("Press Me!", font)
        .text_color(minifb_ui::color::Color::from(0xAAAAAA))
        .label_alignment(minifb_ui::ui::button::Alignment::Center)
        .position(100, 100)
        .size(150, 33)
        .border(true, 1)
        .border_color(minifb_ui::color::Color::from(0x777777))
        .bg_color(minifb_ui::color::Color::from(0x202020));

    // Infinite loop until the window's close button is pressed
    while window.window.is_open() {
        // Draw the text variable at position 10|10 with size 16 and the color white
        window.draw_text(10, 10, &text, 16.0, &minifb_ui::color::Color::from(0xFFFFFF));

        // Self-explanatory
        button.draw(&mut window);

        // Needs to be called every iteration of a loop
        window.update();
    }
}
```
<img src="example/example1.png">
