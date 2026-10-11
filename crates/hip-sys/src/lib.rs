#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::{
    ffi::{CString, c_char, c_int, c_uint, c_void},
    marker::PhantomData,
    ptr::null_mut,
};

pub type HipError = c_int;
pub type HipModule = *mut c_void;
pub type HipFunction = *mut c_void;
pub type HipStream = *mut c_void;

pub const MEMCPY_RAM_TO_RAM: c_int = 0;
pub const MEMCPY_RAM_TO_GPU: c_int = 1;
pub const MEMCPY_GPU_TO_RAM: c_int = 2;
pub const MEMCPY_GPU_TO_GPU: c_int = 3;
pub const MEMCPY_AUTO: c_int = 4;

#[link(name = "amdhip64")]
unsafe extern "C" {
    fn hipGetDeviceCount(ptr: *mut c_int) -> HipError;
    fn hipModuleLoadData(module: *mut HipModule, image: *const c_void) -> HipError;
    fn hipModuleGetFunction(fun: *mut HipFunction, module: HipModule, c: *const c_char);
    fn hipModuleLaunchKernel(
        fun: HipFunction,
        grid_x: c_uint,
        grid_y: c_uint,
        grid_z: c_uint,
        block_x: c_uint,
        block_y: c_uint,
        block_z: c_uint,
        shared_mem_bytes: c_uint,
        stream: HipStream,
        kernel_params: *mut *mut c_void,
        extra: *mut *mut c_void,
    ) -> HipError;
    fn hipDeviceSynchronize() -> HipError;
    fn hipMalloc(ptr: *mut *mut c_void, size: usize) -> HipError;
    fn hipMemcpy(dest: *mut c_void, src: *const c_void, size: usize, kind: c_int) -> HipError;
    pub fn hipFree(ptr: *mut c_void) -> HipError;
}

pub fn devices() -> i32 {
    let mut count = 0;
    let _result = unsafe { hipGetDeviceCount(&mut count) };
    count
}

pub fn load_kernel(kernel: &[u8]) -> HipModule {
    let mut module: HipModule = null_mut();
    let _result = unsafe { hipModuleLoadData(&mut module, kernel.as_ptr() as *const c_void) };
    module
}

pub fn get_function(module: HipModule, name: &str) -> HipFunction {
    let mut function = null_mut();
    let name = CString::new(name).unwrap();
    let name = name.as_ptr();
    unsafe {
        hipModuleGetFunction(&mut function, module, name);
    }
    function
}

pub fn launch_kernel(
    fun: HipFunction,
    grid: (u32, u32, u32),
    block: (u32, u32, u32),
    shared_mem_bytes: u32,
    args: &mut [*mut c_void],
) {
    let _result = unsafe {
        hipModuleLaunchKernel(
            fun,
            grid.0,
            grid.1,
            grid.2,
            block.0,
            block.1,
            block.2,
            shared_mem_bytes,
            null_mut(),
            args.as_mut_ptr(),
            null_mut(),
        )
    };
}

pub fn synchronize() {
    unsafe {
        hipDeviceSynchronize();
    }
}

pub struct GpuBuffer<T: Copy> {
    pub ptr: *mut c_void,
    len: usize,
    _t: PhantomData<T>,
}

impl<T: Copy> GpuBuffer<T> {
    pub fn new(len: usize) -> Self {
        let mut ptr = null_mut();
        unsafe { hipMalloc(&mut ptr, len * size_of::<T>()) };
        Self {
            ptr,
            len,
            _t: PhantomData,
        }
    }

    pub fn from_ram(data: &[T]) -> Self {
        let buf = Self::new(data.len());
        unsafe {
            hipMemcpy(
                buf.ptr,
                data.as_ptr() as *const c_void,
                size_of_val(data),
                MEMCPY_RAM_TO_GPU,
            )
        };
        buf
    }

    pub fn to_ram(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len);
        unsafe {
            hipMemcpy(
                out.as_mut_ptr() as *mut c_void,
                self.ptr,
                self.len * size_of::<T>(),
                MEMCPY_GPU_TO_RAM,
            )
        };
        unsafe { out.set_len(self.len) };
        out
    }
}

impl<T: Copy> Drop for GpuBuffer<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { hipFree(self.ptr) };
        }
    }
}
