use wkslib;

/// cargo run 默认运行最顶层的二进制crate
/// cargo run -p wksbin 可以运行workspace中指定的crate
fn main() {
    println!("wks: {}", wkslib::add_one(1));
}
