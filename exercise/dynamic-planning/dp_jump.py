#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
一只青蛙一次可以跳上1级台阶，也可以跳上2级。
求该青蛙跳上一个n级的台阶总共有多少种跳法。

 - 数组定义
dp[k]: 跳上一个k级的台阶总共有dp[k]种跳法

 - 递归式
dp[n] = dp[n-1] + dp[n-2]

 - 递归基
dp[0] = 0
dp[1] = 1
dp[2] = 2
"""

def jump(n):
    dp = [0 for k in range(n+1)]
    for k in range(n+1):
        if k <= 2:
            dp[k] = k
        else:
            dp[k] = dp[k-1] + dp[k-2]
        print(dp)
    return dp[n]

print(jump(100))
