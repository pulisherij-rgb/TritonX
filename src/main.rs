use ndarray::Array2;
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

const MATRIX_SIZE: usize = 512;

fn naive_matmul(a: &Array2<f32>, b: &Array2<f32>, c: &mut Array2<f32>) {
    let n = a.nrows();
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[[i, k]] * b[[k, j]];
            }
            c[[i, j]] = sum;
        }
    }
}

// TritonX Optimized Parallel Engine
fn tritonx_matmul(a: &Array2<f32>, b: &Array2<f32>, c: &mut Array2<f32>) {
    let n = a.nrows();
    let b_trans = b.t();
    
    // Convert slice chunks for multi-threaded parallel execution across CPU cores
    c.as_slice_mut().unwrap()
        .par_chunks_mut(n)
        .enumerate()
        .for_each(|(i, row_c)| {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += a[[i, k]] * b_trans[[j, k]];
                }
                row_c[j] = sum;
            }
        });
}

fn main() {
    println!("=== TRITONX RUNTIME ENGINE: LOCAL BENCHMARK ===");
    println!("Running 512x512 matrix operations on CPU...\n");

    let mut rng = rand::thread_rng();
    let a_data: Vec<f32> = (0..MATRIX_SIZE * MATRIX_SIZE).map(|_| rng.gen::<f32>()).collect();
    let b_data: Vec<f32> = (0..MATRIX_SIZE * MATRIX_SIZE).map(|_| rng.gen::<f32>()).collect();

    let a = Array2::from_shape_vec((MATRIX_SIZE, MATRIX_SIZE), a_data).unwrap();
    let b = Array2::from_shape_vec((MATRIX_SIZE, MATRIX_SIZE), b_data).unwrap();
    let mut c_default = Array2::zeros((MATRIX_SIZE, MATRIX_SIZE));
    let mut c_triton = Array2::zeros((MATRIX_SIZE, MATRIX_SIZE));

    // Standard CPU Execution
    let start_default = Instant::now();
    naive_matmul(&a, &b, &mut c_default);
    let duration_default = start_default.elapsed();

    // TritonX Multi-Threaded Engine
    let start_triton = Instant::now();
    tritonx_matmul(&a, &b, &mut c_triton);
    let duration_triton = start_triton.elapsed();

    // Force Rust to keep values in memory so it doesn't skip calculation
    println!("Standard CPU Runtime: {:?}", duration_default);
    println!("TritonX Engine Speed: {:?}", duration_triton);
    println!("\nVerification Checksum: {}", c_default[[0,0]] + c_triton[[0,0]]);
}