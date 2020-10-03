//! Deref trait用于重载解引用运算符（dereference operator，即*）；
//! DerefMut trait用于重载可变引用的*运算符。
//!
//! 解引用强制多态（deref coercions）是rust在函数或方法传参上的一种便利，
//! 可以将实现了Deref的类型的引用，转换为原始类型的引用（原始类型即通过Deref所能够转换的类型)；
//! 当实参引用与形参类型不同时，解引用强制多态将自动发生，使得在函数和方法调用时，无需增加过多显式使用&和*的引用和解引用。
//!

use std::ops::Deref;
use std::fmt;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/// 实现Deref trait，需要实现deref方法，用于返回*运算符访问值的引用
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> fmt::Display for MyBox<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

pub fn run() {
    let a = 5;
    let b = &a;
    assert_eq!(5, a);
    assert_eq!(5, *b);
    //assert_eq!(5, b); // error: assert_eq中，必须保证类型是一样的
    println!("{}, {}, {}", a, *b, b);

    let a = 5;
    let b = MyBox::new(a);
    assert_eq!(5, a);
    assert_eq!(5, *b);
    //assert_eq!(5, b); // error: assert_eq中，必须保证类型是一样的
    println!("{}, {}, {}", a, *b, b);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 通过MyBox::deref将&MyBox<String>变为&String，再通过String::deref将&String变为&str
    hello(&m[..]); // 省略*运算符，解引用强制多态将自动发生
    hello(&(*m)[..]); // *m即为MyBox<String>
}
