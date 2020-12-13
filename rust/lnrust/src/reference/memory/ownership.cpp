
/*
 *
 * 参考：
 * - [一文读懂C++右值引用和std::move](https://zhuanlan.zhihu.com/p/335994370)
 */

#include <iostream>
#include <vector>
#include <string>

int ref_left(const int& val) {
    return val + 1;
}

int ref_right(int&& val) {
    val ++;
    return val;
}

int get_value() {
    static int v = 0;
    return v++;
}

void ref_left_right() {
    // 左值：可以取地址（值有变量名），位于等号左边
    // 右值：没法取地址（字面值、临时值），位于等号右边
    // 左值引用：即左值的引用，只能引用有变量名的数据
    // 右值引用：即右值的引用，只能引用没有变量名的数据

    int a = 10; // a是左值
    int &rl1 = a; // 左值引用，数据10有变量名a
    int b = get_value(); // b是左值，get_value的返回值是右值
    int &rl2 = b; // 左值引用，get_value返回的数据先保存在变量名b中，然后引用

    int &&rr1 = 5; // 右值引用，数据5没有变量名
    int &&rr2 = get_value(); // 右值引用，get_value返回的数据没有变量名，直接引用

    int &&rr3 = 5;
    // 等同于以下代码：
    int tmp = 5;
    int &&rr5 = std::move(tmp); // std::move能将左值转化成右值

    ref_left(10); // 正因为是const&，表示不会修改，所以才能传递右值（右值不能取地址，自然不能修改）
    ref_left(rl1);

    ref_right(5); // 传递右值引用也可以修改
    ref_right(get_value()); // get_value的返回值是右值
    //ref_right(rr2); // error: rr2本身是左值，因为有变量名，可以取地址
}


void run() {
    // 右值引用，可以做到对字面量、临时值的引用；
    // 而字面量、临时值没有变量名，用过之后就没法再次用到了，只能等待释放内存；
    // 所以，使用右值引用时进行复制等操作时，可以只复制字面量、临时值中的数据地址，不需要复制整个数据，以提升性能；
    std::vector<std::string> vs;
    vs.push_back(std::string("hello")); // 不需要复制"hello"到vector中，只需要复制其地址
    vs.push_back("world"); // 不需要复制"world"到vector中，只需要复制地址
    // 等同于以下代码：
    std::vector<char*> vc(2, nullptr);
    char s1[] = "hello";
    char s2[] = "world";
    char* p1 = s1;
    char* p2 = s2;
    vc[0] = p1; p1 = nullptr; // 只是将s1、s2的地址放进vector，而不能复制s1、s2具体的内存
    vc[1] = p2; p2 = nullptr;

    for (auto k : vs) std::cout << k << '\n';
    for (auto k : vc) std::cout << k << '\n';


    // 对于左值，通过std::move，同样可以实现右值引用的push_back；
    // 对于rust，这即是 ***所有权的转移***
    std::string s = std::string("hello");
    std::cout << "before mode: " << s << '\n';
    vs.push_back(std::move(s));
    std::cout << "after mode: " << s << '\n';
}


int main(void) {
    ref_left_right();
    run();
    return 0;
}
