
## 函数调用堆栈基本原理

- 几个寄存器

```
ss 栈的段地址寄存器
sp 栈的偏移地址寄存器
bp 基址指针寄存器(默认的段地址为ss)

ss:sp 表示当前的栈顶地址
ss:(bp + n) 表示从栈中访问数据
cs:ip 表示当前的代码执行地址
```

- caller调用func

在caller中会执行：

```
push arg2
push arg1
push ret
```

在func中会执行：

```
push bp
mov  bp, sp
push var1
push var2
```

函数调用堆栈：

```
| 栈底         | 高位地址
| ....         |
| arg2         |
| arg1         |
| ret          |
| caller中bp值 | <- sp   ：sp指向bp所在的地址 ('push bp')
| var1         |           且func中的bp=sp    ('mov bp, sp')
| var2         |
| ....         | 低位地址

对于func来说：
  ss:(bp + n)   bp向上获取func的返回地址和参数
  ss:(bp - n)   bp向下得到func的局部变量
  ss:bp         bp本身保存着caller中的bp值
                使用ss:(ss:(bp ± n))可以获取caller的返回地址、参数、局部变量等
```

这里只是简声描述下基本的堆栈数据，详细的涉及了其它的bx、di等现场数据。
