//! 编译c语言依赖库
//!
//! ## 参考
//! - [Cargo Reference](https://doc.rust-lang.org/stable/cargo/reference/)

pub fn main() {
    // ext.c有修改时，重新运行此build脚本
    println!("cargo:rerun-if-changed=src/example/ffi/ext.c");
    
    // 使用cc crate库编译c文件，并静态链接
    cc::Build::new()
        .file("src/example/ffi/ext.c")
        .compile("ext");
}
