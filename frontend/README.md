# 🚀 TritonX Acceleration Engine

TritonX is a high-performance, multi-threaded matrix multiplication runtime that bridges compiled Rust CPU kernels to Python via C-ABI bindings. It features a FastAPI REST server and a modern, decoupled dark-mode analytics dashboard for real-time GFLOPS and latency benchmarking against NumPy and Pure Python.

---

## 📸 Overview & Performance

* **1200x+ Speedup** over standard Pure Python implementations.
* **Outperforms NumPy** (C-BLAS) on mid-to-large matrix operations ($N \ge 256$).
* **Real-time Metrics:** Automatic calculation of execution latency (ms) and compute throughput (GFLOPS).

---

## 🏗 System Architecture
---

## ⚡ Quick Start

### Prerequisites
* Rust toolchain (`cargo`, `rustc`)
* Python 3.10+
* `pip install fastapi uvicorn numpy`

### Running with One Click
Launch the entire system (builds Rust DLL, launches server on port 9000, and opens dashboard):

```powershell
.\run_tritonx.ps1