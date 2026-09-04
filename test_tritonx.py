import ctypes
import random
import time

# Load the compiled Rust DLL directly
lib = ctypes.CDLL("./target/release/tritonx.dll")

# Define function signatures
lib.matmul_cpu.argtypes = [
    ctypes.POINTER(ctypes.c_float),
    ctypes.POINTER(ctypes.c_float),
    ctypes.POINTER(ctypes.c_float),
    ctypes.c_size_t
]
lib.matmul_cpu.restype = None

SIZE = 512
a = (ctypes.c_float * (SIZE * SIZE))(*[random.random() for _ in range(SIZE * SIZE)])
b = (ctypes.c_float * (SIZE * SIZE))(*[random.random() for _ in range(SIZE * SIZE)])
c = (ctypes.c_float * (SIZE * SIZE))()

start = time.time()
lib.matmul_cpu(a, b, c, SIZE)
end = time.time()

print(f"🎉 TritonX Engine via Python Execution Time: {(end - start) * 1000:.2f} ms")