#!/usr/bin/env python

"""
给定3个正整数m,n,s：
从1到m个数里面，取出n个不同的数，使它们的和为s，有多少种取法？
"""

def ways(m, n, s):
    """动态规化"""
    # 边界条件
    if n == 0 and s == 0: # n个数 and 和为s
        return 1
    elif m == 0: # n和s必定有一个不为0
        return 0
    if m < n:
        return 0

    if m > s:
        # m超过s的部分不能取
        return ways(s, n, s)
    else:
        # 递规式： m的取法 = 取m + 不取m
        return ways(m - 1, n - 1, s - m) + ways(m - 1, n, s)

data = [
    (13, 4, 20),
    (12, 5, 18),
    (1, 1, 1),
    (1, 2, 1),
    (119, 3, 20)
]

for k in data:
    print(ways(*k))

