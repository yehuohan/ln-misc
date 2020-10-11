//! Drop trait允许智能指针的值在离开作用域时执行一些代码

use crate::smart_pointer::deref::MyBox;

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox drop code");
    }
}

pub fn run() {
    println!("defore MyBox");
    {
        let a = MyBox::new(String::from("drop trait"));
        print!("{}, ", a);
    }
    println!("after MyBox drop");

    let a = MyBox::new(String::from("std::mem::drop"));
    drop(a);
    //print!("{}, ", a); // error: a已经提前释放了
}
