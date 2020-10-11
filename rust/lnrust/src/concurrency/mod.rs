//! rust中的并发编程

pub mod thread;
pub mod sync;

pub fn run() {
    thread::run();
    sync::run();
}
