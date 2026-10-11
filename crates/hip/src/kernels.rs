use std::ffi::c_void;

use oxys_hip_sys::{GpuBuffer, Result, get_function, launch_kernel, load_kernel, synchronize};

pub static VADD_CO: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vadd.co"));

pub fn vadd(a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
    let module = load_kernel(VADD_CO)?;
    let kernel = get_function(module, "vector_add")?;

    let d_a = GpuBuffer::from_ram(a)?;
    let d_b = GpuBuffer::from_ram(b)?;
    let d_c = GpuBuffer::<f32>::new(a.len())?;

    let mut p_a = d_a.ptr;
    let mut p_b = d_b.ptr;
    let mut p_c = d_c.ptr;
    let mut n: i32 = a.len() as i32;

    let mut args = [
        &mut p_a as *mut _ as *mut c_void,
        &mut p_b as *mut _ as *mut c_void,
        &mut p_c as *mut _ as *mut c_void,
        &mut n as *mut _ as *mut c_void,
    ];

    let block = 256u32;
    let grid = (n as u32).div_ceil(block);

    launch_kernel(kernel, (grid, 1, 1), (block, 1, 1), 0, &mut args)?;
    synchronize()?;

    d_c.to_ram()
}
