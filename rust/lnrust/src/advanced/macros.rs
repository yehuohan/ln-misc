//! rust的宏（Macro）指的是 Rust 中一系列的功能：包括声明（Declarative）宏，和三种过程（Procedural）宏。
//!
//! # 声明宏
//! 在调用宏的地方，编译器会先将宏进行展开，然后再编译展开后的代码。
//!
//! 宏定义格式：
//! ```
//! macro_rules! macro_name { macro_body }
//! ```
//! macro_body和模式匹配很像，pattern => do_something；
//! 宏调用可以使用(), {}, []任一种，例如：
//! ```
//! println!();
//! vec![];
//! ```
//!
//! ## macro_name
//! 宏必须先定义，然后再调用；
//!
//! ## designator
//! 宏的变量以$开头，其余按字面匹配；
//! 以$开头的变量都是用来表示语法元素，可以使用designator限定语法元素类型；
//! 当前宏支持以下几种designator：
//!
//! - ident: 标识符，用来表示函数或变量名
//! - expr: 表达式
//! - block: 代码块，用花括号包起来的多个语句
//! - pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _)
//! - path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 std::cmp::PartialOrd
//! - tt: 单个语法树
//! - ty: 类型，语义层面的类型，如 i32, char
//! - item: 条目
//! - meta: 元条目
//! - stmt: 单条语句，如 let a = 42;
//!
//! ## repetition
//! 通过repetition可以让宏接受任意多个参数；
//! $(...)+ 一次或多次；
//! $(...)* 零次或多次；
//! $(...);+  表达式使用;分隔；
//! $(...),+  表达式使用,分隔；
//!
//! ## recursion
//! 宏定义可以调用其自身，类似于递归函数；
//!
//! ## hygienic
//! 编译器或运行时会保证宏里面定义的变量或函数不会与外面的冲突，
//! 在宏里面以普通方式定义的变量作用域不会跑到宏外面；
//!
//! ## import/export
//! 宏导入导出用#[macro_use]和#[macro_export]；
//! crate之间只有被标为#[macro_export]的宏可以被其它crate导入；
//! 宏的导入方式：
//! ```
//! #[macro_use]
//! extern crate bar;
//! // bar中的宏被导入到当前作用域
//! ```
//!
//! ## debug
//! 查看宏展开后的代码：
//! - `rustc -Z unstable-options --pretty=expanded hello.rs`
//! - `cargo rustc -- -Z unstable-options --pretty=expanded`
//!
//!
//! # 过程宏
//! 过程宏接收rust代码作为输入， 在这些代码上进行操作，然后产生另一些代码作为输出；
//!
//! 有三种类型的过程宏：
//! - 自定义#[derive]宏在结构体和枚举上指定通过derive属性添加的代码
//! - 类属性（Attribute-like）宏定义可用于任意项的自定义属性
//! - 类函数宏看起来像函数不过作用于作为参数传递的token。


macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("function {:?} is called", stringify!($func_name))
        }
    }
}

macro_rules! vector {
    ( $( $x: expr ),* ) => {
        {
            let mut tmp_vec = Vec::new();
            $(
                tmp_vec.push($x);
                println!("repetition: {}", $x);
            )* //这里会将表达式按逗号分割，然后代入$(R)*的R部分重复展开
            tmp_vec
        }
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

use procmacro::call;
use procmacro::show_attr;
use hello::Hello;
use hello_macro::HelloMacro;

/// 派生宏
#[derive(HelloMacro)]
struct Pancakes;

/// 类属性宏
#[show_attr(INDEX, "idx")]
fn index() { }


pub fn run() {
    // 声明宏
    let v: Vec<u32> = vector![1, 2, 3];
    println!("v: {:?}", v);
    create_function!(foo);
    foo();
    println!("find_mid: {}", find_min!(1, 2, 3));

    // 过程宏
    Pancakes::hello_macro();
    fn bycall() {
        println!("This is bycall function");
    }
    call!(bycall);
    index();
}
