
# 🚀 Solana RPC Performance Checker

A blazing-fast CLI tool to test and benchmark Solana RPC endpoints.  
Measure latency, reliability, and responsiveness for key Solana JSON-RPC methods in seconds.

<p align="center">
  <img src="https://solana.com/src/img/branding/solanaLogoMark.svg" alt="Solana Logo" width="90" />
</p>

---

## ✨ Features

- 🧪 Benchmarks key RPC methods:
  - `getLatestBlockhash`
  - `getSlot`
  - `getBalance`
  - `getAccountInfo`
  - `getBlock`
  - `getTokenAccountsByOwner`
  - `getHealth`
- ⚙️ Sequential or parallel testing
- 🔁 Customizable test iterations
- 📊 Detailed metrics: min/avg/max latency
- 🏅 Performance ratings: Excellent → Very Slow
- ✅ Success rate calculations
- ❌ Full error visibility
- 🎨 Clean, colorful CLI interface

---

## 🛠️ Requirements

- [Rust](https://www.rust-lang.org/tools/install) (v1.70.0+)
- Internet connection to access Solana RPC endpoints

---

## 🔧 Installation

```bash
# Clone the repo
git clone https://github.com/nitriot/solana-rpc-checker.git
cd solana-rpc-checker

# Build the binary
cargo build --release
```

👉 Output binary will be available at `target/release/rpc-checker`.

---

## 🚀 Usage

```bash
# Default usage (Helius RPC)
cargo run

# Use a custom RPC
cargo run -- -u https://your-rpc-url.com

# Run 5 iterations
cargo run -- -i 5

# Run tests in parallel
cargo run -- -p

# Disable progress bar
cargo run -- --no-progress

# Combine all
cargo run -- -u https://rpc.example.com -i 5 -p --no-progress
```

---

## 📘 Command-line Flags

| Flag                 | Description                                      |
|----------------------|--------------------------------------------------|
| `-u`, `--url`        | Set custom RPC endpoint URL                      |
| `-i`, `--iterations` | Set number of iterations per method (default: 3) |
| `-p`, `--parallel`   | Run in parallel (default: sequential)            |
| `--no-progress`      | Disable the progress bar                         |

---

## 📊 Example Output

```text
╔═══════════════════════════════════════════════════════════════╗
║                  RPC PERFORMANCE REPORT                      ║
╚═══════════════════════════════════════════════════════════════╝

📅 Timestamp: 2025-05-08 06:04:05 UTC
✅ Success Rate: 100.0%
⚡ Speed Rating: Good (280ms avg)

🔹 getHealth — 93ms avg (Excellent)
🔹 getBalance — 196ms avg (Good)
🔹 getSlot — 282ms avg (Good)
🔹 getAccountInfo — 277ms avg (Good)
🔹 getLatestBlockhash — 447ms avg (Average)
🔹 getTokenAccountsByOwner — 690ms avg (Slow)
🔹 getBlock — 1820ms avg (Very Slow)
```

---

## 📈 Speed Rating Breakdown

| Rating      | Response Time     |
|-------------|-------------------|
| 🟢 Excellent | 0–100 ms          |
| 🟡 Good      | 101–300 ms        |
| 🟠 Average   | 301–600 ms        |
| 🔴 Slow      | 601–1000 ms       |
| 🚨 Very Slow | > 1000 ms         |

---

## 🤝 Contributing

All contributions are welcome 💜

```bash
# Fork and clone
git clone https://github.com/yourusername/solana-rpc-checker.git
cd solana-rpc-checker

# Make a new feature branch
git checkout -b feature/my-awesome-thing

# Code and commit
git commit -am "Add something cool"

# Push and PR
git push origin feature/my-awesome-thing
```

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## 🌐 Socials

<p align="center">
  <a href="https://twitter.com/nitriotsol" target="_blank">
    <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/twitter/twitter-original.svg" width="30" alt="Twitter" />
  </a>&nbsp;&nbsp;
  <a href="https://t.me/vitualsolana" target="_blank">
    <img src="https://upload.wikimedia.org/wikipedia/commons/8/82/Telegram_logo.svg" width="30" alt="Telegram" />
  </a>&nbsp;&nbsp;
  <a href="https://discordapp.com/users/nitriot" target="_blank">
    <img src="https://cdn.jsdelivr.net/gh/devicons/devicon/icons/discord/discord-original.svg" width="30" alt="Discord" />
  </a>
</p>

<p align="center">
  Made with ❤️ by <strong>Nitriot</strong> 
</p>
