//! 学习[rust book](https://doc.rust-lang.org/book/)的笔记

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


#[cfg(feature = "info")]
#[cfg(feature = "ver")]
pub mod info {
    pub fn name() -> &'static str {
        "lurust"
    }
    pub fn ver() -> &'static str {
        "0.1.0"
    }
}

use wkslib;

pub fn run() {
    #[cfg(feature = "info")]
    #[cfg(feature = "ver")]
    println!("{}, {}", info::name(), info::ver());

    basic::run();
    collections::run();
    panic::run();
    generics::run();

    autotests::run();
    cmdbin::main();
    println!("wks: {}", wkslib::add_one(1));

    functional::run();
    smart_pointer::run();
    concurrency::run();
    oop::run();
    pattern::run();
    advanced::run();
}
