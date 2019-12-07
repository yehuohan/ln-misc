#!/usr/bin/env python3
# -*- coding: utf-8 -*-

#%% 无限循环
from itertools import *
for k in count(10, 2):
    print(k)
    break

for k in cycle(range(10)):
    print(k)
    break

for k in repeat(10, 5):
    print(k)

#%% 序列循环
from itertools import *
for k in accumulate(range(10)):
    print(k)

for k in chain(range(3), range(4)):
    print(k)

for k in compress(range(5), [1, 0, 1, 0, 1]):
    print(k)
