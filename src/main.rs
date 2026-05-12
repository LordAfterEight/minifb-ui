use minifb_ui::{Color, Font, Window, Text, TextInput, Switch, Key, KeyRepeat,
    Checkbox, ProgressBar, Dropdown, Tooltip, ScrollArea, Tabs, ContextMenu, Tween, Easing,
    Theme};

// ─── Desktop-specific theme extension ──────────────────────
#[derive(Clone, Copy)]
struct DesktopTheme {
    base: Theme,
    taskbar_bg: Color,
    taskbar_border: Color,
    taskbar_icon_hover: Color,
    panel_tint: Color,
    panel_border: Color,
    window_tint: Color,
    window_titlebar_tint: Color,
    window_border: Color,
}

impl DesktopTheme {
    fn dark() -> Self {
        Self {
            base: Theme::dark(),
            taskbar_bg: Color::rgba(8, 8, 18, 210),
            taskbar_border: Color::from(0x2A2A44),
            taskbar_icon_hover: Color::rgba(255, 255, 255, 20),
            panel_tint: Color::rgba(14, 14, 28, 200),
            panel_border: Color::from(0x303050),
            window_tint: Color::rgba(12, 12, 28, 190),
            window_titlebar_tint: Color::rgba(20, 20, 38, 200),
            window_border: Color::from(0x303050),
        }
    }

    fn light() -> Self {
        Self {
            base: Theme::light(),
            taskbar_bg: Color::rgba(240, 242, 248, 220),
            taskbar_border: Color::from(0xC8C8D8),
            taskbar_icon_hover: Color::rgba(0, 0, 0, 15),
            panel_tint: Color::rgba(245, 245, 252, 220),
            panel_border: Color::from(0xC8C8D8),
            window_tint: Color::rgba(240, 240, 250, 190),
            window_titlebar_tint: Color::rgba(228, 228, 240, 210),
            window_border: Color::from(0xC8C8D8),
        }
    }
}

impl std::ops::Deref for DesktopTheme {
    type Target = Theme;
    fn deref(&self) -> &Theme {
        &self.base
    }
}

// ─── Desktop app icons ──────────────────────────────────────
struct DesktopIcon {
    name: &'static str,
    color: Color,
    symbol: &'static str,
    col: usize,
    row: usize,
}

// ─── Start menu app entry ───────────────────────────────────
struct AppEntry {
    name: &'static str,
    color: Color,
    symbol: &'static str,
}

// ─── App window ─────────────────────────────────────────────
struct AppWindow {
    title: String,
    color: Color,
    symbol: String,
    x: isize,
    y: isize,
    w: usize,
    h: usize,
    dragging: bool,
    drag_off_x: f32,
    drag_off_y: f32,
}

impl AppWindow {
    fn new(title: &str, color: Color, symbol: &str, x: isize, y: isize, w: usize, h: usize) -> Self {
        Self {
            title: title.to_string(), color, symbol: symbol.to_string(),
            x, y, w, h,
            dragging: false, drag_off_x: 0.0, drag_off_y: 0.0,
        }
    }
}

fn open_app(windows: &mut Vec<AppWindow>, name: &str, color: Color, symbol: &str) {
    // Don't open duplicate
    if windows.iter().any(|w| w.title == name) {
        // Bring to front
        let idx = windows.iter().position(|w| w.title == name).unwrap();
        let win = windows.remove(idx);
        windows.push(win);
        return;
    }
    let offset = (windows.len() * 30) as isize;
    let (w, h) = match name {
        "Terminal" => (600, 380),
        "Settings" => (480, 400),
        "Calculator" => (280, 380),
        "Components" => (640, 520),
        _ => (560, 400),
    };
    let x = 300 + offset;
    let y = 60 + offset;
    windows.push(AppWindow::new(name, color, symbol, x, y, w, h));
}

fn main() {
    let mut window = Window::custom("Desktop", 1280, 720, false, false);
    let font = Font::new("assets/whitrabt.ttf").unwrap();

    // ─── Layout ─────────────────────────────────────────────
    let taskbar_h: usize = 48;
    let taskbar_y: usize = 720 - taskbar_h;

    // ─── State ──────────────────────────────────────────────
    let mut prev_lmb = false;
    let mut prev_rmb = false;
    let mut start_open = false;
    let mut calendar_open = false;
    let mut notif_open = false;
    let mut selected_desktop_icon: Option<usize> = None;
    let mut last_click_time: u32 = 0; // frame counter for double-click
    let mut last_click_icon: Option<usize> = None;
    let mut frame: u32 = 0;
    let mut app_windows: Vec<AppWindow> = Vec::new();

    // ─── Dark mode switch ───────────────────────────────────
    let mut dark_mode = Switch::default()
        .position(1280 - 280, 720 - 48 + 13) // final position in tray
        .size(42, 22)
        .default_on(true)
        .anim_speed(0.12)
        .track_color_off(Color::new(160, 160, 170))
        .track_color_on(Color::from(0x6C5CE7));

    // ─── Settings switches ──────────────────────────────────
    let mut sw_notif = Switch::default().size(42, 22).default_on(true).anim_speed(0.12)
        .track_color_off(Color::new(120, 120, 130)).track_color_on(Color::from(0x34C759));
    let mut sw_sounds = Switch::default().size(42, 22).default_on(false).anim_speed(0.12)
        .track_color_off(Color::new(120, 120, 130)).track_color_on(Color::from(0x34C759));
    let mut sw_updates = Switch::default().size(42, 22).default_on(true).anim_speed(0.12)
        .track_color_off(Color::new(120, 120, 130)).track_color_on(Color::from(0x34C759));
    let mut sw_wifi = Switch::default().size(42, 22).default_on(true).anim_speed(0.12)
        .track_color_off(Color::new(120, 120, 130)).track_color_on(Color::from(0x34C759));
    let mut sw_bluetooth = Switch::default().size(42, 22).default_on(false).anim_speed(0.12)
        .track_color_off(Color::new(120, 120, 130)).track_color_on(Color::from(0x34C759));

    // ─── Notes text input ───────────────────────────────────
    let mut notes_input = TextInput::default()
        .font(font.clone(), 13.0)
        .position(0, 0)
        .size(300, 30)
        .border(1).radius(6)
        .idle_bg(Color::from(0x1A1A28))
        .editing_bg(Color::from(0x222234))
        .idle_border_col(Color::from(0x303048))
        .editing_border_col(Color::from(0x6C5CE7))
        .text_color(Color::from(0xE8E8F0))
        .cursor_color(Color::from(0x8B7CF7))
        .placeholder("Type here...");

    // ─── Calculator state ───────────────────────────────────
    let mut calc_display = String::from("0");
    let mut calc_operand: f64 = 0.0;
    let mut calc_operator: Option<char> = None;
    let mut calc_fresh = true;

    // ─── Search bar in start menu ───────────────────────────
    let mut search_input = TextInput::default()
        .font(font.clone(), 13.0)
        .position(0, 0)
        .size(300, 34)
        .border(1).radius(17)
        .idle_bg(Color::from(0x1A1A28))
        .editing_bg(Color::from(0x222234))
        .idle_border_col(Color::from(0x303048))
        .editing_border_col(Color::from(0x6C5CE7))
        .text_color(Color::from(0xE8E8F0))
        .cursor_color(Color::from(0x8B7CF7))
        .placeholder("Search apps...");

    // ─── Components demo state ────────────────────────────
    let mut demo_checkbox1 = Checkbox::default()
        .size(18).radius(4).border(1).default_checked(true)
        .box_color(Color::from(0x1A1A2E)).box_color_checked(Color::from(0x6C5CE7))
        .check_color(Color::from(0xFFFFFF)).border_color(Color::from(0x505068));
    let mut demo_checkbox2 = Checkbox::default()
        .size(18).radius(4).border(1)
        .box_color(Color::from(0x1A1A2E)).box_color_checked(Color::from(0x34C759))
        .check_color(Color::from(0xFFFFFF)).border_color(Color::from(0x505068));
    let mut demo_progress = ProgressBar::default()
        .size(200, 14).radius(7).border(0)
        .track_color(Color::from(0x1A1A2E)).fill_color(Color::from(0x6C5CE7));
    let mut demo_dropdown = Dropdown::default()
        .items(&["Option A", "Option B", "Option C", "Option D"])
        .size(160, 28).radius(6).border(1).font(font.clone(), 12.0)
        .bg_color(Color::from(0x1A1A2E)).bg_open_color(Color::rgba(20, 20, 34, 200))
        .border_color(Color::from(0x505068)).text_color(Color::from(0xE8E8F0))
        .hover_color(Color::rgba(108, 92, 231, 40)).blur(8);
    let mut demo_tabs = Tabs::default()
        .tabs(&["General", "Display", "Audio"])
        .width(300).tab_height(32).font(font.clone(), 12.0)
        .bg_color(Color::from(0x12121E)).active_color(Color::from(0x1A1A2E))
        .text_color(Color::from(0x8888A0)).active_text_color(Color::from(0xE8E8F0))
        .indicator_color(Color::from(0x6C5CE7)).border_color(Color::from(0x2A2A44));
    let mut demo_context = ContextMenu::default()
        .items(&["Toggle Theme", "Refresh", "Open Settings", "Open Components"])
        .width(140).font(font.clone(), 12.0)
        .bg_color(Color::from(0x141422)).border_color(Color::from(0x505068))
        .text_color(Color::from(0xE8E8F0)).hover_color(Color::rgba(108, 92, 231, 40));
    let mut demo_scroll = ScrollArea::default()
        .size(280, 160).content_height(400).scroll_speed(3.0)
        .scrollbar_width(5).scrollbar_radius(3)
        .scrollbar_color(Color::rgba(200, 200, 210, 100))
        .scrollbar_track_color(Color::rgba(60, 60, 80, 30));
    let demo_tooltip = Tooltip::new("This is a tooltip!", font.clone(), 11.0)
        .bg_color(Color::from(0x0E0E1C)).text_color(Color::from(0xE8E8F0))
        .border_color(Color::from(0x505068)).border(1).radius(6).padding(6);
    let mut demo_tween = Tween::new(0.0, 0.02).with_easing(Easing::EaseInOut);

    // ─── Desktop icons ──────────────────────────────────────
    let desktop_icons = vec![
        DesktopIcon { name: "Files",    color: Color::from(0xEAB308), symbol: "F", col: 0, row: 0 },
        DesktopIcon { name: "Terminal", color: Color::from(0x1A1A2E), symbol: ">", col: 0, row: 1 },
        DesktopIcon { name: "Browser",  color: Color::from(0x3B82F6), symbol: "W", col: 0, row: 2 },
        DesktopIcon { name: "Notes",    color: Color::from(0x34C759), symbol: "N", col: 0, row: 3 },
        DesktopIcon { name: "Music",    color: Color::from(0xEC4899), symbol: "M", col: 1, row: 0 },
        DesktopIcon { name: "Photos",   color: Color::from(0x8B5CF6), symbol: "P", col: 1, row: 1 },
        DesktopIcon { name: "Components", color: Color::from(0x06B6D4), symbol: "#", col: 1, row: 2 },
    ];

    // ─── Start menu apps ───────────────────────────────────
    let pinned_apps = vec![
        AppEntry { name: "Files",       color: Color::from(0xEAB308), symbol: "F" },
        AppEntry { name: "Browser",     color: Color::from(0x3B82F6), symbol: "W" },
        AppEntry { name: "Terminal",    color: Color::from(0x1A1A2E), symbol: ">" },
        AppEntry { name: "Settings",    color: Color::from(0x7A7A90), symbol: "=" },
        AppEntry { name: "Music",       color: Color::from(0xEC4899), symbol: "M" },
        AppEntry { name: "Photos",      color: Color::from(0x8B5CF6), symbol: "P" },
        AppEntry { name: "Notes",       color: Color::from(0x34C759), symbol: "N" },
        AppEntry { name: "Mail",        color: Color::from(0x06B6D4), symbol: "@" },
        AppEntry { name: "Calendar",    color: Color::from(0xE05555), symbol: "C" },
        AppEntry { name: "Calculator",  color: Color::from(0x64748B), symbol: "+" },
        AppEntry { name: "Store",       color: Color::from(0x22C55E), symbol: "S" },
        AppEntry { name: "Camera",      color: Color::from(0xF97316), symbol: "O" },
        AppEntry { name: "Components", color: Color::from(0x06B6D4), symbol: "#" },
    ];

    let recent_items = [
        ("report_final.pdf", "Documents", "2 hours ago"),
        ("screenshot.png",   "Pictures",  "Yesterday"),
        ("main.rs",          "Projects",  "Yesterday"),
        ("notes.txt",        "Documents", "3 days ago"),
    ];

    let taskbar_icons: Vec<(&str, Color, &str)> = vec![
        ("F", Color::from(0xEAB308), "Files"),
        ("W", Color::from(0x3B82F6), "Browser"),
        (">", Color::from(0x505068), "Terminal"),
        ("M", Color::from(0xEC4899), "Music"),
        ("N", Color::from(0x34C759), "Notes"),
    ];

    // ─── Notifications ──────────────────────────────────────
    let mut notifications: Vec<(&str, &str, &str, Color)> = vec![
        ("System Update", "A new update is available", "5 min ago", Color::from(0x3B82F6)),
        ("Low Storage", "Drive 85% full", "1 hr ago", Color::from(0xEAB308)),
        ("Download Complete", "report.pdf saved", "2 hr ago", Color::from(0x34C759)),
    ];

    // ─── Calendar data ──────────────────────────────────────
    let cal_month = "May 2026";
    let cal_day_hdrs = ["Mo","Tu","We","Th","Fr","Sa","Su"];
    let cal_start_offset = 4; // May 2026 starts on Friday
    let cal_num_days = 31;
    let cal_today = 11;

    // ─── Static texts ───────────────────────────────────────
    let t_pinned = Text::new("PINNED", font.clone());
    let t_recent = Text::new("RECENT", font.clone());
    let t_notif_title = Text::new("Notifications", font.clone());
    let t_clear = Text::new("Clear all", font.clone());

    // Terminal mock output
    let terminal_lines = [
        "user@desktop:~$ neofetch",
        "  OS:     MinifbOS 1.0",
        "  Host:   Virtual Desktop",
        "  Kernel: minifb-ui 0.1.20",
        "  Shell:  rush 0.1",
        "  CPU:    Simulated @ 60fps",
        "  Memory: 256MB / 512MB",
        "",
        "user@desktop:~$ ls -la",
        "drwxr-xr-x  Documents/",
        "drwxr-xr-x  Downloads/",
        "drwxr-xr-x  Pictures/",
        "-rw-r--r--  notes.txt",
        "-rw-r--r--  main.rs",
        "",
        "user@desktop:~$ _",
    ];

    // Files mock
    let file_entries = [
        ("Documents", "F", Color::from(0xEAB308), "Folder", "12 items"),
        ("Downloads", "F", Color::from(0x3B82F6), "Folder", "8 items"),
        ("Pictures",  "F", Color::from(0x8B5CF6), "Folder", "24 items"),
        ("Music",     "F", Color::from(0xEC4899), "Folder", "16 items"),
        ("main.rs",   "R", Color::from(0xF97316), "File",   "4.2 KB"),
        ("notes.txt", "T", Color::from(0x64748B), "File",   "1.1 KB"),
        ("report.pdf","P", Color::from(0xE05555), "File",   "2.8 MB"),
        ("photo.png", "I", Color::from(0x22C55E), "File",   "840 KB"),
    ];

    // Settings labels
    let settings_items = ["Notifications", "Sounds", "Auto Updates", "Wi-Fi", "Bluetooth"];

    // Calculator buttons
    let calc_buttons = [
        ["C", "+/-", "%", "/"],
        ["7", "8", "9", "x"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
        ["0", "0", ".", "="],
    ];

    while window.window.is_open() {
        frame += 1;

        // ─── Theme (read state BEFORE drawing) ──────────────
        let theme = if dark_mode.is_on() { DesktopTheme::dark() } else { DesktopTheme::light() };

        let mouse = window.get_mouse_state();
        let lmb = mouse.lmb_clicked;
        let lmb_just = lmb && !prev_lmb;
        let rmb = mouse.rmb_clicked;
        let rmb_just = rmb && !prev_rmb;
        let mx = mouse.pos_x;
        let my = mouse.pos_y;

        let any_overlay = start_open || calendar_open || notif_open;

        // ═════════════════════════════════════════════════════
        //  DESKTOP BACKGROUND
        // ═════════════════════════════════════════════════════
        window.draw_gradient_v(0, 0, 1280, taskbar_y, 0, &theme.bg_primary, &theme.bg_secondary);

        // Subtle decorative orbs
        window.draw_ellipse_f(200, 250, 180, 180, &Color::rgba(108, 92, 231, 35), 0);
        window.draw_ellipse_f(900, 150, 220, 220, &Color::rgba(59, 130, 246, 28), 0);
        window.draw_ellipse_f(1100, 500, 160, 160, &Color::rgba(236, 72, 153, 22), 0);

        // ─── Desktop icons ──────────────────────────────────
        let icon_grid_x: usize = 40;
        let icon_grid_y: usize = 30;
        let icon_cell_w: usize = 90;
        let icon_cell_h: usize = 100;
        let icon_size: usize = 48;

        for (i, dicon) in desktop_icons.iter().enumerate() {
            let cx = icon_grid_x + dicon.col * icon_cell_w + icon_cell_w / 2;
            let cy = icon_grid_y + dicon.row * icon_cell_h + 30;

            let hovered = !any_overlay
                && mx >= (cx - icon_size / 2) as f32 && mx < (cx + icon_size / 2) as f32
                && my >= (cy - icon_size / 2) as f32 && my < (cy + icon_size / 2 + 24) as f32;

            let is_selected = selected_desktop_icon == Some(i);

            if is_selected || hovered {
                window.draw_rect_f(
                    cx - icon_cell_w / 2 + 4, icon_grid_y + dicon.row * icon_cell_h + 2,
                    icon_cell_w - 8, icon_cell_h - 4, 8,
                    &if is_selected { theme.accent_soft } else { theme.highlight },
                    0,
                );
            }

            window.draw_rect_f(cx - icon_size / 2, cy - icon_size / 2, icon_size, icon_size, 12, &dicon.color, 0);
            let sym = Text::new(dicon.symbol, font.clone());
            window.draw_text_centered(cx - icon_size / 2, cy - icon_size / 2, icon_size, icon_size, &sym, 22.0, &Color::from(0xFFFFFF));

            let label = Text::new(dicon.name, font.clone());
            let lw = label.get_width(11.0);
            window.draw_text(cx - lw / 2, cy + icon_size / 2 + 6, &label, 11.0, &theme.text_primary);

            if lmb_just && hovered && !any_overlay {
                // Double-click detection
                if last_click_icon == Some(i) && frame - last_click_time < 20 {
                    open_app(&mut app_windows, dicon.name, dicon.color, dicon.symbol);
                    selected_desktop_icon = None;
                    last_click_icon = None;
                } else {
                    selected_desktop_icon = Some(i);
                    last_click_icon = Some(i);
                    last_click_time = frame;
                }
            }
        }

        // Deselect on empty desktop click
        if lmb_just && !any_overlay && selected_desktop_icon.is_some() {
            let mut on_icon = false;
            for dicon in desktop_icons.iter() {
                let cx = icon_grid_x + dicon.col * icon_cell_w + icon_cell_w / 2;
                let cy = icon_grid_y + dicon.row * icon_cell_h + 30;
                if mx >= (cx - icon_size / 2) as f32 && mx < (cx + icon_size / 2) as f32
                    && my >= (cy - icon_size / 2) as f32 && my < (cy + icon_size / 2 + 24) as f32
                { on_icon = true; break; }
            }
            if !on_icon { selected_desktop_icon = None; }
        }

        // Desktop right-click context menu
        if rmb_just && !any_overlay && (my as usize) < taskbar_y {
            demo_context.open(mx as usize, my as usize);
        }
        if let Some(idx) = demo_context.clicked_item() {
            match idx {
                0 => { let v = dark_mode.is_on(); dark_mode.set_on(!v); },
                1 => { /* Refresh — no-op */ },
                2 => open_app(&mut app_windows, "Settings", Color::from(0x6366F1), "\u{2699}"),
                3 => open_app(&mut app_windows, "Components", Color::from(0x06B6D4), "#"),
                _ => {}
            }
        }
        demo_context.draw(&mut window);

        // ═════════════════════════════════════════════════════
        //  APP WINDOWS
        // ═════════════════════════════════════════════════════
        let titlebar_h: usize = 36;
        let win_radius: usize = 10;
        let mut close_idx: Option<usize> = None;
        let mut bring_front_idx: Option<usize> = None;

        // Handle dragging (for the topmost dragging window)
        for aw in app_windows.iter_mut().rev() {
            if aw.dragging {
                if lmb {
                    aw.x = (mx - aw.drag_off_x) as isize;
                    aw.y = (my - aw.drag_off_y) as isize;
                    // Clamp so title bar is always reachable
                    aw.x = aw.x.max(-(aw.w as isize) + 80).min(1200);
                    aw.y = aw.y.max(0).min((taskbar_y - titlebar_h) as isize);
                } else {
                    aw.dragging = false;
                }
                break; // only one window drags at a time
            }
        }

        // Draw all windows
        for (wi, aw) in app_windows.iter().enumerate() {
            let wx = aw.x.max(0) as usize;
            let wy = aw.y.max(0) as usize;

            // Frosted glass window
            window.draw_rect_f(wx, wy, aw.w, aw.h, win_radius, &theme.window_tint, 30);
            window.draw_rect(wx, wy, aw.w, aw.h, win_radius, &theme.window_border);

            // Title bar (slightly more opaque)
            window.draw_rect_f(wx, wy, aw.w, titlebar_h, win_radius, &theme.window_titlebar_tint, 0);
            window.draw_rect_f(wx, wy + titlebar_h - win_radius, aw.w, win_radius, 0, &theme.window_titlebar_tint, 0);
            window.draw_rect_f(wx, wy + titlebar_h, aw.w, 1, 0, &theme.window_border, 0);

            // Title bar icon + text
            window.draw_rect_f(wx + 10, wy + 8, 20, 20, 5, &aw.color, 0);
            let sym_t = Text::new(&aw.symbol, font.clone());
            window.draw_text_centered(wx + 10, wy + 8, 20, 20, &sym_t, 12.0, &Color::from(0xFFFFFF));
            let title_t = Text::new(&aw.title, font.clone());
            window.draw_text(wx + 38, wy + 11, &title_t, 13.0, &theme.text_primary);

            // Close button
            let close_x = wx + aw.w - 32;
            let close_y = wy + 8;
            let close_hov = mx >= close_x as f32 && mx < (close_x + 20) as f32
                && my >= close_y as f32 && my < (close_y + 20) as f32;
            if close_hov {
                window.draw_rect_f(close_x, close_y, 20, 20, 5, &theme.danger, 0);
                let xt = Text::new("x", font.clone());
                window.draw_text_centered(close_x, close_y, 20, 20, &xt, 12.0, &Color::from(0xFFFFFF));
            } else {
                let xt = Text::new("x", font.clone());
                window.draw_text_centered(close_x, close_y, 20, 20, &xt, 12.0, &theme.text_dim);
            }

            if lmb_just && close_hov {
                close_idx = Some(wi);
            }

            // ─── Window content area ────────────────────────
            let cx = wx + 1;
            let cy = wy + titlebar_h + 1;
            let cw = aw.w - 2;
            let ch = aw.h - titlebar_h - 2;

            match aw.title.as_str() {
                "Terminal" => {
                    // Dark terminal background (semi-transparent)
                    window.draw_rect_f(cx, cy, cw, ch, 0, &Color::rgba(8, 8, 14, 200), 0);
                    for (li, line) in terminal_lines.iter().enumerate() {
                        let ly = cy + 10 + li * 18;
                        if ly + 14 > cy + ch { break; }
                        let lt = Text::new(line, font.clone());
                        let col = if line.starts_with("user@") {
                            Color::from(0x34C759)
                        } else if line.starts_with("drw") || line.starts_with("-rw") {
                            Color::from(0x8888A0)
                        } else if line.contains(':') && !line.starts_with("user") {
                            Color::from(0x6C5CE7)
                        } else {
                            Color::from(0xC0C0D0)
                        };
                        window.draw_text(cx + 12, ly, &lt, 12.0, &col);
                    }
                }
                "Files" => {
                    // Toolbar
                    window.draw_rect_f(cx, cy, cw, 32, 0, &theme.surface, 0);
                    window.draw_rect_f(cx, cy + 32, cw, 1, 0, &theme.separator, 0);
                    let path_t = Text::new("/ Home", font.clone());
                    window.draw_text(cx + 12, cy + 9, &path_t, 12.0, &theme.text_secondary);

                    // Column headers
                    let hy = cy + 40;
                    let name_h = Text::new("Name", font.clone());
                    let type_h = Text::new("Type", font.clone());
                    let size_h = Text::new("Size", font.clone());
                    window.draw_text(cx + 44, hy, &name_h, 10.0, &theme.text_dim);
                    window.draw_text(cx + cw - 180, hy, &type_h, 10.0, &theme.text_dim);
                    window.draw_text(cx + cw - 80, hy, &size_h, 10.0, &theme.text_dim);
                    window.draw_rect_f(cx + 8, hy + 16, cw - 16, 1, 0, &theme.separator, 0);

                    let row_h: usize = 32;
                    for (fi, &(fname, fsym, fcolor, ftype, fsize)) in file_entries.iter().enumerate() {
                        let ry = hy + 22 + fi * row_h;
                        if ry + row_h > cy + ch { break; }

                        let row_hov = mx >= cx as f32 && mx < (cx + cw) as f32
                            && my >= ry as f32 && my < (ry + row_h) as f32;
                        if row_hov {
                            window.draw_rect_f(cx + 4, ry, cw - 8, row_h, 0, &theme.highlight, 0);
                        }

                        // Icon
                        window.draw_rect_f(cx + 12, ry + 4, 24, 24, 5, &fcolor, 0);
                        let fs = Text::new(fsym, font.clone());
                        window.draw_text_centered(cx + 12, ry + 4, 24, 24, &fs, 12.0, &Color::from(0xFFFFFF));

                        // Name, type, size
                        let nt = Text::new(fname, font.clone());
                        window.draw_text(cx + 44, ry + 8, &nt, 12.0, &theme.text_primary);
                        let tt = Text::new(ftype, font.clone());
                        window.draw_text(cx + cw - 180, ry + 8, &tt, 11.0, &theme.text_dim);
                        let st = Text::new(fsize, font.clone());
                        window.draw_text(cx + cw - 80, ry + 8, &st, 11.0, &theme.text_secondary);
                    }

                    // Status bar
                    let sb_y = cy + ch - 24;
                    window.draw_rect_f(cx, sb_y, cw, 1, 0, &theme.separator, 0);
                    let items_t = Text::new("8 items", font.clone());
                    window.draw_text(cx + 12, sb_y + 6, &items_t, 10.0, &theme.text_dim);
                }
                "Notes" => {
                    // Note lines
                    let note_lines = [
                        "== My Notes ==",
                        "",
                        "- Review PR #42",
                        "- Fix the rendering bug",
                        "- Update documentation",
                        "- Test on Linux and macOS",
                        "",
                        "Ideas:",
                        "  * Add drag-and-drop",
                        "  * Dark mode improvements",
                    ];
                    for (li, line) in note_lines.iter().enumerate() {
                        let ly = cy + 12 + li * 20;
                        if ly + 16 > cy + ch - 40 { break; }
                        let lt = Text::new(line, font.clone());
                        let col = if line.starts_with("==") { theme.accent }
                            else if line.starts_with("- ") { theme.text_primary }
                            else { theme.text_secondary };
                        window.draw_text(cx + 16, ly, &lt, 12.0, &col);
                    }

                    // Input at bottom
                    window.draw_rect_f(cx, cy + ch - 40, cw, 1, 0, &theme.separator, 0);
                    notes_input.pos_x = cx + 8;
                    notes_input.pos_y = cy + ch - 34;
                    notes_input.width = cw - 16;
                    notes_input.bg_col_idle = theme.surface;
                    notes_input.bg_col_editing = theme.surface_hover;
                    notes_input.border_col_idle = theme.separator;
                    notes_input.border_col_editing = theme.accent;
                    notes_input.text_col_idle = theme.text_primary;
                    notes_input.text_col_editing = theme.text_primary;
                    notes_input.draw(&mut window);
                }
                "Settings" => {
                    // Header
                    let sh_t = Text::new("GENERAL", font.clone());
                    window.draw_text(cx + 20, cy + 16, &sh_t, 10.0, &theme.text_dim);

                    let switches: [&mut Switch; 5] = [
                        &mut sw_notif, &mut sw_sounds, &mut sw_updates, &mut sw_wifi, &mut sw_bluetooth,
                    ];
                    for (si, (sw, label)) in switches.into_iter().zip(settings_items.iter()).enumerate() {
                        let ry = cy + 40 + si * 48;
                        let lt = Text::new(label, font.clone());
                        window.draw_text(cx + 20, ry + 4, &lt, 13.0, &theme.text_primary);

                        sw.pos_x = cx + cw - 66;
                        sw.pos_y = ry + 2;
                        sw.draw(&mut window);

                        if si < settings_items.len() - 1 {
                            window.draw_rect_f(cx + 20, ry + 38, cw - 40, 1, 0,
                                &Color::rgba(theme.separator.r, theme.separator.g, theme.separator.b, 120), 0);
                        }
                    }

                    // About section
                    let about_y = cy + 40 + settings_items.len() * 48 + 10;
                    window.draw_rect_f(cx + 20, about_y, cw - 40, 1, 0, &theme.separator, 0);
                    let about_h = Text::new("ABOUT", font.clone());
                    window.draw_text(cx + 20, about_y + 14, &about_h, 10.0, &theme.text_dim);
                    let ver_t = Text::new("MinifbOS 1.0", font.clone());
                    window.draw_text(cx + 20, about_y + 34, &ver_t, 12.0, &theme.text_primary);
                    let ver2 = Text::new("Built with minifb-ui", font.clone());
                    window.draw_text(cx + 20, about_y + 52, &ver2, 11.0, &theme.text_dim);
                }
                "Calculator" => {
                    // Display
                    window.draw_rect_f(cx + 12, cy + 12, cw - 24, 52, 8, &theme.surface, 0);
                    let disp_t = Text::new(&calc_display, font.clone());
                    let dw = disp_t.get_width(24.0);
                    window.draw_text(cx + cw - 24 - dw, cy + 24, &disp_t, 24.0, &theme.text_primary);

                    // Buttons grid
                    let btn_w = (cw - 24 - 12) / 4; // 4 columns, 12px total gap
                    let btn_h: usize = 44;
                    let btn_gap: usize = 4;
                    let grid_y = cy + 76;

                    for (row_i, row) in calc_buttons.iter().enumerate() {
                        let mut col_i = 0;
                        while col_i < row.len() {
                            let label = row[col_i];
                            // "0" spans 2 columns in last row
                            let span = if row_i == 4 && col_i == 0 { 2 } else { 1 };
                            let bx = cx + 12 + col_i * (btn_w + btn_gap);
                            let by = grid_y + row_i * (btn_h + btn_gap);
                            let bw = btn_w * span + btn_gap * (span - 1);

                            let is_op = matches!(label, "/" | "x" | "-" | "+" | "=");
                            let is_func = matches!(label, "C" | "+/-" | "%");

                            let btn_hov = mx >= bx as f32 && mx < (bx + bw) as f32
                                && my >= by as f32 && my < (by + btn_h) as f32;

                            let bg = if btn_hov {
                                if is_op { Color::from(0x8B7CF7) }
                                else { theme.surface_hover }
                            } else if is_op {
                                theme.accent
                            } else if is_func {
                                theme.surface_hover
                            } else {
                                theme.surface
                            };

                            window.draw_rect_f(bx, by, bw, btn_h, 8, &bg, 0);
                            let bt = Text::new(label, font.clone());
                            let txt_col = if is_op { Color::from(0xFFFFFF) } else { theme.text_primary };
                            window.draw_text_centered(bx, by, bw, btn_h, &bt, 16.0, &txt_col);

                            if lmb_just && btn_hov {
                                match label {
                                    "C" => { calc_display = "0".to_string(); calc_operand = 0.0; calc_operator = None; calc_fresh = true; }
                                    "+/-" => {
                                        if let Ok(v) = calc_display.parse::<f64>() {
                                            calc_display = format_calc(-v);
                                        }
                                    }
                                    "%" => {
                                        if let Ok(v) = calc_display.parse::<f64>() {
                                            calc_display = format_calc(v / 100.0);
                                        }
                                    }
                                    "+" | "-" | "x" | "/" => {
                                        if let Ok(v) = calc_display.parse::<f64>() {
                                            if let Some(op) = calc_operator {
                                                calc_operand = calc_eval(calc_operand, v, op);
                                                calc_display = format_calc(calc_operand);
                                            } else {
                                                calc_operand = v;
                                            }
                                        }
                                        calc_operator = Some(label.chars().next().unwrap());
                                        calc_fresh = true;
                                    }
                                    "=" => {
                                        if let (Some(op), Ok(v)) = (calc_operator, calc_display.parse::<f64>()) {
                                            let result = calc_eval(calc_operand, v, op);
                                            calc_display = format_calc(result);
                                            calc_operand = result;
                                            calc_operator = None;
                                            calc_fresh = true;
                                        }
                                    }
                                    "." => {
                                        if calc_fresh { calc_display = "0.".to_string(); calc_fresh = false; }
                                        else if !calc_display.contains('.') { calc_display.push('.'); }
                                    }
                                    digit => {
                                        if calc_fresh { calc_display = digit.to_string(); calc_fresh = false; }
                                        else { calc_display.push_str(digit); }
                                    }
                                }
                            }

                            col_i += span;
                        }
                    }
                }
                "Components" => {
                    // Tabs at top
                    demo_tabs.pos_x = cx;
                    demo_tabs.pos_y = cy;
                    demo_tabs.width = cw;
                    demo_tabs.bg_color = theme.surface;
                    demo_tabs.active_color = theme.surface_hover;
                    demo_tabs.text_color = theme.text_dim;
                    demo_tabs.active_text_color = theme.text_primary;
                    demo_tabs.indicator_color = theme.accent;
                    demo_tabs.border_color = theme.separator;
                    demo_tabs.draw(&mut window);

                    let tab_cy = demo_tabs.content_y() + 1;
                    let tab_ch = ch.saturating_sub(demo_tabs.tab_height + 1);

                    match demo_tabs.selected_index() {
                        0 => {
                            // ── General tab: Checkbox, ProgressBar, Dropdown, Tooltip ──
                            let col1_x = cx + 20;
                            let col2_x = cx + cw / 2 + 10;

                            // Checkboxes
                            let sec1 = Text::new("CHECKBOXES", font.clone());
                            window.draw_text(col1_x, tab_cy + 12, &sec1, 10.0, &theme.text_dim);

                            demo_checkbox1.pos_x = col1_x;
                            demo_checkbox1.pos_y = tab_cy + 32;
                            demo_checkbox1.border_color = theme.separator;
                            demo_checkbox1.box_color = theme.surface;
                            demo_checkbox1.label = Some(Text::new("Enable notifications", font.clone()));
                            demo_checkbox1.label_color = theme.text_primary;
                            demo_checkbox1.draw(&mut window);

                            demo_checkbox2.pos_x = col1_x;
                            demo_checkbox2.pos_y = tab_cy + 60;
                            demo_checkbox2.border_color = theme.separator;
                            demo_checkbox2.box_color = theme.surface;
                            demo_checkbox2.label = Some(Text::new("Auto-save", font.clone()));
                            demo_checkbox2.label_color = theme.text_primary;
                            demo_checkbox2.draw(&mut window);

                            // Separator
                            window.draw_rect_f(col1_x, tab_cy + 90, cw / 2 - 40, 1, 0, &theme.separator, 0);

                            // Progress Bar
                            let sec2 = Text::new("PROGRESS BAR", font.clone());
                            window.draw_text(col1_x, tab_cy + 100, &sec2, 10.0, &theme.text_dim);

                            // Animate progress
                            demo_tween.update();
                            demo_progress.set_progress(demo_tween.value());
                            if demo_tween.done() {
                                if demo_tween.value() < 0.5 {
                                    demo_tween.set_target(1.0);
                                } else {
                                    demo_tween.set_target(0.0);
                                }
                            }

                            demo_progress.pos_x = col1_x;
                            demo_progress.pos_y = tab_cy + 120;
                            demo_progress.width = cw / 2 - 40;
                            demo_progress.track_color = theme.surface;
                            demo_progress.fill_color = theme.accent;
                            demo_progress.border_color = theme.separator;
                            demo_progress.draw(&mut window);

                            let pct_str = format!("{}%", (demo_progress.progress() * 100.0) as u32);
                            let pct_t = Text::new(&pct_str, font.clone());
                            window.draw_text(col1_x + demo_progress.width + 8, tab_cy + 120, &pct_t, 11.0, &theme.text_secondary);

                            // Separator
                            window.draw_rect_f(col1_x, tab_cy + 150, cw / 2 - 40, 1, 0, &theme.separator, 0);

                            // Dropdown
                            let sec3 = Text::new("DROPDOWN", font.clone());
                            window.draw_text(col1_x, tab_cy + 160, &sec3, 10.0, &theme.text_dim);

                            demo_dropdown.pos_x = col1_x;
                            demo_dropdown.pos_y = tab_cy + 180;
                            demo_dropdown.width = cw / 2 - 40;
                            demo_dropdown.bg_color = theme.surface;
                            demo_dropdown.bg_open_color = Color::rgba(theme.surface.r, theme.surface.g, theme.surface.b, 200);
                            demo_dropdown.border_color = theme.separator;
                            demo_dropdown.text_color = theme.text_primary;
                            demo_dropdown.hover_color = theme.highlight;
                            let sel_t = Text::new(&format!("Selected: {}", demo_dropdown.selected_text()), font.clone());
                            window.draw_text(col1_x, tab_cy + 216, &sel_t, 10.0, &theme.text_dim);

                            // ── Right column: ScrollArea, Tooltip, Shadow ──
                            let sec4 = Text::new("SCROLL AREA", font.clone());
                            window.draw_text(col2_x, tab_cy + 12, &sec4, 10.0, &theme.text_dim);

                            // ScrollArea with content
                            demo_scroll.pos_x = col2_x;
                            demo_scroll.pos_y = tab_cy + 30;
                            demo_scroll.width = cw / 2 - 30;
                            demo_scroll.set_content_height(400);
                            demo_scroll.scrollbar_color = Color::rgba(theme.text_dim.r, theme.text_dim.g, theme.text_dim.b, 120);
                            demo_scroll.begin_draw(&mut window);

                            let scroll_off = demo_scroll.offset() as usize;
                            let scroll_items = [
                                "Item 1: Welcome to minifb-ui",
                                "Item 2: SDF-based rendering",
                                "Item 3: Smooth anti-aliasing",
                                "Item 4: Alpha blending",
                                "Item 5: Background blur",
                                "Item 6: Box shadows",
                                "Item 7: Text wrapping",
                                "Item 8: Clipping regions",
                                "Item 9: Layout helpers",
                                "Item 10: Animation/Tween",
                                "Item 11: Theme support",
                                "Item 12: Checkbox widget",
                                "Item 13: ProgressBar widget",
                                "Item 14: Dropdown widget",
                                "Item 15: Tooltip widget",
                                "Item 16: ScrollArea widget",
                                "Item 17: Tabs widget",
                                "Item 18: ContextMenu widget",
                            ];

                            for (i, item) in scroll_items.iter().enumerate() {
                                let iy = tab_cy + 30 + i * 22;
                                if (iy as isize - scroll_off as isize) < (tab_cy + 30) as isize - 10 { continue; }
                                if iy.saturating_sub(scroll_off) > tab_cy + 30 + 160 { continue; }
                                let real_y = iy.saturating_sub(scroll_off);
                                let it = Text::new(item, font.clone());
                                let item_col = if i % 2 == 0 { theme.text_primary } else { theme.text_secondary };
                                window.draw_text(col2_x + 6, real_y + 2, &it, 11.0, &item_col);
                            }

                            demo_scroll.end_draw(&mut window);

                            // Tooltip demo (hover over a button-like area)
                            let tip_x = col2_x;
                            let tip_y = tab_cy + 210;
                            let tip_w = 120;
                            let tip_h = 28;
                            window.draw_rect_f(tip_x, tip_y, tip_w, tip_h, 6, &theme.surface, 0);
                            window.draw_rect(tip_x, tip_y, tip_w, tip_h, 6, &theme.separator);
                            let tip_label = Text::new("Hover me!", font.clone());
                            window.draw_text_centered(tip_x, tip_y, tip_w, tip_h, &tip_label, 11.0, &theme.text_primary);
                            demo_tooltip.draw_if_hovered(&mut window, tip_x, tip_y, tip_w, tip_h);

                            // Rounded box shadow demo
                            let sec5 = Text::new("BOX SHADOW", font.clone());
                            window.draw_text(col2_x, tab_cy + 256, &sec5, 10.0, &theme.text_dim);
                            let shad_x = col2_x + 20;
                            let shad_y = tab_cy + 276;
                            window.draw_box_shadow(shad_x as isize, shad_y as isize, 100, 50, 8, 3, 4, 0, 16, &Color::rgba(0, 0, 0, 80));
                            window.draw_rect_f(shad_x, shad_y, 100, 50, 8, &theme.surface, 0);
                            window.draw_rect(shad_x, shad_y, 100, 50, 8, &theme.separator);
                            let shadow_label = Text::new("Shadow", font.clone());
                            window.draw_text_centered(shad_x, shad_y, 100, 50, &shadow_label, 12.0, &theme.text_primary);

                            // Text wrapping demo
                            let sec6 = Text::new("TEXT WRAP", font.clone());
                            window.draw_text(col1_x, tab_cy + 250, &sec6, 10.0, &theme.text_dim);
                            let wrap_text = Text::new("This text is automatically wrapped within a max width, demonstrating the draw_text_wrapped feature of the window.", font.clone());
                            window.draw_text_wrapped(col1_x, tab_cy + 270, &wrap_text, 11.0, &theme.text_secondary, cw / 2 - 40);

                            // Dropdown drawn last so its open list overlays everything
                            demo_dropdown.draw(&mut window);
                        }
                        1 => {
                            // Display tab placeholder
                            let msg = Text::new("Display settings", font.clone());
                            window.draw_text_centered(cx, tab_cy, cw, tab_ch, &msg, 16.0, &theme.text_dim);
                        }
                        _ => {
                            // Audio tab placeholder
                            let msg = Text::new("Audio settings", font.clone());
                            window.draw_text_centered(cx, tab_cy, cw, tab_ch, &msg, 16.0, &theme.text_dim);
                        }
                    }
                }
                _ => {
                    // Generic placeholder
                    let placeholder = Text::new(&format!("{} - Coming Soon", aw.title), font.clone());
                    window.draw_text_centered(cx, cy, cw, ch, &placeholder, 16.0, &theme.text_dim);
                    window.draw_rect_f(cx + cw / 2 - 30, cy + ch / 2 - 60, 60, 60, 14, &aw.color, 0);
                    let sym_big = Text::new(&aw.symbol, font.clone());
                    window.draw_text_centered(cx + cw / 2 - 30, cy + ch / 2 - 60, 60, 60, &sym_big, 28.0, &Color::from(0xFFFFFF));
                }
            }
        }

        // Handle window interactions (click to focus, drag titlebar)
        if lmb_just && !any_overlay {
            for (wi, aw) in app_windows.iter_mut().enumerate().rev() {
                let wx = aw.x.max(0) as usize;
                let wy = aw.y.max(0) as usize;
                if mx >= wx as f32 && mx < (wx + aw.w) as f32
                    && my >= wy as f32 && my < (wy + aw.h) as f32
                {
                    // In titlebar and not on close button?
                    let close_x = wx + aw.w - 32;
                    let on_close = mx >= close_x as f32 && mx < (close_x + 20) as f32
                        && my >= (wy + 8) as f32 && my < (wy + 28) as f32;
                    if !on_close && my < (wy + titlebar_h) as f32 {
                        aw.dragging = true;
                        aw.drag_off_x = mx - aw.x as f32;
                        aw.drag_off_y = my - aw.y as f32;
                    }
                    bring_front_idx = Some(wi);
                    break;
                }
            }
        }

        // Process close/bring-to-front
        if let Some(idx) = close_idx {
            app_windows.remove(idx);
        } else if let Some(idx) = bring_front_idx {
            if idx < app_windows.len() - 1 {
                let win = app_windows.remove(idx);
                app_windows.push(win);
            }
        }

        // ═════════════════════════════════════════════════════
        //  TASKBAR
        // ═════════════════════════════════════════════════════
        window.draw_rect_f(0, taskbar_y, 1280, taskbar_h, 0, &theme.taskbar_bg, 20);
        window.draw_rect_f(0, taskbar_y, 1280, 1, 0, &theme.taskbar_border, 0);

        // ─── Start button ───────────────────────────────────
        let start_btn_x: usize = 8;
        let start_btn_y = taskbar_y + 8;
        let start_btn_w: usize = 40;
        let start_btn_h: usize = 32;

        let start_hovered = mx >= start_btn_x as f32 && mx < (start_btn_x + start_btn_w) as f32
            && my >= start_btn_y as f32 && my < (start_btn_y + start_btn_h) as f32;

        if start_hovered || start_open {
            window.draw_rect_f(start_btn_x, start_btn_y, start_btn_w, start_btn_h, 8, &theme.taskbar_icon_hover, 0);
        }

        let grid_cx = start_btn_x + start_btn_w / 2;
        let grid_cy = start_btn_y + start_btn_h / 2;
        let sq = 5usize; let gap = 2usize;
        window.draw_rect_f(grid_cx - sq - gap / 2, grid_cy - sq - gap / 2, sq, sq, 0, &theme.accent, 0);
        window.draw_rect_f(grid_cx + gap / 2,       grid_cy - sq - gap / 2, sq, sq, 0, &theme.accent, 0);
        window.draw_rect_f(grid_cx - sq - gap / 2, grid_cy + gap / 2,       sq, sq, 0, &theme.accent, 0);
        window.draw_rect_f(grid_cx + gap / 2,       grid_cy + gap / 2,       sq, sq, 0, &theme.accent, 0);

        if lmb_just && start_hovered {
            start_open = !start_open;
            calendar_open = false;
            notif_open = false;
        }

        // ─── Pinned taskbar icons ───────────────────────────
        let tb_icons_total_w = taskbar_icons.len() * 44;
        let tb_icons_x = (1280 - tb_icons_total_w) / 2;

        for (i, &(sym, color, name)) in taskbar_icons.iter().enumerate() {
            let ix = tb_icons_x + i * 44;
            let iy = taskbar_y + 6;
            let iw = 40usize;
            let ih = 36usize;

            let hovered = mx >= ix as f32 && mx < (ix + iw) as f32
                && my >= iy as f32 && my < (iy + ih) as f32;

            let is_running = app_windows.iter().any(|w| w.title == name);

            if hovered {
                window.draw_rect_f(ix, iy, iw, ih, 8, &theme.taskbar_icon_hover, 0);
            }

            window.draw_rect_f(ix + 8, iy + 6, 24, 24, 6, &color, 0);
            let s = Text::new(sym, font.clone());
            window.draw_text_centered(ix + 8, iy + 6, 24, 24, &s, 14.0, &Color::from(0xFFFFFF));

            // Running indicator
            if is_running {
                window.draw_rect_f(ix + 14, taskbar_y + taskbar_h - 4, 12, 2, 0, &theme.accent, 0);
            } else {
                window.draw_rect_f(ix + 17, taskbar_y + taskbar_h - 3, 6, 1, 0, &theme.text_dim, 0);
            }

            // Click to open/focus app
            if lmb_just && hovered && !any_overlay {
                open_app(&mut app_windows, name, color, sym);
            }
        }

        // ─── System tray ────────────────────────────────────
        dark_mode.pos_x = 1280 - 280;
        dark_mode.pos_y = taskbar_y + 13;
        dark_mode.draw(&mut window);

        // Notification bell
        let bell_x: usize = 1280 - 220;
        let bell_y = taskbar_y + 10;
        let bell_w = 28usize;
        let bell_h = 28usize;
        let bell_hovered = mx >= bell_x as f32 && mx < (bell_x + bell_w) as f32
            && my >= bell_y as f32 && my < (bell_y + bell_h) as f32;
        if bell_hovered || notif_open {
            window.draw_rect_f(bell_x, bell_y, bell_w, bell_h, 6, &theme.taskbar_icon_hover, 0);
        }
        let bell_t = Text::new("!", font.clone());
        window.draw_text_centered(bell_x, bell_y, bell_w, bell_h, &bell_t, 14.0, &theme.text_secondary);
        if !notifications.is_empty() {
            window.draw_ellipse_f((bell_x + bell_w - 4) as isize, (bell_y + 4) as isize, 5, 5, &theme.danger, 0);
            let badge = Text::new(&format!("{}", notifications.len()), font.clone());
            window.draw_text_centered(bell_x + bell_w - 9, bell_y - 1, 10, 10, &badge, 8.0, &Color::from(0xFFFFFF));
        }

        if lmb_just && bell_hovered {
            notif_open = !notif_open;
            start_open = false;
            calendar_open = false;
        }

        window.draw_rect_f(1280 - 185, taskbar_y + 12, 1, 24, 0, &theme.separator, 0);

        // Volume
        let vol_x: usize = 1280 - 175;
        let vol_y = taskbar_y + 10;
        let vol_hov = mx >= vol_x as f32 && mx < (vol_x + 28) as f32
            && my >= vol_y as f32 && my < (vol_y + 28) as f32;
        if vol_hov {
            window.draw_rect_f(vol_x, vol_y, 28, 28, 6, &theme.taskbar_icon_hover, 0);
        }
        let vol_t = Text::new("V", font.clone());
        window.draw_text_centered(vol_x, vol_y, 28, 28, &vol_t, 12.0, &theme.text_secondary);

        // Wi-Fi
        let wifi_x: usize = 1280 - 145;
        let wifi_y = taskbar_y + 10;
        let wifi_hov = mx >= wifi_x as f32 && mx < (wifi_x + 28) as f32
            && my >= wifi_y as f32 && my < (wifi_y + 28) as f32;
        if wifi_hov {
            window.draw_rect_f(wifi_x, wifi_y, 28, 28, 6, &theme.taskbar_icon_hover, 0);
        }
        let wifi_t = Text::new("~", font.clone());
        window.draw_text_centered(wifi_x, wifi_y, 28, 28, &wifi_t, 14.0, &theme.text_secondary);

        // Battery
        let bat_x: usize = 1280 - 115;
        let bat_y = taskbar_y + 14;
        window.draw_rect(bat_x, bat_y, 22, 12, 2, &theme.text_secondary);
        window.draw_rect_f(bat_x + 22, bat_y + 3, 2, 6, 0, &theme.text_secondary, 0);
        window.draw_rect_f(bat_x + 2, bat_y + 2, 14, 8, 1, &theme.success, 0);

        window.draw_rect_f(1280 - 82, taskbar_y + 12, 1, 24, 0, &theme.separator, 0);

        // Clock
        let clock_x: usize = 1280 - 76;
        let clock_y = taskbar_y + 8;
        let clock_w: usize = 68;
        let clock_h: usize = 32;
        let clock_hovered = mx >= clock_x as f32 && mx < (clock_x + clock_w) as f32
            && my >= clock_y as f32 && my < (clock_y + clock_h) as f32;
        if clock_hovered || calendar_open {
            window.draw_rect_f(clock_x, clock_y, clock_w, clock_h, 6, &theme.taskbar_icon_hover, 0);
        }

        let t_time = Text::new("14:37", font.clone());
        let t_date = Text::new("05/11", font.clone());
        window.draw_text(clock_x + 8, clock_y + 2, &t_time, 13.0, &theme.text_primary);
        window.draw_text(clock_x + 10, clock_y + 18, &t_date, 10.0, &theme.text_secondary);

        if lmb_just && clock_hovered {
            calendar_open = !calendar_open;
            start_open = false;
            notif_open = false;
        }

        // ═════════════════════════════════════════════════════
        //  START MENU
        // ═════════════════════════════════════════════════════
        if start_open {
            let sm_w: usize = 540;
            let sm_h: usize = 590;
            let sm_x: usize = 12;
            let sm_y: usize = taskbar_y - sm_h - 8;
            let sm_r: usize = 14;

            window.draw_rect_f(sm_x, sm_y, sm_w, sm_h, sm_r, &theme.panel_tint, 35);
            window.draw_rect(sm_x, sm_y, sm_w, sm_h, sm_r, &theme.panel_border);

            // Search bar
            search_input.pos_x = sm_x + 20;
            search_input.pos_y = sm_y + 18;
            search_input.bg_col_idle = theme.surface;
            search_input.bg_col_editing = theme.surface_hover;
            search_input.border_col_idle = theme.separator;
            search_input.border_col_editing = theme.accent;
            search_input.text_col_idle = theme.text_primary;
            search_input.text_col_editing = theme.text_primary;
            search_input.width = sm_w - 40;
            search_input.draw(&mut window);

            // Pinned section
            let pinned_y = sm_y + 68;
            window.draw_text(sm_x + 24, pinned_y, &t_pinned, 10.0, &theme.text_dim);

            let grid_cols = 4;
            let grid_cell_w = (sm_w - 48) / grid_cols;
            let grid_cell_h: usize = 76;
            let grid_x = sm_x + 24;
            let grid_y = pinned_y + 22;

            for (i, app) in pinned_apps.iter().enumerate() {
                let col = i % grid_cols;
                let row = i / grid_cols;
                let ax = grid_x + col * grid_cell_w + grid_cell_w / 2;
                let ay = grid_y + row * grid_cell_h + 22;

                let app_hov = mx >= (ax - grid_cell_w / 2 + 4) as f32 && mx < (ax + grid_cell_w / 2 - 4) as f32
                    && my >= (grid_y + row * grid_cell_h + 2) as f32 && my < (grid_y + (row + 1) * grid_cell_h - 2) as f32;

                if app_hov {
                    window.draw_rect_f(
                        ax - grid_cell_w / 2 + 4, grid_y + row * grid_cell_h + 2,
                        grid_cell_w - 8, grid_cell_h - 4, 8,
                        &theme.highlight,
                        0,
                    );
                }

                window.draw_rect_f(ax - 18, ay - 18, 36, 36, 10, &app.color, 0);
                let sym = Text::new(app.symbol, font.clone());
                window.draw_text_centered(ax - 18, ay - 18, 36, 36, &sym, 18.0, &Color::from(0xFFFFFF));

                let name = Text::new(app.name, font.clone());
                let nw = name.get_width(10.0);
                window.draw_text(ax - nw / 2, ay + 22, &name, 10.0, &theme.text_primary);

                // Click to open app
                if lmb_just && app_hov {
                    open_app(&mut app_windows, app.name, app.color, app.symbol);
                    start_open = false;
                }
            }

            // Separator
            let sep_y = grid_y + ((pinned_apps.len() + grid_cols - 1) / grid_cols) * grid_cell_h + 4;
            window.draw_rect_f(sm_x + 20, sep_y, sm_w - 40, 1, 0, &theme.separator, 0);

            // Recent section
            let recent_y = sep_y + 14;
            window.draw_text(sm_x + 24, recent_y, &t_recent, 10.0, &theme.text_dim);

            let recent_start_y = recent_y + 22;
            let recent_row_h: usize = 40;

            for (i, &(name, folder, time)) in recent_items.iter().enumerate() {
                let ry = recent_start_y + i * recent_row_h;
                let row_hov = mx >= (sm_x + 12) as f32 && mx < (sm_x + sm_w - 12) as f32
                    && my >= ry as f32 && my < (ry + recent_row_h) as f32;

                if row_hov {
                    window.draw_rect_f(sm_x + 12, ry, sm_w - 24, recent_row_h, 8, &theme.highlight, 0);
                }

                let file_icon_color = if name.ends_with(".pdf") { Color::from(0xE05555) }
                    else if name.ends_with(".png") { Color::from(0x8B5CF6) }
                    else if name.ends_with(".rs") { Color::from(0xF97316) }
                    else { Color::from(0x64748B) };
                window.draw_rect_f(sm_x + 24, ry + 8, 24, 24, 5, &file_icon_color, 0);

                let ext = if let Some(pos) = name.rfind('.') { &name[pos + 1..] } else { "?" };
                let ext_t = Text::new(&ext[..ext.len().min(3)], font.clone());
                window.draw_text_centered(sm_x + 24, ry + 8, 24, 24, &ext_t, 8.0, &Color::from(0xFFFFFF));

                let n = Text::new(name, font.clone());
                window.draw_text(sm_x + 58, ry + 6, &n, 12.0, &theme.text_primary);
                let f = Text::new(folder, font.clone());
                window.draw_text(sm_x + 58, ry + 22, &f, 10.0, &theme.text_dim);

                let tt = Text::new(time, font.clone());
                let tw = tt.get_width(10.0);
                window.draw_text(sm_x + sm_w - 24 - tw, ry + 14, &tt, 10.0, &theme.text_dim);
            }

            // Bottom bar
            let bot_y = sm_y + sm_h - 50;
            window.draw_rect_f(sm_x + 20, bot_y, sm_w - 40, 1, 0, &theme.separator, 0);

            window.draw_ellipse_f((sm_x + 40) as isize, (bot_y + 26) as isize, 14, 14, &theme.accent, 0);
            let user_init = Text::new("U", font.clone());
            window.draw_text_centered(sm_x + 26, bot_y + 12, 28, 28, &user_init, 14.0, &Color::from(0xFFFFFF));
            let user_name = Text::new("User", font.clone());
            window.draw_text(sm_x + 62, bot_y + 18, &user_name, 13.0, &theme.text_primary);

            let pwr_x = sm_x + sm_w - 50;
            let pwr_y = bot_y + 12;
            let pwr_hov = mx >= pwr_x as f32 && mx < (pwr_x + 30) as f32
                && my >= pwr_y as f32 && my < (pwr_y + 28) as f32;
            if pwr_hov {
                window.draw_rect_f(pwr_x, pwr_y, 30, 28, 6, &Color::rgba(theme.danger.r, theme.danger.g, theme.danger.b, 40), 0);
            }
            let pwr_t = Text::new("O", font.clone());
            window.draw_text_centered(pwr_x, pwr_y, 30, 28, &pwr_t, 14.0,
                &if pwr_hov { theme.danger } else { theme.text_secondary });

            let in_start = mx >= sm_x as f32 && mx < (sm_x + sm_w) as f32
                && my >= sm_y as f32 && my < (sm_y + sm_h) as f32;
            if lmb_just && !in_start && !start_hovered {
                start_open = false;
            }
            if window.window.is_key_pressed(Key::Escape, KeyRepeat::No) {
                start_open = false;
            }
        }

        // ═════════════════════════════════════════════════════
        //  CALENDAR
        // ═════════════════════════════════════════════════════
        if calendar_open {
            let cal_w: usize = 280;
            let cal_h: usize = 320;
            let cal_x: usize = 1280 - cal_w - 8;
            let cal_y: usize = taskbar_y - cal_h - 8;
            let cal_r: usize = 14;

            window.draw_rect_f(cal_x, cal_y, cal_w, cal_h, cal_r, &theme.panel_tint, 35);
            window.draw_rect(cal_x, cal_y, cal_w, cal_h, cal_r, &theme.panel_border);

            let month_t = Text::new(cal_month, font.clone());
            window.draw_text(cal_x + 20, cal_y + 16, &month_t, 16.0, &theme.text_primary);

            let larr = Text::new("<", font.clone());
            let rarr = Text::new(">", font.clone());
            window.draw_text(cal_x + cal_w - 52, cal_y + 18, &larr, 14.0, &theme.text_secondary);
            window.draw_text(cal_x + cal_w - 30, cal_y + 18, &rarr, 14.0, &theme.text_secondary);

            let cell_w = (cal_w - 32) / 7;
            let hdr_y = cal_y + 48;
            for (j, day) in cal_day_hdrs.iter().enumerate() {
                let dx = cal_x + 16 + j * cell_w;
                let d = Text::new(day, font.clone());
                window.draw_text_centered(dx, hdr_y, cell_w, 16, &d, 10.0, &theme.text_dim);
            }
            window.draw_rect_f(cal_x + 16, hdr_y + 20, cal_w - 32, 1, 0, &theme.separator, 0);

            let grid_start_y = hdr_y + 28;
            let row_h: usize = 32;

            for day in 1..=cal_num_days {
                let cell_idx = day - 1 + cal_start_offset;
                let col = cell_idx % 7;
                let row = cell_idx / 7;
                let dx = cal_x + 16 + col * cell_w;
                let dy = grid_start_y + row * row_h;

                let day_hov = mx >= dx as f32 && mx < (dx + cell_w) as f32
                    && my >= dy as f32 && my < (dy + row_h) as f32;

                if day == cal_today {
                    window.draw_ellipse_f((dx + cell_w / 2) as isize, (dy + row_h / 2) as isize, 13, 13, &theme.accent, 0);
                } else if day_hov {
                    window.draw_ellipse_f((dx + cell_w / 2) as isize, (dy + row_h / 2) as isize, 13, 13, &theme.highlight, 0);
                }

                let day_str = format!("{}", day);
                let dt = Text::new(&day_str, font.clone());
                let text_col = if day == cal_today { Color::from(0xFFFFFF) } else { theme.text_primary };
                window.draw_text_centered(dx, dy, cell_w, row_h, &dt, 12.0, &text_col);
            }

            let today_label = Text::new("Today: May 11, 2026", font.clone());
            window.draw_text(cal_x + 20, cal_y + cal_h - 30, &today_label, 10.0, &theme.text_dim);

            // Event dot on day 15
            let ev_cell = 15 - 1 + cal_start_offset;
            let ev_x = cal_x + 16 + (ev_cell % 7) * cell_w + cell_w / 2;
            let ev_y = grid_start_y + (ev_cell / 7) * row_h + row_h / 2 + 11;
            window.draw_ellipse_f(ev_x as isize, ev_y as isize, 2, 2, &theme.warning, 0);

            let in_cal = mx >= cal_x as f32 && mx < (cal_x + cal_w) as f32
                && my >= cal_y as f32 && my < (cal_y + cal_h) as f32;
            if lmb_just && !in_cal && !clock_hovered { calendar_open = false; }
            if window.window.is_key_pressed(Key::Escape, KeyRepeat::No) { calendar_open = false; }
        }

        // ═════════════════════════════════════════════════════
        //  NOTIFICATION CENTER
        // ═════════════════════════════════════════════════════
        if notif_open {
            let np_w: usize = 340;
            let notif_card_h: usize = 74;
            let np_h = 52 + notifications.len().max(1) * (notif_card_h + 8) + 10;
            let np_x: usize = 1280 - np_w - 100;
            let np_y: usize = taskbar_y - np_h - 8;
            let np_r: usize = 14;

            window.draw_rect_f(np_x, np_y, np_w, np_h, np_r, &theme.panel_tint, 35);
            window.draw_rect(np_x, np_y, np_w, np_h, np_r, &theme.panel_border);

            window.draw_text(np_x + 20, np_y + 16, &t_notif_title, 14.0, &theme.text_primary);

            let clear_w = t_clear.get_width(10.0);
            let clear_x = np_x + np_w - clear_w - 20;
            let clear_hov = mx >= clear_x as f32 && mx < (clear_x + clear_w) as f32
                && my >= (np_y + 16) as f32 && my < (np_y + 30) as f32;
            window.draw_text(clear_x, np_y + 18, &t_clear, 10.0,
                &if clear_hov { theme.accent } else { theme.text_dim });

            if lmb_just && clear_hov {
                notifications.clear();
            }

            window.draw_rect_f(np_x + 16, np_y + 42, np_w - 32, 1, 0, &theme.separator, 0);

            if notifications.is_empty() {
                let empty_t = Text::new("No notifications", font.clone());
                window.draw_text_centered(np_x, np_y + 52, np_w, 40, &empty_t, 12.0, &theme.text_dim);
            }

            let mut dismiss_idx: Option<usize> = None;
            let notif_start_y = np_y + 52;

            for (i, &(title, body, time, color)) in notifications.iter().enumerate() {
                let ny = notif_start_y + i * (notif_card_h + 8);
                let n_hov = mx >= (np_x + 12) as f32 && mx < (np_x + np_w - 12) as f32
                    && my >= ny as f32 && my < (ny + notif_card_h) as f32;

                window.draw_rect_f(
                    np_x + 12, ny, np_w - 24, notif_card_h, 10,
                    &if n_hov { theme.surface_hover } else { theme.surface },
                    0,
                );

                window.draw_rect_f(np_x + 14, ny + 12, 3, notif_card_h - 24, 2, &color, 0);

                let nt = Text::new(title, font.clone());
                window.draw_text(np_x + 28, ny + 12, &nt, 12.0, &theme.text_primary);
                let nb = Text::new(body, font.clone());
                window.draw_text(np_x + 28, ny + 30, &nb, 11.0, &theme.text_secondary);
                let ntime = Text::new(time, font.clone());
                window.draw_text(np_x + 28, ny + 50, &ntime, 9.0, &theme.text_dim);

                let x_x = np_x + np_w - 38;
                let x_y = ny + 8;
                let x_hov = mx >= x_x as f32 && mx < (x_x + 16) as f32
                    && my >= x_y as f32 && my < (x_y + 16) as f32;
                let xt = Text::new("x", font.clone());
                window.draw_text_centered(x_x, x_y, 16, 16, &xt, 10.0,
                    &if x_hov { theme.danger } else { theme.text_dim });

                if lmb_just && x_hov {
                    dismiss_idx = Some(i);
                }
            }

            if let Some(idx) = dismiss_idx {
                notifications.remove(idx);
            }

            let in_np = mx >= np_x as f32 && mx < (np_x + np_w) as f32
                && my >= np_y as f32 && my < (np_y + np_h) as f32;
            if lmb_just && !in_np && !bell_hovered { notif_open = false; }
            if window.window.is_key_pressed(Key::Escape, KeyRepeat::No) { notif_open = false; }
        }

        prev_lmb = lmb;
        prev_rmb = rmb;
        window.update();
    }
}

fn calc_eval(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        'x' => a * b,
        '/' => if b != 0.0 { a / b } else { 0.0 },
        _ => b,
    }
}

fn format_calc(v: f64) -> String {
    if v == v.floor() && v.abs() < 1e12 {
        format!("{}", v as i64)
    } else {
        let s = format!("{:.8}", v);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
