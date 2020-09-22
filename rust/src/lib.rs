//! 学习rust笔记
//!
//! 参考：
//! - https://doc.rust-lang.org/book/
//! - https://github.com/KaiserY/rust-book-chinese

// 模块声明，内容在src/<mod>.rs或src/<mod>/mod.rs文件中
// bin(main.rs)使用mod，需要用 use lnrust::<mod>
// lib(<mod>.rs)相互之间使用mod，可以用 use crate::<mod>
pub mod basic;
pub mod ownership;
pub mod structure;


pub mod info {
    pub fn name() -> &'static str {
        "lurst"
    }
    pub fn ver() -> &'static str {
        "0.1.0"
    }
}
