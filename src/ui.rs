use crate::config::Config;
use crate::webview::reload_webview;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use webkit2gtk::WebView;

pub fn create_preferences_window(config: Rc<RefCell<Config>>, main_webview: WebView) {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Preferences");
    window.set_default_size(400, 300);
    window.set_modal(true);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    window.add(&vbox);

    // Theme selection
    let theme_label = gtk::Label::new(Some("Theme:"));
    theme_label.set_halign(gtk::Align::Start);
    vbox.pack_start(&theme_label, false, false, 0);

    let theme_combo = gtk::ComboBoxText::new();
    for theme in &[
        "Tokyo Night",
        "Dracula",
        "Nord",
        "Monokai",
        "Solarized Dark",
        "Pastel",
        "Light",
    ] {
        theme_combo.append_text(theme);
    }
    let current_theme = &config.borrow().theme.name;
    let active_index = match current_theme.as_str() {
        "dracula" => 1,
        "nord" => 2,
        "monokai" => 3,
        "solarized-dark" => 4,
        "pastel" => 5,
        "light" => 6,
        _ => 0,
    };
    theme_combo.set_active(Some(active_index));
    vbox.pack_start(&theme_combo, false, false, 0);

    // Font family entry
    let font_label = gtk::Label::new(Some("Font Family:"));
    font_label.set_halign(gtk::Align::Start);
    vbox.pack_start(&font_label, false, false, 0);

    let font_entry = gtk::Entry::new();
    font_entry.set_text(&config.borrow().font.family);
    vbox.pack_start(&font_entry, false, false, 0);

    // Font size
    let size_label = gtk::Label::new(Some("Font Size:"));
    size_label.set_halign(gtk::Align::Start);
    vbox.pack_start(&size_label, false, false, 0);

    let size_spin = gtk::SpinButton::with_range(8.0, 72.0, 1.0);
    size_spin.set_value(config.borrow().font.size as f64);
    vbox.pack_start(&size_spin, false, false, 0);

    // Line height
    let line_height_label = gtk::Label::new(Some("Line Height:"));
    line_height_label.set_halign(gtk::Align::Start);
    vbox.pack_start(&line_height_label, false, false, 0);

    let line_height_spin = gtk::SpinButton::with_range(1.0, 3.0, 0.1);
    line_height_spin.set_value(config.borrow().font.line_height as f64);
    vbox.pack_start(&line_height_spin, false, false, 0);

    // Padding
    let padding_label = gtk::Label::new(Some("Padding:"));
    padding_label.set_halign(gtk::Align::Start);
    vbox.pack_start(&padding_label, false, false, 0);

    let padding_spin = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
    padding_spin.set_value(config.borrow().editor.padding as f64);
    vbox.pack_start(&padding_spin, false, false, 0);

    // Border radius
    let radius_label = gtk::Label::new(Some("Border Radius:"));
    radius_label.set_halign(gtk::Align::Start);
    vbox.pack_start(&radius_label, false, false, 0);

    let radius_spin = gtk::SpinButton::with_range(0.0, 50.0, 1.0);
    radius_spin.set_value(config.borrow().editor.border_radius as f64);
    vbox.pack_start(&radius_spin, false, false, 0);

    // Save button
    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    button_box.set_layout(gtk::ButtonBoxStyle::End);
    button_box.set_spacing(10);
    vbox.pack_start(&button_box, false, false, 10);

    let save_button = gtk::Button::with_label("Save");
    button_box.add(&save_button);

    let config_clone = config.clone();
    let window_clone = window.clone();
    let main_webview = main_webview.clone();

    save_button.connect_clicked(move |_| {
        let old_theme;
        let old_font;
        let old_size;
        let old_line_height;
        let old_padding;
        let old_radius;

        {
            let config = config_clone.borrow();
            old_theme = config.theme.name.clone();
            old_font = config.font.family.clone();
            old_size = config.font.size;
            old_line_height = config.font.line_height;
            old_padding = config.editor.padding;
            old_radius = config.editor.border_radius;
        }

        let mut config = config_clone.borrow_mut();

        // Update config
        config.theme.name = match theme_combo.active_text().unwrap().as_str() {
            "Dracula" => "dracula",
            "Nord" => "nord",
            "Monokai" => "monokai",
            "Solarized Dark" => "solarized-dark",
            "Pastel" => "pastel",
            "Light" => "light",
            _ => "tokyo-night",
        }
        .to_string();

        config.font.family = font_entry.text().to_string();
        config.font.size = size_spin.value() as u32;
        config.font.line_height = line_height_spin.value() as f32;
        config.editor.padding = padding_spin.value() as u32;
        config.editor.border_radius = radius_spin.value() as u32;

        // Only save and reload if something actually changed
        if old_theme != config.theme.name
            || old_font != config.font.family
            || old_size != config.font.size
            || old_line_height != config.font.line_height
            || old_padding != config.editor.padding
            || old_radius != config.editor.border_radius
        {
            // Save config to file
            config.save().unwrap();

            // Reload webview
            reload_webview(&main_webview, &config);
        }

        window_clone.close();
    });

    window.show_all();
}

pub fn setup_main_window(config: Rc<RefCell<Config>>, webview: WebView) -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Yamli Editor");
    window.set_default_size(1000, 700);

    // Create a vertical box
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);

    // Add keyboard shortcut for preferences
    let webview_clone = webview.clone();
    let config_clone = config.clone();
    window.connect_key_press_event(move |_, key| {
        let keyval = key.keyval();
        let state = key.state();

        if state.contains(gdk::ModifierType::CONTROL_MASK) && keyval == gdk::keys::constants::p {
            create_preferences_window(config_clone.clone(), webview_clone.clone());
            Inhibit(true)
        } else {
            Inhibit(false)
        }
    });

    webview.set_vexpand(true);
    vbox.pack_start(&webview, true, true, 0);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window
}
