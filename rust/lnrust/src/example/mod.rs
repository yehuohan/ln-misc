
mod async_await;
mod raii;

pub fn run() {
    async_await::run();
    raii::run();
}
