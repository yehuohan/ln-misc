#!/usr/bin/env python3
# -*- coding: utf-8 -*-

#%% numba with jit
import time
import numpy as np
from numba import jit

"""
常用decorator: jit, njit, vectorize, guvectorize ...
"""
@jit
def fast(a):
    trace = 0
    for n in range(a.shape[0]):
        trace += np.tanh(a[n, n])
    return a + trace

a = np.arange(100, dtype=np.int32).reshape(10, 10)

s = time.time()
fast(a)
e = time.time()
print("Time: ", (e - s))

s = time.time()
fast(a)
e = time.time()
print("Time: ", (e - s))


#%% numba with GPU(device management)
from numba import cuda
cuda.select_device(0)
print(cuda.gpus)


#%% numba with GPU(kernel funcion)
import time
import numpy as np
from numba import cuda
import math
import os
os.environ['CUDA_HOME'] = r'C:/MyApps/cuda/v9.2/Development'

"""
host: CPU
device: GPU
kernel function: a GPU function launched by the host and executed on the device
device function: a GPU function executed on the device which can only be called from the device

@cuda.jit: kernel function
@cuda.jit(device=True): device function

需要CUDA编程：https://docs.nvidia.com/cuda/cuda-c-programming-guide/

┌────────┐      ┌─────────────────┐
│Host    │      │Device           │
│        │      │ ┌──────────────┐│
│kernel0 -------->│Grid0         ││
│        │      │ │  Block0      ││
│kernel1 │      │ │ ┌───────────┐││
│        │      │ │ │Block1     │││
│        │      │ │ │  Thread0  │││
│        │      │ │ │  Thread1  │││
│        │      │ │ │  ...      │││
│        │      │ │ └───────────┘││
│        │      │ └──────────────┘│
└────────┘      └─────────────────┘
Grid, Block, Thread可以是一维、二维或三维的，分别用x,y,z索引。
根据GPU计算能力的不同，每个Grid的最大Block数，以及每个Block的最大Thread数都是不同的。

cuda.gridDim, cuda.blockDim用于索引每个维度的长度。
cuda.blockIdx, cuda.threadIdx用于索引每个维度的下标。
cuda.grid(n=1,2,3)用于索引thread在各维度的下标
"""

@cuda.jit
def index(a):
    a.flat[0] = cuda.gridDim.x
    a.flat[1] = cuda.gridDim.y
    a.flat[2] = cuda.gridDim.z
    a.flat[3] = cuda.blockDim.x
    a.flat[4] = cuda.blockDim.y
    a.flat[5] = cuda.blockDim.z
    # 使用Dim,Idx索引
    pos = cuda.blockIdx.x * (cuda.blockDim.x * cuda.blockDim.y)+ \
          cuda.threadIdx.y * cuda.blockDim.x + \
          cuda.threadIdx.x
    if 6 <= pos and pos < a.size:
        a[pos] = pos    # 默认是指针传值

a = np.zeros(30, dtype=np.int32)
grid_shape = (2, 1, 1)
block_shape = (3, 5, 1)
index[grid_shape, block_shape](a)     # 调用kernel函数时，需要指定grid和block的维度大小
print(a)

@cuda.jit
def multi_index(b):
    x, y, z = cuda.grid(3)
    b[x % cuda.gridDim.x,  y % cuda.gridDim.y,  z % cuda.gridDim.z,
      x % cuda.blockDim.x, y % cuda.blockDim.y, z % cuda.blockDim.z
        ] = math.sin(float(x)) + math.cos(float(y)) + math.log(float(z) + 1.0)
    dx, dy, dz = cuda.gridsize(3)
    # grid索引与Dim,Idx索引的联系
    b.flat[0] = dx
    b.flat[1] = dy
    b.flat[2] = dz
    b.flat[3] = cuda.gridDim.x * cuda.blockDim.x
    b.flat[4] = cuda.gridDim.y * cuda.blockDim.y
    b.flat[5] = cuda.gridDim.z * cuda.blockDim.z

def multi_index_cpu(b):
    for x in np.arange(b.shape[0] * b.shape[3]):
        for y in np.arange(b.shape[1] * b.shape[4]):
            for z in np.arange(b.shape[2] * b.shape[5]):
                b[x % b.shape[0],  y % b.shape[1],  z % b.shape[2],
                  x % b.shape[3],  y % b.shape[4],  z % b.shape[5]
                    ] = np.sin(x) + np.cos(y) + np.log(z + 1.0)
    b.flat[0] = b.shape[0] * b.shape[3]
    b.flat[1] = b.shape[1] * b.shape[4]
    b.flat[2] = b.shape[2] * b.shape[5]
    b.flat[3] = b.flat[0]
    b.flat[4] = b.flat[1]
    b.flat[5] = b.flat[2]

# max thread per block: 4*4*64=512
# shape = (8, 16, 128, 4, 4, 64)
shape = (2, 8, 32, 4, 4, 64)
b = np.zeros(shape, dtype=np.float64)
s = time.time()
multi_index_cpu(b)
e = time.time()
print("CPU Time: ", (e - s))
print(b.ravel())
b = np.zeros(shape, dtype=np.float64)
s = time.time()
multi_index[shape[0:3], shape[3:6]](b)
e = time.time()
print("GPU Time: ", (e - s))
print(b.ravel())
# print(b)


#%% numba with GPU(memory management)
from numba import cuda
import numpy as np
import os
os.environ['CUDA_HOME'] = r'C:/apps/cuda/v9.2/Development'

@cuda.jit
def inc(a):
    pos = cuda.grid(1)
    a[pos] += 1

a = np.zeros(16)
a_gpu = cuda.to_device(a)
print(type(a_gpu))
inc[1, 16](a_gpu)
print(a)
print(a_gpu.copy_to_host())

b_gpu = cuda.device_array(16)
b = b_gpu.copy_to_host()
print(type(b_gpu))
inc[1, 16](b_gpu)
print(b)
print(b_gpu.copy_to_host())
