
import os
import queue
import pysnooper

class Solution:
    def getPos(self, ch) -> tuple:
        for r in range(self.n):
            for c in range(self.m):
                if self.grid[r][c] == ch:
                    return (r, c)

    def getCh(self, pos) -> str:
        return self.grid[pos[0]][pos[1]]

    def getNextPos(self, pos, direction):
        r, c = pos
        if direction == 0:
            return (r-1, c) if r - 1 >= 0 else None # up
        elif direction == 1:
            return (r, c-1) if c - 1 >= 0 else None # left
        elif direction == 2:
            return (r+1, c) if r + 1 < self.n else None # down 
        elif direction == 3:
            return (r, c+1) if c + 1 < self.m else None # right

    def check(self, cur_s, cur_b, pre_b) -> bool:
        """
        检测S(cur_s)能否从pre_b推动B(cur_b)
        """
        cur = {cur_s, }
        path = {cur_s, }
        while cur:
            tmp = []
            for ps in cur:
                for dr in (0, 1, 2, 3):
                    pnext = self.getNextPos(ps, dr)
                    # pnext没越界
                    # pnext不是墙
                    # pnext还未到达当前B的位置
                    # pnext不在已搜索的path中
                    if pnext != None and \
                            self.getCh(pnext) != '#' and \
                            pnext != cur_b and \
                            pnext not in path:
                        tmp += [pnext]
            cur = set(tmp)
            path |= cur
            # S可以在路径推动B（S若能推动B，则S, B, pnext在一条直线上）
            if pre_b in path:
                return True
        return False

    # @pysnooper.snoop()
    def minPushBox(self, grid) -> int:
        self.grid = grid
        self.n = len(grid)
        self.m = len(grid[0])
        player_pos = self.getPos("S")
        box_pos = self.getPos("B")
        pt = self.getPos("T")

        cur = {(box_pos, player_pos), }
        path = {(box_pos, player_pos), }
        steps = 0
        while cur:
            # 最短路径搜索用BFS，这里使用set代替BFS中的queue
            # 因为并不关心每一层的顺序，只关心是否到达某层（对于某一层，最小移动次数相同）
            tmp = []
            steps += 1
            for pb, ps in cur:
                for dr in (0, 1, 2, 3):
                    pnext = self.getNextPos(pb, dr)
                    ppre = self.getNextPos(pb, (dr + 2) % 4)
                    # pnext没越界
                    # pnext不是墙
                    # pnext不在已搜索的path中
                    # ps可以到达pb
                    if pnext != None and \
                            self.getCh(pnext) != '#' and \
                            (pnext, pb) not in path and \
                            self.check(ps, pb, ppre):
                        # B移到了pnext，S则必定移到了pb
                        tmp += [(pnext, pb)]    # 添加下一层需要搜索的节点
            cur = set(tmp)
            path |= cur
            for dr in (0, 1, 2, 3):
                pnext = self.getNextPos(pt, dr)
                # B到达T
                if (pt, pnext) in path:
                    return steps
        return -1

if __name__ == "__main__":
    grid = [["#","#","#","#","#","#"],
            ["#","T","#","#","#","#"],
            ["#",".",".","B",".","#"],
            ["#",".","#","#",".","#"],
            ["#",".",".",".","S","#"],
            ["#","#","#","#","#","#"]]
    # grid =  [["#","#","#","#","#","#"],
    #          ["#","T","#","#","#","#"],
    #          ["#",".",".","B",".","#"],
    #          ["#","#","#","#",".","#"],
    #          ["#",".",".",".","S","#"],
    #          ["#","#","#","#","#","#"]]
    su = Solution()
    print(su.minPushBox(grid))
