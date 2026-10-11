use oxys_hip_sys::{devices, load_module};

use crate::kernels::VADD_KERNEL;

mod kernels;

fn main() {
    let devices = devices();
    println!("{}", devices);
    let module = load_module(VADD_KERNEL);
    
}