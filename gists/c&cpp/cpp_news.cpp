
#include <iostream>
#include <vector>
#include <map>
#include <tuple>
#include <variant>
#include <regex>
#include <string>
#include <type_traits>
#include <functional>

typedef struct S_s {
    int a;
    char b;
    double c;
}S_t;

// 可变模板参数解包的终止函数
void print() {}
// 可变模板参数
template<typename T, typename ... Args>
void print(T first, Args ... last) {
    std::cout << first;
    print(last...);
}

template<typename T>
void output(T a) {
    std::cout << a;
}
template<typename ... Args>
void output(Args ... args) {
    // 数组初始化列表
    int res[] = {(output(args), 0)...};
}

// enum class
//enum SideOld { Left, Right };
//enum BoolOld { Wrong, Right };
enum class Side { Left, Right };
enum class Bool { Wrong, Right };
enum class Colo : char {Center, Right};

int main(int argc, char *argv[])
{
    /* c++11 */
    // lambda auto and decltype
    auto f = []{std::cout << "lambda\n";};
    decltype(f) y = f;
    y();

    // tuple
    std::tuple<int, char, double> t{1, 'a', 1.0};
    std::cout << std::get<0>(t) << std::endl;
    std::cout << std::get<1>(t) << std::endl;

    // offsetof
    std::cout << offsetof(S_t, b) << std::endl;
    std::cout << offsetof(S_t, c) << std::endl;

    // regex
    std::string s = "This test string";
    std::regex re("(\\s[a-z]*\\s)");
    for (auto i = std::sregex_iterator(s.begin(), s.end(), re);
            i != std::sregex_iterator();
            ++i) {
        std::smatch m = *i;
        std::cout << m.str() << std::endl;
    }

    // initializer_list
    std::vector<int> vec{1, 2, 3, 4, 5};
    std::map<std::string, double> mp{{"first", 3.14}, {"second", 2.718}};
    for (auto i:vec) {std::cout << i << ' ';}
    std::cout << vec[4] << std::endl;
    std::cout << mp["second"] << std::endl;

    // type_traits
    std::cout << std::boolalpha;
    std::cout << std::is_void<void>::value << std::endl;
    std::cout << std::is_same<char, Colo>::value << std::endl;
    std::cout << std::is_same<char, std::underlying_type<Colo>::type>::value << std::endl;

    // 可变模板参数
    print(10, "test", 1.23, true, false, 'c');
    std::cout << std::endl;
    output(10, "test", 1.23, true, false, 'c');

    /* c++17 */
#if 0
    // variant
    std::variant<int, float, char> v;
    v = 'a';
    std::cout << std::get<char>(v) << std::endl;
#endif
    return 0;
}
