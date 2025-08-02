# outlook-rust

**outlook-rust** is a lightweight desktop application built with [Tauri](https://tauri.app) and [Rust](https://www.rust-lang.org), offering a secure and native-like experience of Outlook 365 by embedding its web version into a system WebView.

## Features

- 📨 Seamless access to [Outlook 365](https://outlook.office.com) in a native desktop window
- 🖥️ Built using Rust and WebKit (via Tauri) for performance and security
- 🔔 Native system notifications
- 🛠️ System tray integration with customizable menu (Settings, About, etc.)
- 🎨 Custom icons and app identity
- 🔐 (Optional) SSO / Microsoft authentication support

## Screenshots

![Screenshot](./assets/screenshot.png)

## Installation

Download the latest release from the [Releases](./releases) section or build it manually:

### Build Requirements

- Rust (stable)
- Node.js + npm
- Tauri CLI

### Build Instructions

```bash
# Install Tauri CLI if not installed
cargo install tauri-cli

# Clone the repo
git clone https://github.com/yourusername/outlook-rust.git
cd outlook-rust

# Install dependencies and run
npm install
npm run tauri dev
