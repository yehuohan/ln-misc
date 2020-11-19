//! 线程
//! 编程语言有一些不同的方法来实现线程。很多操作系统提供了创建新线程的 API。这种
//!
//! 由编程语言调用操作系统API创建线程的模型有时被称为1:1，一个OS线程对应一个语言线程。
//!
//! 很多编程语言提供了自己特殊的线程实现。编程语言提供的线程被称为绿色（green）线程，使用绿色线程的语言会在不同数量的 OS 线程的上下文中执行它们。
//! 为此，绿色线程模式被称为M:N模型，M个绿色线程对应N个OS线程，这里M和N不必相同。
//!
//! rust标准库只提供了1:1线程模型实现。


use std::thread;
use std::time::Duration;

/// 使用spawn创建线程
///
/// spawn接受一个闭包或函数（不能带参数），返回值的类型，也是join的返回；
/// FnOnce需要保证闭包对于访问的外部变量拥有所有权；
///
/// ```
/// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
/// where
///     F: FnOnce() -> T,
///     F: Send + 'static,
///     T: Send + 'static,
/// ```
///
/// 设置线程函数的方法：
/// - 使用闭包；
/// - 使用函数，但是函数不能带参数；
/// - 在闭包中调用函数（间接达到传递参数的目的）；
fn ln_spawn() {
    let jh = thread::spawn (|| {
        println!("in thread closure: by closure");
        "return closure" // 返回&str
    });
    println!("{}", jh.join().unwrap());

    let jh = thread::spawn(thread_func1); // 函数不能有参数
    println!("{}", jh.join().unwrap()); // join的返回值即为thread_func1的返回

    let jh = thread::spawn(|| thread_func2("by closure")); // 在闭包里调用函数，从而传递参数
    jh.join().unwrap();

    let v = vec![-1, 2, -3, 4, -5, 6];
    let jh = thread::spawn(move || { // 使用move表进这个是个FnOnce闭包，将v的所有权转移到新开线程中
        for i in v {
            println!("thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }

    });
    for i in 1..5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    jh.join().unwrap();
}

fn thread_func1() -> i32 {
    println!("in thread func1: by fn");
    1
}

fn thread_func2(text: &str) {
    println!("in thread func2: {}", text);
}


pub fn run() {
    ln_spawn();
}
