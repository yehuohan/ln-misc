
## C++异常处理中的栈回退机制（stack unwind）

[C++异常机制的实现方式和开销分析](http://www.baiy.cn/doc/cpp/inside_exception.htm)

异常处理涉及了函数之间的跳转（goto只是函数局部调转），比如c中用的setjump、longjump。

C++中的异常处理，是在函数调用堆栈中添加了栈回退机制。

- 栈回退需要释放throw内声明的临时变量等资源；
- 栈回退需要在当前函数中查找try对应的catch代码块，若未找到，则回退到上一级函数继续查找；

```
try {
  func();           // 栈回退实现了try中catch到func内部的throw
} catch(){...}

void func() {
  ClassA a;
  ClassB b;

  throw();          // throw后，需要释放a和b，故出现异常时，通过栈回退来调用a和b的析构函数；
                    // 若在异常，析构函数中再次引发异常，则会强行停止进程；
                    // 故在析构函数中不应该throw，即使throw也最好在析构函数内部就catch并处理；

  ClassC c;
}
```

```
#include <iostream>

class A {
    ~A() {
        // throw 1;     // 会在异常中再次引发引常
        try {
            throw 1;
        } catch (int t) {
            std::cout << t << std::endl;
        }
    }
};

void func() {
    A a();
    throw 2;
}

int main(void)
{
    try {
        func();
    } catch (int t) {
        std::cout << t << std::endl;
    }
    return 0;
}
```
