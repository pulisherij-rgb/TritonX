# TritonX Acceleration Engine

A high-performance, C-ABI matrix multiplication engine written in Rust with Rayon multi-threading, bridged seamlessly to Python for low-latency, GPU-comparable CPU compute.

![TritonX Performance Demo](demogif1.gif)

---

## Key Features

* **~1200x Speedup**: Massive acceleration over pure Python matrix operations via low-overhead C-ABI bindings.
* **Rust + Rayon Backend**: Fully utilizes multi-core CPU architectures for high throughput (GFLOPS).
* **Decoupled Architecture**: Fast API service paired with a real-time Chart.js dashboard.
* **Hardware-Agnostic**: Delivers high-performance compute without requiring expensive GPU infrastructure.

---

## System Architecture
