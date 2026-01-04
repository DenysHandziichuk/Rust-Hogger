# Rust Internet Traffic Hogger (Educational)

A Rust-based **network traffic generator** built to explore **async networking**, **concurrency**, and **performance** using modern Rust tooling.

This project was created **strictly for educational and experimental purposes** to better understand how high-throughput HTTP downloads behave under parallel load.

---

## ⚠️ Educational Use Only

> **This project is intended for learning purposes only.**  
> Do **NOT** use this tool to disrupt networks, violate terms of service, or perform unauthorized stress testing.

The author takes **no responsibility** for misuse.

---

## 🧠 What This Project Demonstrates

- Asynchronous programming with **Tokio**
- Concurrent stream handling with `futures_util`
- Efficient HTTP downloading using `reqwest`
- Atomic counters for real-time bandwidth tracking
- Safe shared-state management with `Arc` + `AtomicU64`

---

## 🚀 How It Works

- Continuously downloads large public test files
- Runs multiple concurrent downloads in parallel
- Tracks total data consumed in real time
- Prints bandwidth usage statistics at fixed intervals

---

## 🛠️ Tech Stack

- Rust
- Tokio (async runtime)
- Reqwest (HTTP client)
- futures-util (stream concurrency)

---

## 📊 Example Output

```text
🚀 Internet Hogger Active...
📊 Total Consumed: 1.42 GB (1456 MB)
