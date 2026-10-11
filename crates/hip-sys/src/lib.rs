use std::{
    ffi::{c_int, c_void},
    ptr::null_mut,
};

pub type HipError = c_int;
pub type HipModule = *mut c_void;
pub type HipFunction = *mut c_void;
pub type HipStream = *mut c_void;

#[link(name = "amdhip64")]
unsafe extern "C" {
    fn hipGetDeviceCount(ptr: *mut c_int) -> HipError;
    fn hipModuleLoadData(module: *mut HipModule, image: *const c_void) -> HipError;
}

pub fn devices() -> i32 {
    let mut count = 0;
    let _result = unsafe {
        hipGetDeviceCount(&mut count)
    };
    count
}

pub fn load_module(kernel: &[u8]) -> HipModule {
    let mut module: HipModule = null_mut();
    let _result = unsafe { hipModuleLoadData(&mut module, kernel.as_ptr() as *const c_void) };
    module
}
