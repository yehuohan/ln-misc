//! 借用规则：
//! - 在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用之一（而不是两者）；
//! - 引用必须总是有效的；
//!
//! 对于引用和Box<T>，借用规则的不可变性作用于编译时；
//! 对RefCell<T>，借用规则的不可变性作用于运行时，违返规则时，程序会painc并退出。
//!
//! 当创建不可变和可变引用时，分别使用&和&mut语法；对于RefCell<T>来说，则是borrow和borrow_mut方法，这属于RefCell<T>安全API的一部分；
//! borrow方法返回Ref<T>类型的智能指针，borrow_mut方法返回RefMut类型的智能指针；
//!
//! RefCell<T>记录当前有多少个活动的Ref<T>和RefMut<T>智能指针；
//! 每次调用borrow，RefCell<T>将活动的不可变借用计数加一；当Ref<T>值离开作用域时，不可变借用计数减一；
//! 就像编译时借用规则一样，RefCell<T>在任何时候只允许有多个不可变借用或一个可变借用。
//!
//! 类似于 Rc<T>，RefCell<T> 只能用于单线程场景。

use std::cell::RefCell;

trait Msg {
    fn send(&self, msg: &str);
}

#[allow(dead_code)]
struct Speaker {
    sent_msg1: Vec<String>,
    sent_msg2: RefCell<Vec<String>>,
}

impl Speaker {
    fn new() -> Speaker {
        Speaker {sent_msg1: vec![], sent_msg2: RefCell::new(vec![])}
    }
}

impl Msg for Speaker {
    fn send(&self, msg: &str) {
        //self.sent_msg1.push(String::from(msg)); // error: Speaker内部也不能修改sent_msg1
        self.sent_msg2.borrow_mut().push(String::from(msg)); // 通过RefCell实现在Speaker内部修改sent_msg2
        
        let _b1 = self.sent_msg2.borrow_mut();
        //let _b2 = self.sent_msg2.borrow_mut(); // panic: RefCell<T>在运行时，最多只能有一个可变引用
    }
}

pub fn run() {
    let s = Speaker::new();
    s.send("refcell");
    println!("{:?}", s.sent_msg2);
}
