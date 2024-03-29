//! # 智能指针
//!
//! 智能指针通常使用结构体实现；
//! 智能指针区别于常规结构体的显著特性在于其实现了Deref和Drop trait；
//! Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用、又用于智能指针的代码；
//! Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。
//!
//! 标准库中最常用的一些智能指针：
//! ```
//!  Box<T>: 用于在堆上分配值；只能有一个所有者；在编译时检查借用规则；
//!  Rc<T>: 一个引用计数类型，其数据可以有多个所有者；其指向的数据是只读的；
//!  Ref<T>,RefMut<T>: 通过RefCell<T>访问，可以修改指向的数据（RefCell<T>在运行时检查借用规则）。
//! ```


pub mod boxt;
pub mod deref;
pub mod drop;
pub mod rc;
pub mod refcell;

pub fn run() {
    boxt::run();
    deref::run();
    drop::run();
    rc::run();
    refcell::run();
}

