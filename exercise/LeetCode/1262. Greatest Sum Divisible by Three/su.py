#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
https://leetcode.com/problems/greatest-sum-divisible-by-three/
"""

import os

class Solution:
    def maxSumDivThree(self, nums) -> int:
        """
        动态规划思想
        0 <= n%3 < 3
        seen[i]       : 使sum % 3 = i的最大sum
        (seen[i] + n) : 加上n后，需要更新seen[(seen[i]+n) % 3]的值

        n:      1 + 2 + 3 + 4 + 4
        s0  0   0   3   6   9   12
        s1  0   1   1   4   10  13
        s2  0   0   2   5   8   14
        """
        seen = [0, 0, 0]
        for n in nums:
            print(seen)
            for k in seen[:]:
                seen[(n + k) % 3] = max(seen[(n + k) % 3], n + k)
        print(seen)
        return seen[0]

if __name__ == "__main__":
    nums = [3, 6, 5, 1, 8]
    nums = [1, 2, 3, 4, 4]
    su = Solution()
    print(su.maxSumDivThree(nums))
