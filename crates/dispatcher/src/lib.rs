pub enum Kernel {
    VectorAdd
}

pub enum Dispatcher {
    Auto,
    AutoGpu,
    Wgpu,
    Cpu,
    Hip,
    Cuda
}

pub fn dispatch(dispatcher: Kernel, )