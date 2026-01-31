# 🔥 Rust Hogger

Rust Hogger is a Rust-based network traffic generator built to explore async networking, concurrency, and performance using modern Rust tooling. This project was created strictly for educational and experimental purposes to better understand how high-throughput HTTP downloads behave under parallel load. It demonstrates various concepts, including asynchronous programming, concurrent stream handling, and efficient HTTP downloading.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

---
## ✨ Key Features
🔹 Asynchronous programming with Tokio
🔹 Concurrent stream handling with `futures_util`
🔹 Efficient HTTP downloading using `reqwest`
🔹 Atomic counters for real-time bandwidth tracking
🔹 Safe shared-state management with `Arc` + `AtomicU64`

---
## 🧭 Simple Workflow
1. Open the Rust Hogger app
2. Select the number of concurrent downloads
3. Choose the test file to download
4. Adjust the download interval
5. Start the traffic generation
6. Monitor the bandwidth usage statistics
7. Stop the traffic generation when finished

---
## 🎯 Purpose
Rust Hogger is intended for educational and experimental purposes, allowing users to explore async networking, concurrency, and performance in a controlled environment.

---
## 🧩 Installation & Usage
```bash
git clone https://github.com/DenysHandziichuk/Rust-Hogger
cd Rust-Hogger
```
To run the Rust Hogger, use the following command:
```bash
cargo run
```
Note: This will start the traffic generation on localhost.

---
## 🛠️ Tech Stack
Rust Hogger is built using Rust, with a tech stack that includes Tokio for async runtime and Reqwest for HTTP requests.