use minifb_ui;

fn main() {
    let mut window = minifb_ui::window::Window::custom(
        "TestWindow",
        1920,
        1080,
        false,
        false,
    );
    let font = minifb_ui::ttf::Font::new("assets/Ethnocentric-Regular.otf");
    let text = minifb_ui::ui::Text::new("Hello World", &font);
    while window.window.is_open() {
        window.draw_line(100, 100, 300, 100, 4, minifb_ui::color::Color::from(0xFF0000));
        window.draw_line(100, 104, 300, 104, 4, minifb_ui::color::Color::from(0x00FF00));
        window.draw_line(100, 108, 300, 108, 4, minifb_ui::color::Color::from(0x0000FF));
        window.draw_line(100, 112, 300, 112, 4, minifb_ui::color::Color::from(0xFFFF00));
        window.draw_line(100, 116, 300, 116, 4, minifb_ui::color::Color::from(0x00FFFF));
        window.draw_line(100, 120, 300, 120, 4, minifb_ui::color::Color::from(0xFF00FF));

        window.draw_line(400, 100, 400, 300, 4, minifb_ui::color::Color::from(0xFF0000));
        window.draw_line(404, 100, 404, 300, 4, minifb_ui::color::Color::from(0x00FF00));
        window.draw_line(408, 100, 408, 300, 4, minifb_ui::color::Color::from(0x0000FF));
        window.draw_line(412, 100, 412, 300, 4, minifb_ui::color::Color::from(0xFFFF00));
        window.draw_line(416, 100, 416, 300, 4, minifb_ui::color::Color::from(0x00FFFF));
        window.draw_line(420, 100, 420, 300, 4, minifb_ui::color::Color::from(0xFF00FF));

        window.draw_line(500, 100, 800, 300, 4, minifb_ui::color::Color::from(0xFF0000));
        window.draw_line(508, 100, 808, 300, 4, minifb_ui::color::Color::from(0x00FF00));
        window.draw_line(516, 100, 816, 300, 4, minifb_ui::color::Color::from(0x0000FF));
        window.draw_line(524, 100, 824, 300, 4, minifb_ui::color::Color::from(0xFFFF00));
        window.draw_line(532, 100, 832, 300, 4, minifb_ui::color::Color::from(0x00FFFF));
        window.draw_line(540, 100, 840, 300, 4, minifb_ui::color::Color::from(0xFF00FF));

        window.draw_text(600, 100, &text, 80.0, minifb_ui::color::Color::from(0xFFAAAA));

        window.update();
    }
}
