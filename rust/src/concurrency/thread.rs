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

/// 使用thread::spawn创建线程
pub fn run() {
    let v = vec![-1, 2, -3, 4, -5, 6];
    let handle = thread::spawn(move || { // 将v的所有权转移到新开线程中
        for i in v {
            println!("thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }

    });
    for i in 1..5 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
