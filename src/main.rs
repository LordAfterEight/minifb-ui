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
        .label_color(minifb_ui::color::Color::from(0xFFFFFF))
        .label_alignment(minifb_ui::ui::button::Alignment::Center)
        .position(100, 100)
        .size(150, 33)
        .border(true, 1)
        .shadow(true, 0, 0)
        .idle_shadow(5, 10)
        .hover_shadow(7, 15)
        .click_shadow(10, 20)
        .border_color(minifb_ui::color::Color::from(0x444444))
        .bg_color(minifb_ui::color::Color::from(0x222222));

    while window.window.is_open() {
        window.clear(&minifb_ui::color::Color::from(0x0));
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
