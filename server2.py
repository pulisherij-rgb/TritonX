from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import ctypes
import numpy as np
import time
import os

app = FastAPI(title="TritonX Engine API")

# Enable CORS for standalone frontend access
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Load Rust Shared Library
dll_path = os.path.abspath("./target/release/tritonx.dll")
lib = ctypes.CDLL(dll_path)

lib.matmul_cpu.argtypes = [
    ctypes.POINTER(ctypes.c_float),
    ctypes.POINTER(ctypes.c_float),
    ctypes.POINTER(ctypes.c_float),
    ctypes.c_size_t
]
lib.matmul_cpu.restype = None

class MatrixRequest(BaseModel):
    size: int = 256

def py_matmul(a, b, size):
    c = [0.0] * (size * size)
    for i in range(size):
        for k in range(size):
            a_val = a[i * size + k]
            for j in range(size):
                c[i * size + j] += a_val * b[k * size + j]
    return c

@app.post("/api/v1/benchmark")
def run_benchmark(req: MatrixRequest):
    size = req.size
    
    np_a = np.random.rand(size, size).astype(np.float32)
    np_b = np.random.rand(size, size).astype(np.float32)

    # 1. Rust Engine
    a_ctypes = np_a.ctypes.data_as(ctypes.POINTER(ctypes.c_float))
    b_ctypes = np_b.ctypes.data_as(ctypes.POINTER(ctypes.c_float))
    c_ctypes = (ctypes.c_float * (size * size))()

    start_rust = time.time()
    lib.matmul_cpu(a_ctypes, b_ctypes, c_ctypes, size)
    rust_time_sec = time.time() - start_rust
    rust_time_ms = rust_time_sec * 1000

    # Metrics
    total_flops = 2 * (size ** 3)
    rust_gflops = round((total_flops / rust_time_sec) / 1e9, 2)

    # 2. NumPy Engine
    start_np = time.time()
    _ = np.dot(np_a, np_b)
    numpy_time_ms = (time.time() - start_np) * 1000

    # 3. Pure Python (capped)
    py_time_ms = None
    if size <= 256:
        a_py = np_a.flatten().tolist()
        b_py = np_b.flatten().tolist()
        start_py = time.time()
        py_matmul(a_py, b_py, size)
        py_time_ms = (time.time() - start_py) * 1000

    return {
        "matrix_size": f"{size}x{size}",
        "rust_time_ms": round(rust_time_ms, 2),
        "rust_gflops": rust_gflops,
        "numpy_time_ms": round(numpy_time_ms, 2),
        "python_time_ms": round(py_time_ms, 2) if py_time_ms else None,
    }