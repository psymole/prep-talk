# PrepTalk

![App Logo a gear in a speechbubble](https://github.com/psymole/prep-talk/blob/main/imgs/logo_prep-talk_circle.png)

A simple Linux laptop onboarding application built with [Rust](https://www.rust-lang.org/) and [iced](https://github.com/iced-rs/iced).  
It guides users through initial configuration steps with progress tracking, messages, and an optional logo, making the setup process more friendly and visual.

---

## ✨ Features

- **Custom Welcome Screen** – Set your own heading, body text, and logo.
- **Live Progress Tracking** – Monitors a `.log` file for task updates.
- **Progress Bar** – Shows percentage completion based on task number/total.
- **Configurable via TOML** – Uses `~/.prepTalk.toml` for customization.

---

## 📸 Example Screenshot
*(Optional — add a screenshot here once you have one)*
![main screen, not logs](https://github.com/psymole/prep-talk/blob/main/imgs/Screenshot%20from%202025-08-11%2014-04-11.png)
![main screen, logs and progress](https://github.com/psymole/prep-talk/blob/main/imgs/Screenshot%20from%202025-08-11%2014-06-36.png)
![main screen, shows button](https://github.com/psymole/prep-talk/blob/main/imgs/Screenshot%20from%202025-08-11%2014-13-12.png)
---

## 🛠️ Installation & Usage

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/preptalk.git
cd preptalk
```

### 1. Build & Run
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
We can also read one-line pseudo TOML
```toml
task_msg = "Configuring default Browser" task_number = 65
```


