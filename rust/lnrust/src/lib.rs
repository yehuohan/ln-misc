//! 学习rust笔记
//!
//! ## 关于rust标准库
//!
//! - core: 是std库的基础，同时core不任何依赖其它库（包括系统、libc等库）
//! - std: rust标准备库，std包括core里面的内容（开发操作系统时，没法使用std，但可以使用core）
//!
//! ## 参考：
//! - [rust book](https://doc.rust-lang.org/book/)
//! - [rust book zh-cn](https://github.com/KaiserY/rust-book-chinese)
//! - [rust reference](https://doc.rust-lang.org/reference/)
//! - [rust std crate](https://doc.rust-lang.org/std/index.html)
//! - [rust官网文档](https://www.rust-lang.org/zh-CN/learn)

// 一个package有一个Cargo.toml来描述如何构建这些crate；
// 一个package可以包含多个crate bin，但至多包含一个crate lib；
// crate将一个rs文件作为root，作为模块编译的起点；
// src/main.rs表示存在一个与包同名的二进制crate，src/main.rs为crate root；
// src/lib.rs表示存在一个与包同名的库crate，src/lib.rs为crate root；

// 模块声明，内容在src/<mod>.rs或src/<mod>/mod.rs文件中
// bin(main.rs)使用mod，需要用 use lnrust::<mod>
// lib(<mod>.rs)相互之间使用mod，可以用 use crate::<mod>
pub mod basic;
pub mod collections;
pub mod panic;
pub mod generics;
pub mod autotests;
pub mod cmdbin;
pub mod functional;
pub mod smart_pointer;
pub mod concurrency;
pub mod oop;
pub mod pattern;
pub mod advanced;


pub mod info {
    pub fn name() -> &'static str {
        "lurust"
    }
    pub fn ver() -> &'static str {
        "0.1.0"
    }
}

