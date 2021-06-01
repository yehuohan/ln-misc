//! async和await是Rust编译器的语法糖；
//! async将普通函数转批成返回Future的函数；
//! await用于获取Future中的数据；
//!
//! Rust编译器本质上是将async函数转换成一个状态机，每一个await，就是一个不同的状态；
//! （联想到使用c语言的switch case实现协程）。
//!
//! - async中才能使用await？
//! async函数才会转换成一个状态机，状态机中才有await代表的状态。
//!
//! - await用于获取Future中的值？
//! Future需要异步运行计算，这样await才能实现状态的返回。


#![allow(dead_code)]

use futures::{future, FutureExt, executor::block_on};


/// 编写返回Future函数，同样可以使用await获取Future中的数据。
pub fn aread_file(name: &str) -> impl future::Future<Output = String> {
    std::thread::sleep(std::time::Duration::from_secs(1));
    future::ready(String::from(name))
    // 和std::future::ready是一样的
    //std::future::ready(String::from(name))
}

/// 通地async语法，Rust会将返回类型转换成Future。
pub async fn awrite_file(_name: &str, _txt: String) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn exec_read(m: usize) -> impl future::Future<Output = String> {
    aread_file("foo.txt").then(move |content| {
        if content.len() < m {
            future::Either::Left(aread_file("bar.txt").map(|s| content + &s))
        } else {
            future::Either::Right(future::ready(content))
        }
    })
}

async fn exec_write() -> i32 {
    let arr = [1, 2, 3];
    let ele = &arr[2];
    awrite_file("foo.txt", ele.to_string()).await;
    *ele
}


async fn foo(idx: usize) {
    println!("foo-{}", idx);
}

async fn bar() {
    foo(0).await; // 只有一个await执行结束，才会转到下一个await（即从一个转状转到另一个状态）
    let f = foo(1);
    println!("bar");
    f.await;

    let er = exec_read(20);
    let ew = exec_write();
    println!("exec read and write:");
    println!("{}", ew.await);
    println!("{}", er.await);
}

pub fn run() {
    let b = bar();
    println!("Hello, async and await!");
    block_on(b);
}
