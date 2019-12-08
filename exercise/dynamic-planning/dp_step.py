#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
一个机器人位于一个 m x n 网格的左上角，机器人每次只能向下或者向右移动一步，机器人试图达到网格的右下角。
问总共有多少条不同的路径？

 - dp[i][j]: 走到(i,j)共有dp[i][j]条路径
 - dp[i][j] = dp[i-1][j] + dp[i][j-1]
 - dp[:][0]=1, dp[0][:]=1
"""

def step(m, n):
    if m <= 0 or n <= 0:
        return 0
    dp = [[1 for x in range(n)] for y in range(m)]
    # dp[0][:] = 1
    # dp[:][0] = 1
    for i in range(1, m):
        for j in range(1, n):
            dp[i][j] = dp[i-1][j] + dp[i][j-1]
    return dp[m-1][n-1]

print(step(20, 30))
