
````markdown
# 🧵 Minimal HTTP Server with Thread Pool in Rust

This project implements a simple multithreaded HTTP server in Rust using a custom-built thread pool. It serves static HTML files and demonstrates concurrency with graceful shutdown.

## 🚀 Features

- Serves `hello.html` on:
  - `/`
  - `/sleep` (with a simulated 5-second delay)
- Serves a `404.html` page for all other routes
- Handles requests concurrently using a fixed-size thread pool
- Shuts down cleanly after handling 2 incoming requests (for demo purposes)

## 📁 Project Structure

```plaintext
.
├── Cargo.toml
├── src
│   ├── main.rs       # Starts the server and handles incoming HTTP requests
│   └── lib.rs        # Custom thread pool implementation
├── hello.html        # Served on `/` and `/sleep`
└── 404.html          # Served on all other paths
````

## ▶️ Usage

### 1. Run the server:

```bash
cargo run
```

### 2. Open in your browser:

* `http://127.0.0.1:7878/` → ✅ returns `hello.html`
* `http://127.0.0.1:7878/sleep` → ⏳ waits 5s, then returns `hello.html`
* `http://127.0.0.1:7878/unknown` → ❌ returns `404.html`

> **Note:** The server only accepts 2 requests before shutting down (for demonstration purposes).

## 📚 Documentation

### 🧠 Manual Overview

#### `main.rs`
Handles:
- Starting the TCP listener
- Accepting incoming connections
- Dispatching request handling to the thread pool
- Serving either `hello.html` or `404.html` based on the request path

#### `lib.rs`
Defines:
- `ThreadPool`: a basic thread pool that distributes tasks
- `Worker`: listens for and executes jobs sent via an MPSC channel

---

### 🛠️ Auto-generated Rust Documentation

You can generate and view the full API documentation using [`cargo doc`](https://doc.rust-lang.org/cargo/commands/cargo-doc.html):

#### 📄 Generate Docs Locally

```bash
cargo doc --open
```

## 🧪 Tests

### ✅ Run tests:

```bash
cargo test
```

### 📌 Example Tests

* `test_thread_pool_executes_task_pass` → should **pass**
* `test_thread_pool_executes_task_fail` → intentionally **fails** for demo

## 🌐 HTML Pages

### `hello.html`

A modern welcome page with:

* Responsive layout
* Centered message
* Beautiful gradient background

### `404.html`

A clean error page that says:

* `404 Not Found`
* “Go back home” button

## 📸 Screenshots

<details>
<summary>hello.html</summary>

![hello page](screenshots/hello.png)

</details>

<details>
<summary>404.html</summary>

![404 page](screenshots/404.png)

</details>

## ✍️ Author

Waleed Ebrahem Mohamed
[LinkedIn](https://www.linkedin.com/in/waleed-ebrahem-46624a1b2/)
[GitHub](https://github.com/waleed-222)


