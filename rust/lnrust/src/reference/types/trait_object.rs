//! Trait Object
//!
//! Trait Object是一个动态类型（DST, Dynamically Sized Types），表示指向Trait实例的指针。
//! 一个Trait Object实际上需要2个指针数据：一个指向实现Trait类型的实例，一个vtable(virtual methon
//! table)。
//!
//! ```
//! // Trait Object的数据结构示意
//! pub struct TraitObject {
//!    pub ptr: *mut (),
//!    pub vtable: *mut (),
//! }
//! ```
//!
//! DST类型需要通过指针使用，Trait Object即通过`&dyn SomeTrait`或`Box<dyn SomeTrait>`使用。

use std::mem;
use std::any::Any;

trait TObj {
    fn func(&self);
}

struct Foo {
    name: String,
    data: usize,
}

impl TObj for Foo {
    fn func(&self) {
        println!("This is Foo");
    }
}

/// 分析Trait Object内部数据
fn trait_object_mem() {
    let foo = Foo {
        name: String::from("foo"),
        data: 123,
    };
    let pfoo = &foo; // 指向Foo实例的指针
    let pobj = pfoo as &dyn TObj; // 指向Trait Object的“胖”指针，Trait Object的ptr指向实例foo

    println!("Size(pfoo) = {}", mem::size_of_val(&pfoo));
    println!("Size(pobj) = {}", mem::size_of_val(&pobj));

    let pfoo_addr: usize = unsafe { mem::transmute(pfoo) };
    println!("Addr(pfoo) = 0x{:X}", pfoo_addr); // 此即TraitObject的ptr值

    let pfoo_fun_addr: usize = unsafe { mem::transmute(&Foo::func) };
    println!("Addr(pfoo.func) = 0x{:X}", pfoo_fun_addr); // 此（首个函数地址）即TraitObject的vtable值

    let (ptr, vtable): (usize, usize) = unsafe { mem::transmute(pobj) };
    println!("Addr(pobj) = {{ptr:0x{:X}, vtable:0x{:X}}}", ptr, vtable);
}

/// Trait Object转换成实例
fn trait_object_downcast(t: Box<dyn Any>) {
    if let Ok(dt) = t.downcast::<Foo>() {
        println!("Foo = {{name = {}, data = {}}}", (*dt).name, (*dt).data);
    }
}

pub fn run() {
    trait_object_mem();

    let foo = Foo {
        name: String::from("foo-downcast"),
        data: 123,
    };
    // 这里Box用move语义（不然编译出错），
    // 传参时，需要创建一个Box::new()，表示move的Box在stack中的内容，而heap中的不动
    trait_object_downcast(Box::new(foo))
}
