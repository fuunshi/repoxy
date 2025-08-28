# 🚀 Repoxy

**Repoxy** is a lightweight, experimental **reverse proxy / API gateway** written in [Rust](https://www.rust-lang.org/).  
It’s currently a **WIP / learning project**, built as part of my journey into Rust, systems programming, and network infrastructure.  

---

## ✨ Goals
- High-performance reverse proxy built on async Rust (`tokio`, `hyper`, `axum`)
- Configurable routing (via `repoxy.toml`)
- TLS termination (via `rustls`)
- API Gateway features:
  - ✅ Path-based routing  
  - 🔒 JWT / API key authentication  
  - ⚡ Rate limiting  
  - 📊 Observability (logging, metrics, tracing)  
- Load balancing (round-robin / least-connections)  
- Extensible middleware (filters, request rewriting, etc.)

---

## 📦 Installation
Clone the repo and build with Cargo:
```bash
git clone https://github.com/fuunshi/repoxy.git
cd repoxy
cargo build --release
````

Run:

```bash
cargo run
```

---

## 🛠️ Example Config (`repoxy.toml`)

```toml
[[routes]]
path = "/api/users"
backends = ["http://localhost:5000", "http://localhost:5001"]

[[routes]]
path = "/api/orders"
backends = ["http://localhost:6000"]
```

---

## 📍 Roadmap

* [ ] Minimal reverse proxy (forward requests → backend)
* [ ] Configurable routes via TOML
* [ ] JWT & API key validation
* [ ] Rate limiting per client
* [ ] TLS termination
* [ ] Load balancing
* [ ] Prometheus metrics & tracing
* [ ] Plugin system (maybe WASM-based)

---

## ⚠️ Disclaimer

This project is a **work in progress (WIP)** and part of my **learning journey in Rust**.
Expect breaking changes, missing features, and experiments. Not production-ready (yet).

---

## 📜 License

Licensed under the **Apache License, Version 2.0**.
See [LICENSE](LICENSE) for details.
