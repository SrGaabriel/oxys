#include <hip/hip_runtime.h>

__global__ void vector_add(const float* a, const float* b, float* c, int n) {
    int id = blockIdx.x * blockDim.x + threadIdx.x;
    if (id < n) {
        c[id] = a[id] + b[id];
    }
}