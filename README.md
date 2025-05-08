# 🤖 Telegram Command Bot

## [Português] 🇧🇷 [README.pt.md](README.pt.md).

A Telegram bot developed in Rust using the [`teloxide`](https://github.com/teloxide/teloxide) library. It allows you to execute operating system commands remotely through a bot interface, with support for Windows and Linux.

## 📦 Dependencies

- [Rust](https://www.rust-lang.org/)
- [teloxide](https://crates.io/crates/teloxide)
- [tokio](https://crates.io/crates/tokio)
- [dotenv](https://crates.io/crates/dotenv)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)

## 🛠 Project Structure

- `main.rs`: Main bot code.
- `commands.json`: Defines available commands for each operating system.
- `.env`: Contains sensitive environment variables (do not version control!).

## 🤖 Creating the Bot on Telegram

1. On Telegram, search for [`@BotFather`](https://t.me/BotFather).
2. Send the `/newbot` command and follow the instructions to set the bot's name and username.
3. After creation, BotFather will provide an API token. This will be your `TELEGRAM_BOT_TOKEN`.

## 🔐 How to get your `ALLOWED_USER_ID`

1. Access the [`@userinfobot`](https://t.me/userinfobot) bot on Telegram.
2. It will return your `user_id`. Copy this number and add it to the `.env` file.

## 📁 Example `commands.json`

```json
{
    "windows": {
        "📁 List Files": "dir",
        "🌐 Network Info": "ipconfig /all",
        "🖥️ Processes": "tasklist",
        "⚙️ System Info": "systeminfo",
        "🔌 Shutdown": "shutdown /s /t 60"        
    },
    "linux": {
        "📁 List Files": "ls -la",
        "💾 Disk Space": "df -h",
        "📄 Memory": "free -h",
        "🌐 Network Info": "ifconfig || ip a",
        "🔌 Shutdown": "sudo /sbin/shutdown -h now",
        "⚙️ System Info": "uname -a"
    }   
}
```

## 🔐 `.env` (DO NOT COMMIT!)

Create an `.env` file with the following variables:

```env
TELEGRAM_BOT_TOKEN=your_token_here
ALLOWED_USER_ID=123456789
```

## 🚀 Running the project

```bash
cargo run
```

> Make sure the `commands.json` file is present in the root directory.

## 🧾 Features

- Interactive keyboard for command selection.
- Authorized user verification via `ALLOWED_USER_ID`.
- Safe command execution with confirmation keyboard.
- Cross-platform support (Windows/Linux).

## 🛡 Security

- Only an authorized user can interact with the bot.
- Confirmation before critical commands like shutdown.
- `.env` file is ignored by `.gitignore`.

## 📄 License

MIT
