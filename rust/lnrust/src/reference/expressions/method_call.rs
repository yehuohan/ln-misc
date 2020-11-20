//! 方法调用
//!
//! 对于T.method()，匹配method的顺序：
//! - 先按T, &T, &mut T的顺序
//! - 对T解引用，R=*T，再按R, &R, &mut R的顺序
//! - 再对R解引用，S=*R，再按S, &S, &mut S的顺序
//! - ......
//! - （其实最后会尝试unsized coercion）
//!

#![allow(dead_code)]
#![allow(unused_mut)]

struct Foo {}
trait Bar { fn bar(&self); }
trait Bbr { fn bar(self); }

impl Foo {
    pub fn bar(&mut self) { println!("Foo::bar(self)"); }
}
impl Bar for Foo {
    fn bar(&self) { println!("Bar::bar(&self)"); }
}
impl Bbr for Foo {
    fn bar(self) { println!("Bbr::bar(self)"); }
}

pub fn run() {
    let mut foo = Foo{};
    foo.bar(); // 第1个匹配的是Bbr::bar(foo)，即T
    //Bbr::bar(foo);
    //Bar::bar(&foo);
    //Foo::bar(&mut foo); // 可以通过完全限定语法指定method（而不用管顺序）

    let foo = Box::new(Box::new(Box::new(Foo {})));
    foo.bar(); // 自动解引用过程中，Bbr::bar是第1个匹配的
}
