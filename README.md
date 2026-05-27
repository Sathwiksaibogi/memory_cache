# Concurrent In-Memory Cache with TTL Eviction

A lightweight, thread-safe, in-memory key-value storage engine built from scratch in Rust. This project implements a synchronous cache capable of safely sharing state across multiple threads, leveraging a dual-eviction strategy (passive and active) to manage memory efficiently without data racing.

## 🚀 Features

* **Thread-Safe Architecture:** Wrapped in `Arc<Mutex<T>>` to support safe concurrent read and write operations across independent execution threads.
* **Dual Eviction Strategy:**
    * **Passive Eviction:** Expired items are intercepted and filtered out on the fly during `get` requests.
    * **Active Eviction:** A dedicated background cleaner thread runs on a loop to actively purge expired keys from the underlying `HashMap`, preventing memory leaks.
* **Monotonic Time Tracking:** Uses `std::time::Instant` to calculate Time-To-Live (TTL) boundaries, making the eviction engine immune to manual system clock or timezone adjustments.
* **Generic Data Storage:** Built using Rust type generics (`<T>`) to support caching any uniform data type.

---

---

## 🛠️ Project Structure

* `src/cache.rs` - Contains the internal logic, including the `CacheItem<T>` blueprint, structural implementation blocks (`impl`), lookups, and the two-phase "hit list" collection mechanism used during cleanup to safely alter the map without iterator invalidation.
* `src/main.rs` - Houses the runtime environment, the background worker thread spawn sequence, type initialization utilizing the Turbofish syntax, and the multi-threaded simulation assertions.

---

## 🏃‍♂️ Getting Started

### Prerequisites
Ensure you have the Rust toolchain installed on your machine. If not, get it via [rustup.rs](https://rustup.rs/).

### Installation & Execution
1. Clone the repository:
   ```bash
   git clone [https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git](https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git)
   cd YOUR_REPO_NAME
   
2. Run the concurrent test suite:

```Bash
cargo run
