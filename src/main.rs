mod config;
mod themes;
mod ui;
mod webview;

use config::Config;
use gtk::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    gtk::init().unwrap();

    // Load configuration
    let config = Rc::new(RefCell::new(Config::load()));

    // Setup WebView
    let webview = webview::setup_webview();

    // Initial load
    webview::reload_webview(&webview, &config.borrow());

    // Setup main window
    let window = ui::setup_main_window(config.clone(), webview.clone());
    window.show_all();

    // Setup file watcher for config changes only if config file exists
    if Path::new("config.toml").exists() {
        let config_clone = config.clone();
        let webview_clone = webview.clone();

        let (tx, rx) = channel();
        if let Ok(mut watcher) = watcher(tx, Duration::from_millis(500)) {
            if watcher
                .watch("config.toml", RecursiveMode::NonRecursive)
                .is_ok()
            {
                // Use a more efficient idle callback that doesn't block the UI
                let mut last_reload = std::time::Instant::now();
                glib::source::idle_add_local(move || {
                    if let Ok(DebouncedEvent::Write(_)) = rx.try_recv() {
                        // Only reload if enough time has passed since last reload
                        let now = std::time::Instant::now();
                        if now.duration_since(last_reload) > Duration::from_millis(500) {
                            last_reload = now;

                            // Schedule the reload on the main thread
                            let config = config_clone.borrow_mut();
                            webview::reload_webview(&webview_clone, &config);
                        }
                    }
                    glib::Continue(true)
                });
            }
        }
    }

    // Start the GTK main loop
    gtk::main();
}
