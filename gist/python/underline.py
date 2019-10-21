#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
_var    : 命名约定，表示非开放接口，同时'from <mod> import *'不会导入_var
var_    : 无特殊用意，可以避免与python关键字冲突
__var   : python进行名称修饰，会添加'_<ClassName>'作为前缀
__var__ : 一般用于特殊用途，如'__init__'用于构造函数，'__call__'用于括号调用
_       : 表示临时或无意义变量

REF:
https://zhuanlan.zhihu.com/p/36173202
"""

class Var:
    def __init__(self):
        self.foo = 11
        self._bar = 22

    def __hello(self):
        return(self.foo)

def _var():
    return '_var'

def __var():
    return '__var'

if __name__ == "__main__":
    v = Var()
    print(v.foo)
    print(v._bar)
    # print(v.__hello())    # 名称修饰后，原函数没了
    print(v._Var__hello())
    for _ in range(2):
        print(_)            # 单个下划线同样可以使用，但一般用于丢弃变量值
