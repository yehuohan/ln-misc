#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
给定两个单词 word1 和 word2，计算出将 word1 转换成 word2 所使用的最少操作数 。
你可以对一个单词进行如下三种操作：

 - 插入一个字符
 - 删除一个字符
 - 替换一个字符

 - 数组
dp[i][j]: len(word1)=i, len(word2)=j, 将word1转化成word2所需要的最少操作次数为dp[i][j]

 - 递归式
word1[i] == word2[j] : dp[i][j] = dp[i-1][j-1]  ，不需要调整
word1[i] != word2[j] : dp[i][j] = min(dp[i-1][j-1], dp[i][j-1], dp[i-1][j]) + 1
                       dp[i][j] = dp[i-1][j-1]+1，将字符word1[i]替换成字符word2[j]
                       dp[i][j] = dp[i][j-1]+1  ，在word1[i]未尾插入一个字符
                       dp[i][j] = dp[i-1][j]+1  ，删除字符word1[i]

 - 递归基
dp[i][0] = dp[i-1][0]
dp[0][j] = dp[0][j-1]
"""

def dist(w1, w2):
    m, n = len(w1), len(w2)
    dp = [[0 for x in range(n+1)] for x in range(m+1)]
    # 初始值
    for k in range(1, m):
        dp[k][0] = dp[k-1][0] + 1
    for k in range(1, n):
        dp[0][k] = dp[0][k-1] + 1
    # 迭代计算
    for i in range(1, m+1):
        for j in range(1, n+1):
            # dp下标和word下标的对应关系：
            # [1,m] : [0,m-1]
            # [1,n] : [0,n-1]
            if w1[i-1] == w2[j-1]:
                dp[i][j] = dp[i-1][j-1]
            else:
                dp[i][j] = min(dp[i-1][j-1], dp[i-1][j], dp[i][j-1]) + 1
    return dp[m][n]

print(dist("hello", "helloa"))
