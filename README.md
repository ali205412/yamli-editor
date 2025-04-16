# Yamli Editor

[![CI](https://github.com/ali205412/yamli-editor/actions/workflows/ci.yml/badge.svg)](https://github.com/ali205412/yamli-editor/actions/workflows/ci.yml)
[![Release](https://github.com/ali205412/yamli-editor/actions/workflows/release.yml/badge.svg)](https://github.com/ali205412/yamli-editor/actions/workflows/release.yml)
[![AUR version](https://img.shields.io/aur/version/yamli-editor)](https://aur.archlinux.org/packages/yamli-editor/)

A modern, native Yamli editor for Wayland that lets you type Arabic using Latin characters. Built with GTK and WebKit, featuring a clean interface and customizable themes.

## Features

- Native Wayland support
- Multiple themes (Tokyo Night, Dracula, Nord, Monokai, Solarized Dark)
- Real-time Arabic transliteration
- Auto-saves content locally
- Clean, distraction-free interface
- Customizable fonts, sizes, and spacing
- Fast and lightweight

## Installation

### Arch Linux (AUR)

```bash
yay -S yamli-editor
```
or
```bash
paru -S yamli-editor
```

### Fedora (COPR)

Enable the COPR repository and install the package:

```bash
dnf copr enable yuikotegawa/yamli-editor
dnf install yamli-editor
```

More information: [https://copr.fedorainfracloud.org/coprs/yuikotegawa/yamli-editor/](https://copr.fedorainfracloud.org/coprs/yuikotegawa/yamli-editor/)

### Debian/Ubuntu

Download the latest .deb package from the [releases page](https://github.com/ali205412/yamli-editor/releases) and install it:
```bash
sudo dpkg -i yamli-editor-*.deb
sudo apt-get install -f  # Install dependencies if needed
```

### Building from Source

Requirements:
- Rust (1.70.0 or later)
- GTK3 development files
- WebKit2GTK development files

```bash
# Install dependencies (Arch Linux)
sudo pacman -S gtk3 webkit2gtk base-devel

# Install dependencies (Debian/Ubuntu)
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev build-essential

# Clone the repository
git clone https://github.com/ali205412/yamli-editor.git
cd yamli-editor

# Build and run
cargo build --release
./target/release/yamli-editor
```

## Usage

1. Launch the application
2. Start typing in Latin characters - they will be automatically converted to Arabic
3. Press Ctrl+P to open preferences and customize the editor

### Keyboard Shortcuts

- `Ctrl + P`: Open preferences
- `Ctrl + Q`: Quit application

## Configuration

The application creates a `config.toml` file in the same directory as the executable. You can customize:

- Theme
- Font family and size
- Line height
- Padding
- Border radius

Example configuration:
```toml
[theme]
name = "tokyo-night"

[font]
family = "IBM Plex Sans Arabic"
size = 20
line_height = 2.0

[editor]
padding = 30
border_radius = 16
```

## Available Themes

- Tokyo Night (default)
- Dracula
- Nord
- Monokai
- Solarized Dark

## Development

### CI/CD Workflows

The project uses GitHub Actions for:
- Continuous Integration (CI)
  - Building and testing on each push
  - Code formatting checks
  - Clippy linting
  - Security audits
- Release automation
  - Building Debian packages
  - Creating GitHub releases
- Automated AUR updates
  - Publishing and updating the AUR package

### Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

