use rayon::prelude::*;

const BLOCK_SIZE: usize = 64;

#[no_mangle]
pub extern "C" fn matmul_cpu(a: *const f32, b: *const f32, c: *mut f32, size: usize) {
    let a_slice = unsafe { std::slice::from_raw_parts(a, size * size) };
    let b_slice = unsafe { std::slice::from_raw_parts(b, size * size) };
    let c_slice = unsafe { std::slice::from_raw_parts_mut(c, size * size) };

    // Parallelize over rows cleanly using Rayon chunks (no pointers needed)
    c_slice
        .par_chunks_mut(size)
        .enumerate()
        .for_each(|(i, c_row)| {
            // Zero out row
            for val in c_row.iter_mut() {
                *val = 0.0;
            }
            // Tiled computation per row
            for k in 0..size {
                let a_val = a_slice[i * size + k];
                let b_row = &b_slice[k * size..(k + 1) * size];
                for j in 0..size {
                    c_row[j] += a_val * b_row[j];
                }
            }
        });
}