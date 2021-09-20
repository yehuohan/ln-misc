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

trait TObj {
    fn func(&self);
}

struct Foo;

impl TObj for Foo {
    fn func(&self) {
        println!("This is Foo");
    }
}

pub fn run() {
    let foo = Foo;
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
