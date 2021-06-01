//! Rust异步特性
//!
//! 参考：
//! [Async/Await](https://os.phil-opp.com/async-await/)
//! [Async Book](https://rust-lang.github.io/async-book)

mod aa;
mod selfref;

pub fn run() {
    aa::run();
    selfref::run();
}
