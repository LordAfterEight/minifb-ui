use minifb_ui;

fn main() {
    let mut window = minifb_ui::window::Window::custom(
        "TestWindow",
        1920,
        1080,
        false,
        false,
    );

    let font = minifb_ui::ttf::Font::new("assets/Dico.ttf");
    let text = minifb_ui::ui::text::Text::new("The quick brown fox jumps over the lazy dog  !\"§$%&/()=?+~*#'-_.:,;<>|", font.clone());

    let button = minifb_ui::ui::button::Button::default()
        .label("Press Me!", font)
        .text_color(minifb_ui::color::Color::from(0xAAAAAA))
        .label_alignment(minifb_ui::ui::button::Alignment::Center)
        .position(100, 100)
        .size(150, 33)
        .border(true, 1)
        .border_color(minifb_ui::color::Color::from(0x777777))
        .bg_color(minifb_ui::color::Color::from(0x202020));

    while window.window.is_open() {
        window.draw_text(10, 10, &text, 16.0, &minifb_ui::color::Color::from(0xFFFFFF));
        button.draw(&mut window);
        window.update();
    }
}
