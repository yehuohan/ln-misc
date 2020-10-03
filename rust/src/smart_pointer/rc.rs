//! Rc<T> 引用计数（reference counting）的缩写，引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。
//! 如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
//! Rc<T>在堆上分配的内存可供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它。
//! 注意： Rc<T> 只能用于单线程场景。

use std::rc::Rc;

enum ListE {
    Cons(i32, Rc<ListE>),
    Nil,
}

pub fn run() {
    use self::ListE::{Cons, Nil};
    let a = Rc::new(
        Cons(5, Rc::new(
                Cons(10, Rc::new(Nil)))));
    println!("ref cnt: {}", Rc::strong_count(&a)); // 输出a的引用计数次数

    let _b = Cons(3, Rc::clone(&a));
    println!("ref cnt: {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("ref cnt: {}", Rc::strong_count(&a));
    }
    println!("ref cnt: {}", Rc::strong_count(&a));
}
