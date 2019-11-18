#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import numpy as np

"""
eigenvectors from eigenvalues.

https://arxiv.org/abs/1908.03795
"""

n = 5
A = np.matrix(np.random.random(n*n).reshape((n, n)))
A = A + A.T     # hermitian matrix

# calculate
w_, v_ = np.linalg.eig(A)
v = np.zeros((n, n))
for i in np.arange(n):
    for j in np.arange(n):
        idx = np.concatenate((np.arange(j), np.arange(j+1, n)), axis=0)
        Mj = A[np.ix_(idx, idx)]
        w, _ = np.linalg.eig(Mj)
        v[i, j] = np.prod(w_[i] - w)
        idx = np.concatenate((np.arange(i), np.arange(i+1, n)), axis=0)
        v[i, j] /= np.prod(w_[i] - w_[np.ix_(idx)])

print(np.power(v_.T, 2))
print(v)
