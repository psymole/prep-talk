# PrepTalk

A simple Linux laptop onboarding application built with [Rust](https://www.rust-lang.org/) and [iced](https://github.com/iced-rs/iced).  
It guides users through initial configuration steps with progress tracking, messages, and an optional logo — making the setup process more friendly and visual.

---

## ✨ Features

- **Custom Welcome Screen** – Set your own heading, body text, and logo.
- **Live Progress Tracking** – Monitors a `.log` file for task updates.
- **Progress Bar** – Shows percentage completion based on task number / total.
- **Configurable via TOML** – Uses `~/.prepTalk.toml` for customization.
- **Simple Exit Flow** – Closes once setup is marked complete.
- **Cross-platform iced UI** – Runs on most Linux desktop environments.

---

## 📸 Example Screenshot
*(Optional — add a screenshot here once you have one)*

---

## 🛠️ Installation & Usage

### 1. Install Rust
If you don’t already have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repository
```bash
git clone https://github.com/yourusername/preptalk.git
cd preptalk
```

### 3. Build & Run
```bash
cargo run --release
```

---

## ⚙️ Configuration

PrepTalk loads its settings from:
```
~/.prepTalk.toml
```

Example configuration:
```toml
heading = "Welcome to your Linux Laptop"
body = "We want you to have a few applications and settings configured..."
logo = "/path/to/logo.png"
file = "/path/to/prep.log"
task_total = 10
```

---

## 📄 Log File Format

PrepTalk reads the **last line** of the log file and expects it in TOML key-value style.  
Example:
```toml
task_msg = "Installing package xyz"
task_number = 4
task_complete = "false"
```

---

## 🧑‍💻 Development Notes

- UI built with [`iced`](https://github.com/iced-rs/iced)
- Config parsing with [`serde`](https://serde.rs/) and [`toml`](https://docs.rs/toml)
- Home directory detection with [`dirs`](https://docs.rs/dirs)
- Periodic updates handled via `iced::time::every`

---

## 🤝 Contributing

Contributions are welcome!  
If you want to add features, fix bugs, or improve docs:

1. Fork this repo
2. Create a new branch (`git checkout -b feature-foo`)
3. Commit changes (`git commit -m "Add foo feature"`)
4. Push your branch (`git push origin feature-foo`)
5. Open a Pull Request

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
