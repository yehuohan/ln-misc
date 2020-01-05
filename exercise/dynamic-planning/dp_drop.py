
#!/usr/bin/env python3

"""
你面前有一栋从 1 到 N 共 N 层的楼，然后给你 K 个鸡蛋（K 至少为 1）。
现在确定这栋楼存在楼层 0 <= F <= N，在这层楼将鸡蛋扔下去，鸡蛋恰好没
摔碎（高于 F 的楼层都会碎，低于 F 的楼层都不会碎）。现在问你，最坏情
况下，你至少要扔几次鸡蛋，才能确定这个楼层 F 呢？

dp[k, n]: 当鸡蛋有k个，需要测试的楼层数为n层时，最坏情况下至少需要扔dp(k, n)次，才能确定F
dp[1, n]: 只有1个鸡蛋，最坏情况下至少需要n次
dp[k, 1]: 只有1层，最坏情况下至少需要1次
dp[i, j] = min(max(dp[i-1, c-1], dp[i, n-c]) + 1), 1<=c<=n

        j
    0 0 0 0 0
    0 1 2 3 4
  i 0 1 x
    0 1
"""

dp = dict()
def drop(k:int, n:int):
    if k == 1: return n
    if n == 0: return 0

    # 利用历史数据
    if (k, n) in dp.keys():
        return dp[(k, n)]

    res = float('INF')
    for c in range(1, n + 1):
        # min: 至少需要
        # c可以选择从1到n，最少的那个选择，就是至少需要扔的次数
        res = min(res,
                # max: 最坏情况
                # 选择c扔，无论碎不碎，最坏情况下，需要扔的次数是较多的那个
                max(drop(k - 1, c - 1), # 碎了
                    drop(k, n - c)      # 没碎
                    ) + 1               # 扔了一次
                )
    dp[(k, n)] = int(res)
    return res

print(drop(9, 100))
