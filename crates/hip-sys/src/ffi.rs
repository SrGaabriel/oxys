use std::ffi::{c_char, c_int, c_uint, c_void};

pub type HipError = c_int;
pub type HipModule = *mut c_void;
pub type HipFunction = *mut c_void;
pub type HipStream = *mut c_void;

pub const _MEMCPY_RAM_TO_RAM: c_int = 0;
pub const MEMCPY_RAM_TO_GPU: c_int = 1;
pub const MEMCPY_GPU_TO_RAM: c_int = 2;
pub const _MEMCPY_GPU_TO_GPU: c_int = 3;
pub const _MEMCPY_AUTO: c_int = 4;

#[link(name = "amdhip64")]
unsafe extern "C" {
    #[must_use]
    pub(crate) fn hipGetDeviceCount(ptr: *mut c_int) -> HipError;
    #[must_use]
    pub(crate) fn hipModuleLoadData(module: *mut HipModule, image: *const c_void) -> HipError;
    #[must_use]
    pub(crate) fn hipModuleGetFunction(
        fun: *mut HipFunction,
        module: HipModule,
        c: *const c_char,
    ) -> HipError;
    #[must_use]
    pub(crate) fn hipModuleLaunchKernel(
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
    #[must_use]
    pub(crate) fn hipDeviceSynchronize() -> HipError;
    #[must_use]
    pub(crate) fn hipMalloc(ptr: *mut *mut c_void, size: usize) -> HipError;
    #[must_use]
    pub(crate) fn hipMemcpy(
        dest: *mut c_void,
        src: *const c_void,
        size: usize,
        kind: c_int,
    ) -> HipError;
    #[must_use]
    pub(crate) fn hipFree(ptr: *mut c_void) -> HipError;
    #[must_use]
    pub(crate) fn hipGetErrorString(err: HipError) -> *const c_char;
}
