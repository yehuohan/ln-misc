//! rust高级函数和闭包


fn add(a: i32, b: i32) -> i32 { a + b }

#[derive(Debug)]
struct State (u32);

#[derive(Debug)]
enum Status {
    Value(u32),
}

/// 函数指针；
/// fn为函数指针，即函数的类型； Fn为闭包trait；
/// 函数指针实现了所有三个闭包trait（Fn、FnMut和FnOnce），
/// 所以总是可以在调用期望闭包的函数时传递函数指针作为参数；
///
/// 元组结构体和元组结构体枚举，可以使用()作为初始化语法，这看起来就像函数调用；
/// 同时它们确实被实现为返回由参数构造的实例的函数，所以也被称为实现了闭包trait的函数指针；
fn ln_function_pointer(f: fn(i32, i32) -> i32, arg: (i32, i32)) {
    println!("add: {}", f(arg.0, arg.1));

    // 元组结构体
    let s = State(0);
    let seqs: Vec<State> = (1u32..5).map(State).collect();
    println!("state: {:?}, seq: {:?}", s, seqs);

    // 元组枚举
    let s = Status::Value(0);
    let seqs: Vec<Status> = (1u32..5).map(Status::Value).collect();
    println!("status: {:?}, seq: {:?}", s, seqs);
}

/// 闭包表现为trait，这意味着不能直接返回闭包；
/// 需要通过trait对象返回函数闭包；
fn ln_return_closure() -> Box<dyn Fn(i32, i32) -> i32>{
    Box::new(|a, b| a + b)
} 

pub fn run() {
    ln_function_pointer(add, (1, 2));
    println!("ret add: {}, {}", (*ln_return_closure())(1, 2), ln_return_closure()(2, 3));
}
