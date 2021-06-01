
use std::{boxed::Box, mem};
use core::{pin::Pin, marker::PhantomPinned};

/// SelfReferential: 引用自身
struct SelfReferential {
    self_ptr: *const Self,
}

fn selfref() {
    let mut sr: Pin<Box<SelfReferential>> = Box::pin(SelfReferential{
        self_ptr: 0 as *const _, // 先用0初始化*const Self
    }); // 通过Box将sr的数据放在heap上

    let ptr = &*sr as *const SelfReferential; // 将放在heap上的sr数据的地址转成SelfReferential地址
    sr.self_ptr = ptr; // 将self_ptr指向heap_heap，即引用自身
    println!("sr at: {:p}", sr);
    println!("sr.self_ptr at: {:p}", sr.self_ptr);

    // &mut可变引用可以修改heap上的数据
    // mem::replace将使用新的SelfReferential替换sr在heap上的数据，
    // 同时将原来的sr数据move到stack上
    let stack_sr = mem::replace(&mut *sr, SelfReferential {
        self_ptr: 0x3 as *const _,
    });
    println!("ori sr at: {:p}", &*sr); // sr本身的地址没变，但是其在heap的数据self_ptr变了
    println!("ori sr.self_ptr at: {:p}", sr.self_ptr);
    println!("stack sr at: {:p}", &stack_sr); // 这是ori sr数据在stack上的地址
    println!("stack sr.self_ptr at: {:p}", stack_sr.self_ptr); // 但是self_ptr还是指向ori sr本身的地址，而不是stack sr本身的地址
}

/// SelfReferential: 引用自身
#[allow(dead_code)]
struct SelfReferentialPin {
    self_ptr: *const Self,
    self_pin: PhantomPinned, // zero-sized marker for NOT implementing 'Unpin' trait
}

fn selfref_pin() {
    let mut heap_value: Pin<Box<SelfReferentialPin>> = Box::pin(SelfReferentialPin{
        self_ptr: 0 as *const _,
        self_pin: PhantomPinned,
    });

    let ptr = &*heap_value as *const SelfReferentialPin;
    //heap_value.self_ptr = ptr; // error
    // Pin<Box<SelfReferentialPin>>类型不再实现DerefMut trait，
    // 也就不能获取&mut，即不能直接修改self_ptr，也没法move，
    // 可以保证self_ptr一直指向heap_value本身的地址
    unsafe {
        // 这里使用get_unchecked_mut完成self_ptr对heap_value的自身引用
        let mut_ref = Pin::as_mut(&mut heap_value);
        Pin::get_unchecked_mut(mut_ref).self_ptr = ptr;
    }
    println!("heap_valuepin at: {:p}", heap_value);
    println!("heap_valuepin.self_ptr at: {:p}", heap_value.self_ptr);
}


pub fn run() {
    selfref();
    selfref_pin();
}
