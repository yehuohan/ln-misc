
import numpy as np

n = 2;
a = np.array([1, 2])
b = np.array([[1, 2],
              [1, 2]])

"""
c_i = \sum_j b_{ij}
"""
c1 = np.zeros(n)
for i in np.arange(n):
    for j in np.arange(n):
        c1[i] += b[i, j]
c2 = np.zeros(n)
for i in np.arange(n):
    c2[i] = np.sum(b[i])
c3 = np.sum(b, axis=1)      # 即以axis为自变量遍历进行sum
# print(c1)
# print(c2)
# print(c3)


"""
c_{ij} = a_i b_{ij}
\sum_i^n \sum_j^n c_{ij}

np.divide 与 np.multiply 类似
"""
c1 = np.zeros((n, n))
for i in np.arange(n):
    for j in np.arange(n):
        c1[i, j] = a[i] * b[i, j]
c2 = np.multiply(a.reshape(n, 1), b)    # i为行，a以i遍历，则a与b的行相同
# print(c1, np.sum(c1))
# print(c2, np.sum(c2))


"""
c_{ij} = a_j b_{ij}
\sum_i^n \sum_j^n c_{ij}
"""
c1 = np.zeros((n, n))
for i in np.arange(n):
    for j in np.arange(n):
        c1[i, j] = a[j] * b[i, j]
c2 = np.multiply(a.reshape(1, n), b)    # j为列，a以j遍历，则a与b的列相同
# print(c1, np.sum(c1))
# print(c2, np.sum(c2))


"""
c_i  = \sum_i \sum_j a_j b_{ij}
"""
c1 = np.zeros((n))
for i in np.arange(n):
    for j in np.arange(n):
        c1[i] += a[j] * b[i][j]
c2 = np.dot(a, b.T)                     # dot代替'求积累加'
# print(c1)
# print(c2)


"""
c = \sum_i a_i a_{n-i}
"""
c1 = 0
for i in np.arange(n):
        c1 += a[i] * a[n-i-1]
c2 = np.dot(a, a[::-1])
# print(c1)
# print(c2)


"""
c_{ij} = \sum_i \sum_j \sum_k b_{ik} b_{kj}
"""
c1 = np.zeros((n, n))
for i in np.arange(n):
    for j in np.arange(n):
        for k in np.arange(n):
            c1[i, j] += b[i, k] * b[k, j]
c2 = np.dot(b, b)
# print(c1)
# print(c2)


#%% tensordot, matmul
a = np.array([[1, 2],
              [2, 3],
              [3, 4]])
b = np.array([[2, 2],
              [3, 3],
              [4, 4]])
print(np.dot(a[0, :], b[0, :]))
print(np.dot(a[1, :], b[1, :]))
print(np.dot(a[2, :], b[2, :]))
print(np.tensordot(a, b, axes=[(1,), (1,)]))
# tensordot: 取a[,:]与b[,:]做dot
print(np.dot(a.reshape(1, 3, 2), b.reshape(1, 2, 3)))
# dot: a的第n-1维作为向量(1x3个2维向量)，与b的第n-2维作为向量(1x3个2维向量)做dot，得到[1x3 x 1x3]
print(np.matmul(a.reshape(3, 1, 2), b.reshape(3, 2, 1)).ravel())
# matmul: 取a和b的n-1和n-2维作为矩阵再做multiply（即3个1x2的矩阵与3个2x1的矩阵multiply），得到(3 x 1x1)
c1 = np.empty(3)
for k in np.arange(3):
    c1[k] = np.dot(a[k], b[k])
print(c1)
