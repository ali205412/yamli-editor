use crate::config::Config;
use crate::themes;
use webkit2gtk::{gio::Cancellable, traits::*, WebView};

pub fn setup_webview() -> WebView {
    let webview = WebView::new();

    // Enable JavaScript
    let settings = webkit2gtk::Settings::new();
    settings.set_enable_javascript(true);
    settings.set_javascript_can_access_clipboard(true);
    settings.set_enable_developer_extras(true);
    webview.set_settings(&settings);

    webview
}

pub fn reload_webview(webview: &WebView, config: &Config) {
    // Instead of immediately reloading, let's wait for the HTML to load first
    let html = generate_html(config);

    // Clone webview for the load finished handler
    let webview_clone = webview.clone();

    // Connect to load-changed signal
    webview.connect_load_changed(move |_, event| {
        use webkit2gtk::LoadEvent;
        if event == LoadEvent::Finished {
            // Re-focus the editor after load
            let js = "document.getElementById('editor').focus();";
            webview_clone.run_javascript(js, None::<&Cancellable>, |_| {});
        }
    });

    // Load the new HTML
    webview.load_html(&html, None);
}

fn generate_html(config: &Config) -> String {
    let theme = themes::get_theme(&config.theme.name);

    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <link href="https://fonts.googleapis.com/css2?family=Noto+Naskh+Arabic:wght@400;500;600&family=IBM+Plex+Sans+Arabic:wght@400;500&display=swap" rel="stylesheet">
            <script type="text/javascript" src="http://api.yamli.com/js/yamli_api.js"></script>
            <style>
                * {{
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }}

                body {{
                    margin: 0;
                    padding: {padding}px;
                    background: {background};
                    color: {text};
                    height: 100vh;
                    font-family: '{font_family}', 'Noto Naskh Arabic', system-ui, -apple-system, sans-serif;
                }}

                #editor {{
                    width: 100%;
                    height: calc(100vh - {padding_double}px);
                    background: {editor_bg};
                    color: {text};
                    border: none;
                    border-radius: {border_radius}px;
                    font-family: '{font_family}', 'Noto Naskh Arabic', system-ui, -apple-system, sans-serif;
                    font-size: {font_size}px;
                    font-weight: 500;
                    line-height: {line_height};
                    resize: none;
                    outline: none;
                    padding: {padding}px;
                    box-shadow: 0 4px 20px {popup_shadow};
                    direction: rtl;
                }}

                #editor:focus {{
                    background: {editor_bg_focus};
                }}

                /* Hide Yamli settings menu and hints */
                .yamliapi_settingsDiv,
                div[style*="background: url"][style*="bulb.gif"],
                div[style*="border: 2px solid red"],
                div[style*="rgb(255, 255, 223)"],
                div[style*="background-color: rgb(255, 255, 223)"] {{
                    display: none !important;
                    opacity: 0 !important;
                    visibility: hidden !important;
                    height: 0 !important;
                    width: 0 !important;
                    margin: 0 !important;
                    padding: 0 !important;
                    border: none !important;
                }}

                /* Style Yamli popup */
                .yamliapi_menuBorder {{
                    background: {popup_bg} !important;
                    border: 1px solid {popup_border} !important;
                    border-radius: {border_radius}px !important;
                    box-shadow: 0 8px 30px {popup_shadow} !important;
                    padding: 0 !important;
                }}

                .yamliapi_menuPanel {{
                    background: {popup_bg} !important;
                    padding: 1px !important;
                }}

                .yamliapi_menuContent {{
                    background: {popup_bg} !important;
                    padding: 4px !important;
                    font-family: '{font_family}', 'Noto Naskh Arabic', system-ui, -apple-system, sans-serif !important;
                }}

                /* Style suggestion items */
                .yamliapi_menuContent div[style*="text-align: right"] {{
                    padding: 8px 12px !important;
                    margin: 2px 0 !important;
                    border-radius: 4px !important;
                    transition: all 0.2s ease !important;
                    color: {suggestion_text} !important;
                    font-weight: 500 !important;
                    cursor: pointer !important;
                    background: transparent !important;
                    font-family: '{font_family}', 'Noto Naskh Arabic', system-ui, -apple-system, sans-serif !important;
                    font-size: {font_size}px !important;
                    white-space: nowrap !important;
                    direction: rtl !important;
                }}

                .yamliapi_menuContent div[style*="text-align: right"]:hover {{
                    background: {suggestion_bg_hover} !important;
                    transform: translateX(-2px) !important;
                }}

                /* Style selected item */
                .yamliapi_menuContent div[style*="background-color: rgb(198, 216, 255)"] {{
                    background: {suggestion_bg_selected} !important;
                    color: {suggestion_text_selected} !important;
                }}

                /* Style "show more" link */
                .yamliapi_menuContent div[style*="color: rgb(17, 42, 187)"] {{
                    color: {suggestion_text_selected} !important;
                    font-size: {font_size_small}px !important;
                    padding: 6px 12px !important;
                }}

                /* Hide Yamli branding and ads */
                div[style*="background-color: rgb(221, 228, 238)"],
                iframe[id^="yamli_spid"] {{
                    display: none !important;
                }}

                /* Scrollbar styling */
                #editor::-webkit-scrollbar {{
                    width: 12px;
                }}

                #editor::-webkit-scrollbar-track {{
                    background: {scrollbar_track};
                    border-radius: 6px;
                }}

                #editor::-webkit-scrollbar-thumb {{
                    background: {scrollbar_thumb};
                    border-radius: 6px;
                    border: 3px solid {scrollbar_track};
                }}

                #editor::-webkit-scrollbar-thumb:hover {{
                    background: {scrollbar_thumb_hover};
                }}

                /* Selection styling */
                #editor::selection {{
                    background: {suggestion_bg_hover};
                }}
            </style>
        </head>
        <body>
            <textarea id="editor" placeholder="ابدأ الكتابة هنا..."></textarea>
            <script>
                function initYamli() {{
                    if (typeof Yamli === 'object') {{
                        Yamli.init({{
                            uiLanguage: 'ar',
                            startMode: 'on',
                            settingsPlacement: 'none',
                            showDefaultPickHint: false,
                            showHint: false
                        }});
                        Yamli.yamlify('editor');
                        document.getElementById('editor').focus();

                        // Remove branding elements and hints periodically
                        setInterval(() => {{
                            document.querySelectorAll(`
                                div[style*="background-color: rgb(221, 228, 238)"],
                                iframe[id^="yamli_spid"],
                                div[style*="background: url"][style*="bulb.gif"],
                                div[style*="border: 2px solid red"],
                                div[style*="rgb(255, 255, 223)"],
                                div[style*="background-color: rgb(255, 255, 223)"]
                            `).forEach(el => el.remove());
                        }}, 100);
                    }} else {{
                        setTimeout(initYamli, 100);
                    }}
                }}

                initYamli();
            </script>
        </body>
        </html>
        "#,
        background = theme.background,
        editor_bg = theme.editor_bg,
        editor_bg_focus = theme.editor_bg_focus,
        text = theme.text,
        popup_bg = theme.popup_bg,
        popup_border = theme.popup_border,
        popup_shadow = theme.popup_shadow,
        suggestion_text = theme.suggestion_text,
        suggestion_bg_hover = theme.suggestion_bg_hover,
        suggestion_bg_selected = theme.suggestion_bg_selected,
        suggestion_text_selected = theme.suggestion_text_selected,
        scrollbar_track = theme.scrollbar_track,
        scrollbar_thumb = theme.scrollbar_thumb,
        scrollbar_thumb_hover = theme.scrollbar_thumb_hover,
        font_family = config.font.family,
        font_size = config.font.size,
        font_size_small = config.font.size - 2,
        line_height = config.font.line_height,
        padding = config.editor.padding,
        padding_double = config.editor.padding * 2,
        border_radius = config.editor.border_radius,
    )
}
