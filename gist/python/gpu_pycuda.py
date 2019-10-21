#!/usr/bin/env python3
# -*- coding: utf-8 -*-


#%% 编写GPU-c代码
import pycuda.autoinit
import pycuda.driver as cuda
from pycuda.compiler import SourceModule
import numpy as np
"""
pycuda可以使用pycuda.compiler.SourceModule编译c代码。
本质上是使用nvcc编译c代码，再由python调用。
"""
mod = SourceModule("""
    __global__ void doublify(int* a)
    {
        int idx = threadIdx.x + threadIdx.y * 4;
        a[idx] *= 2;
        /*
        a[0] = gridDim.x;
        a[1] = gridDim.y;
        a[2] = gridDim.z;
        a[3] = blockDim.x;
        a[4] = blockDim.y;
        a[5] = blockDim.z;
        */
    }
        """)
func = mod.get_function('doublify') # 获取GPU函数
print(type(mod))
print(type(func))


"""
使用GPU进行计算，需要把主机和GPU设备间进行数据拷贝。
"""
a = (10*np.random.randn(4, 4)).astype(np.int32)
a_gpu = cuda.mem_alloc(a.nbytes)    # 申请显存
cuda.memcpy_htod(a_gpu, a)          # 复制主机数据到GPU设备(host to device)
func(a_gpu, grid=(1, 1, 1), block=(4, 4, 1))
# func(cuda.InOut(a), block=(4, 4, 1)) # 使用cuda.InOut，代码更简洁
a_doubled = np.empty_like(a)
cuda.memcpy_dtoh(a_doubled, a_gpu)  # 从GPU设备复制数据到主机(device to host)
print(a)
# print(a_doubled)
print(a_doubled.ravel())


#%% 使用GPUArray
import pycuda.autoinit
import pycuda.gpuarray as gpuarray
import numpy as np
"""
pycuda.gpuarray.GPUArray与numpy.ndarray类似，不过是对GPU内存操作。
具有shape、dtype等属性；
具有ravel()、view()、copy()等操作；
可以进行sum()、dot()、等运算。
"""
a = (10*np.random.randn(4, 4)).astype(np.int32)
a_gpu = gpuarray.to_gpu(a)
a_gpu = gpuarray.ones_like(a_gpu)
a_gpu = gpuarray.arange(0, 16, dtype=np.int32).reshape(4, 4)
a_doubled = (2*a_gpu).get()
a_doubled_sum = gpuarray.sum(a_gpu)
print(type(a_gpu))
print(a_gpu)
print(a_doubled)
print(a_doubled_sum)
