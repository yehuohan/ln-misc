#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
python运算加速，使用numpy, numba, pycuda等加速。

:REF:
https://www.ibm.com/developerworks/community/blogs/jfp/entry/How_To_Compute_Mandelbrodt_Set_Quickly?lang=en
"""


import matplotlib as mpl
import matplotlib.pyplot as plt

def on_key(event:mpl.backend_bases.KeyEvent):
    if event.key == 'escape':
        plt.close()

def mandelbrot_plot(func, xmin, xmax, ymin, ymax,
        width=3, height=3, maxiter=80, dpi=72, cmap='gnuplot2'):
    img_width = dpi * width
    img_height = dpi * height
    x, y, z = func(xmin, xmax, ymin, ymax, img_width, img_height, maxiter)
    plt.subplots(dpi=dpi)
    plt.connect('key_press_event', on_key)
    plt.imshow(z, cmap=cmap, origin='lower', norm=mpl.colors.PowerNorm(0.3))
    plt.show()


#%% python
from timeit import timeit
import numpy as np

def py_mandelbrot_set(xmin, xmax, ymin, ymax, width, height, maxiter):
    def mandelbrot(c, maxiter):
        z = c
        for n in range(maxiter):
            if abs(z) > 2:
                return n
            z = z * z + c
        return 0

    re = np.linspace(xmin, xmax, width)
    im = np.linspace(ymin, ymax, height)
    z = np.array([mandelbrot(complex(r, i), maxiter) for r in re for i in im])
    z = z.reshape((height, width))
    return (re, im, z.T)

def func():
    args = (-0.74877, -0.74872, 0.06505, 0.06510, 1000, 1000, 20)
    py_mandelbrot_set(*args)
# print("python(x1):", timeit(stmt='func()', setup='from __main__ import func', number=1))


#%% numpy
from timeit import timeit
import numpy as np

def np_mandelbrot_set(xmin, xmax, ymin, ymax, width, height, maxiter):
    def mandelbrot(c, maxiter):
        output = np.zeros(c.shape)
        z = np.zeros(c.shape, np.complex64)
        for it in range(maxiter):
            notdone = np.less(z.real*z.real + z.imag*z.imag, 4.0)
            output[notdone] = it
            z[notdone] = z[notdone]**2 + c[notdone]
        output[output == maxiter-1] = 0
        return output

    re = np.linspace(xmin, xmax, width)
    im = np.linspace(ymin, ymax, height)
    c = re + im[:, None]*1j
    z = mandelbrot(c, maxiter)
    return (re, im ,z)

def func():
    args = (-0.74877, -0.74872, 0.06505, 0.06510, 1000, 1000, 20)
    np_mandelbrot_set(*args)
# print("numpy(x10):", timeit(stmt='func()', setup='from __main__ import func', number=10))


#%% numba
from timeit import timeit
import numba as nb
import numpy as np

@nb.jit
def nb_mandelbrot_set(xmin, xmax, ymin, ymax, width, height, maxiter):
    def mandelbrot(c, maxiter):
        z = c
        for n in range(maxiter):
            if abs(z) > 2:
                return n
            z = z * z + c
        return 0

    re = np.linspace(xmin, xmax, width)
    im = np.linspace(ymin, ymax, height)
    z = np.empty((height, width))
    for i in range(width):
        for j in range(height):
            z[i, j] = mandelbrot(re[i] + 1j*im[j], maxiter)
    return (re, im, z.T)

def func():
    args = (-0.74877, -0.74872, 0.06505, 0.06510, 1000, 1000, 20)
    nb_mandelbrot_set(*args)
# print("numba(x100):", timeit(stmt='func()', setup='from __main__ import func', number=100))


#%% numba guvertorsize with parallel
from timeit import timeit
from numba import jit, vectorize, guvectorize, float64, complex64, int32, float32
import numpy as np

@jit(int32(complex64, int32))
def nb_parallel_mandelbrot(c, maxiter):
    nreal = 0
    real = 0
    imag = 0
    z = c
    for n in range(maxiter):
        nreal = real * real - imag * imag + c.real
        imag = 2 * real * imag + c.imag
        real = nreal
        if real * real + imag * imag > 4.0:
            return n
    return 0

@guvectorize([(complex64[:], int32[:], int32[:])], '(n), ()->(n)', target='parallel')
def nb_parallel_mandelbrot_numpy(c, maxit, output):
    maxiter = maxit[0]
    for i in range(c.shape[0]):
        output[i] = nb_parallel_mandelbrot(c[i], maxiter)

def nb_parallel_mandelbrot_set(xmin, xmax, ymin, ymax, width, height, maxiter):
    re = np.linspace(xmin, xmax, width, dtype=np.float32)
    im = np.linspace(ymin, ymax, height, dtype=np.float32)
    c = re + im[:, None]*1j
    z = nb_parallel_mandelbrot_numpy(c, maxiter)
    return (re, im, z.T)

def func():
    args = (-0.74877, -0.74872, 0.06505, 0.06510, 1000, 1000, 20)
    nb_parallel_mandelbrot_set(*args)
# print("numba_parallel(x100):", timeit(stmt='func()', setup='from __main__ import func', number=100))


#%% numba guvertorsize with cuda
from timeit import timeit
from numba import jit, vectorize, guvectorize, float64, complex64, int32, float32
import numpy as np
import os

os.environ['CUDA_HOME'] = r'C:/MyApps/cuda/v9.2/Development'

@guvectorize([(complex64[:], int32[:], int32[:])], '(n), ()->(n)', target='cuda')
def nb_cuda_mandelbrot_numpy(c, maxit, output):
    maxiter = maxit[0]
    for i in range(c.shape[0]):
        creal = c[i].real
        cimag = c[i].imag
        real = creal
        imag = cimag
        output[i] = 0
        for n in range(maxiter):
            nreal = real * real
            nimag = imag * imag
            if nreal + nimag > 4.0:
                output[i] = n
                break
            imag = 2 * real * imag + cimag
            real = nreal - nimag + creal

def nb_cuda_mandelbrot_set(xmin, xmax, ymin, ymax, width, height, maxiter):
    re = np.linspace(xmin, xmax, width, dtype=np.float32)
    im = np.linspace(ymin, ymax, height, dtype=np.float32)
    c = re + im[:, None]*1j
    z = np.empty(c.shape, np.int)
    maxit = np.ones(c.shape, np.int) * maxiter
    z = nb_cuda_mandelbrot_numpy(c, maxiter)
    return (re, im, z.T)

def func():
    args = (-0.74877, -0.74872, 0.06505, 0.06510, 1000, 1000, 20)
    nb_cuda_mandelbrot_set(*args)
# print("numba_cuda(x100):", timeit(stmt='func()', setup='from __main__ import func', number=100))


#%% pycuda
from timeit import timeit
import pycuda.driver as drv
import pycuda.tools
import pycuda.autoinit
from pycuda.compiler import SourceModule
import pycuda.gpuarray as gpuarray
from pycuda.elementwise import ElementwiseKernel
import numpy as np

complex_gpu = ElementwiseKernel(
    "pycuda::complex<float> *q, int *output, int maxiter",
    """
    {
        float nreal, real = 0;
        float imag = 0;
        output[i] = 0;
        for(int curiter = 0; curiter < maxiter; curiter++) {
            float real2 = real*real;
            float imag2 = imag*imag;
            nreal = real2 - imag2 + q[i].real();
            imag = 2* real*imag + q[i].imag();
            real = nreal;
            if (real2 + imag2 > 4.0f){
                output[i] = curiter;
                break;
                };
        };
    }
    """,
    "complex5",
    preamble="#include <pycuda-complex.hpp>",)

def cuda_mandelbrot_set(xmin, xmax, ymin, ymax, width, height, maxiter):
    def mandelbrot(c, maxiter):
        q_gpu = gpuarray.to_gpu(c.astype(np.complex64))
        iterations_gpu = gpuarray.to_gpu(np.empty(c.shape, dtype=np.int))
        complex_gpu(q_gpu, iterations_gpu, maxiter)
        return iterations_gpu.get()

    re = np.linspace(xmin, xmax, width)
    im = np.linspace(ymin, ymax, height)
    c = re + im[:, None]*1j
    c = np.ravel(c)
    z = mandelbrot(c, maxiter)
    z = z.reshape(height, width)
    return (re, im, z)

def func():
    args = (-0.74877, -0.74872, 0.06505, 0.06510, 1000, 1000, 20)
    cuda_mandelbrot_set(*args)
# print("pycude(x100):", timeit(stmt='func()', setup='from __main__ import func', number=100))


#%% plot
def plot():
    args = (-2.0, 0.5, -1.25, 1.25, 3, 3, 80)
    mandelbrot_plot(py_mandelbrot_set, *args)
    args = (-0.74877, -0.74872, 0.06505, 0.06510, 3, 3, 4096, 120)
    mandelbrot_plot(np_mandelbrot_set, *args)
    mandelbrot_plot(nb_mandelbrot_set, *args)
    mandelbrot_plot(nb_parallel_mandelbrot_set, *args)
    mandelbrot_plot(nb_cuda_mandelbrot_set, *args)
    mandelbrot_plot(cuda_mandelbrot_set, *args)

plot()
