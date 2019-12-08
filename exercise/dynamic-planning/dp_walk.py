#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
给定一个包含非负整数的 m x n 网格，每次只能向下或者向右移动一步。
请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

 - dp[i][j]: 走到(i,j)时的最小路径和为dp[i][j]
 - dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j]
 - dp[i][0] = grid[i][0] + dp[i-1][0]
   dp[0][j] = grid[0][j] + dp[0][j-1]
"""

import numpy as np

def walk(grid:np.ndarray):
    m, n = grid.shape
    if m <= 0 or n <= 0:
        return 0
    dp = np.zeros_like(grid)
    # 初始值
    dp[0][0] = grid[0][0]
    for k in range(1, m):
        dp[k][0] = dp[k-1][0] + grid[k][0] 
    for k in range(1, n):
        dp[0][k] = dp[0][k-1] + grid[0][k] 
    # 迭代计算
    for i in range(1, m):
        for j in range(1, n):
            dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j]
    return dp[m-1][n-1]

grid = np.random.randint(0, 10, (3, 4), dtype=np.int8)
print(grid)
print(walk(grid))
