#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import underline
from underline import *

print(underline._var())
print(underline.__var())
v = Var()
print(v.foo)
print(v._bar)       # 前导下划线不影响在类中使用
# print(_var())     # 'from <mod> import *' 这种形式不会导入下划线开头的函数
# print(__var())
