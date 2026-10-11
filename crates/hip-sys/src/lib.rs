#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::{
    ffi::{CStr, CString, c_void},
    fmt::{self, Display},
    marker::PhantomData,
    ptr::null_mut,
};

mod ffi;

#[derive(Debug)]
pub struct HipError {
    pub code: i32,
    pub message: String,
}

impl Display for HipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ROCm error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for HipError {}

pub type Result<T> = std::result::Result<T, HipError>;

#[must_use = "Use the result value"]
fn check(code: ffi::HipError) -> Result<()> {
    if code == 0 {
        return Ok(());
    }
    let message = unsafe {
        let s = ffi::hipGetErrorString(code);
        if s.is_null() {
            "unknown error".to_owned()
        } else {
            CStr::from_ptr(s).to_string_lossy().into_owned()
        }
    };
    Err(HipError { code, message })
}

pub fn devices() -> Result<i32> {
    let mut count = 0;
    check(unsafe { ffi::hipGetDeviceCount(&mut count) })?;
    Ok(count)
}

pub fn load_kernel(kernel: &[u8]) -> Result<ffi::HipModule> {
    let mut module: ffi::HipModule = null_mut();
    check(unsafe { ffi::hipModuleLoadData(&mut module, kernel.as_ptr() as *const c_void) })?;
    Ok(module)
}

pub fn get_function(module: ffi::HipModule, name: &str) -> Result<ffi::HipFunction> {
    let mut function = null_mut();
    let name = CString::new(name).unwrap();
    let name = name.as_ptr();
    check(unsafe { ffi::hipModuleGetFunction(&mut function, module, name) })?;
    Ok(function)
}

pub fn launch_kernel(
    fun: ffi::HipFunction,
    grid: (u32, u32, u32),
    block: (u32, u32, u32),
    shared_mem_bytes: u32,
    args: &mut [*mut c_void],
) -> Result<()> {
    check(unsafe {
        ffi::hipModuleLaunchKernel(
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
    })
}

pub fn synchronize() -> Result<()> {
    check(unsafe { ffi::hipDeviceSynchronize() })
}

pub struct GpuBuffer<T: Copy> {
    pub ptr: *mut c_void,
    len: usize,
    _t: PhantomData<T>,
}

impl<T: Copy> GpuBuffer<T> {
    pub fn new(len: usize) -> Result<Self> {
        let mut ptr = null_mut();
        check(unsafe { ffi::hipMalloc(&mut ptr, len * size_of::<T>()) })?;
        Ok(Self {
            ptr,
            len,
            _t: PhantomData,
        })
    }

    pub fn from_ram(data: &[T]) -> Result<Self> {
        let buf = Self::new(data.len())?;
        check(unsafe {
            ffi::hipMemcpy(
                buf.ptr,
                data.as_ptr() as *const c_void,
                size_of_val(data),
                ffi::MEMCPY_RAM_TO_GPU,
            )
        })?;
        Ok(buf)
    }

    pub fn to_ram(&self) -> Result<Vec<T>> {
        let mut out = Vec::with_capacity(self.len);
        check(unsafe {
            ffi::hipMemcpy(
                out.as_mut_ptr() as *mut c_void,
                self.ptr,
                self.len * size_of::<T>(),
                ffi::MEMCPY_GPU_TO_RAM,
            )
        })?;
        unsafe { out.set_len(self.len) };
        Ok(out)
    }
}

impl<T: Copy> Drop for GpuBuffer<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            check(unsafe { ffi::hipFree(self.ptr) }).unwrap();
        }
    }
}
