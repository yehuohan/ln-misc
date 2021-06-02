
mod async_await;
mod raii;
mod ffi;

pub fn run() {
    async_await::run();
    raii::run();
    ffi::run();
}
