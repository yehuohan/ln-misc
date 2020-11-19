
/// 引用与解引用
///
/// - 引用： &T , &mut T
/// 保存值T的地址，占用大小为usize；当引用作为函数参数时，则称为借用；
///
/// - 切片：T[&]
/// 即 地址&T + 长度sizeof([])，长度指T的个数，占用大小为usize + usize；
///
/// - 裸指针： *const T, *mut T
/// 本质也是值T一个地址，占用大小为usize；
///
/// - 解引用： *ptr
/// 与&互为逆操作，获取引用ptr的值；
fn oprt_borrow_deref() {
    println!("ref size: {}", std::mem::size_of::<&u8>());
    println!("slice size: {}", std::mem::size_of::<&[u8]>());
    println!("raw pointer size: {}", std::mem::size_of::<*const u8>());

    let a: [u16; 3] = [2, 3, 5];
    let r = &a;
    //let r = &&a; // 多次&，也还是引用，不是多级指针！
    //let r = &&&a;
    println!("{:?}", r); // 无论多少次&，都可以自动deref
    println!("{:?}", &r); // 只是再次对r进行&，还是可以自动deref
    //println!("{:?}", &&r);
    //println!("{:?}", ***&&&*r); // *的deref次数，可以比&少，但不能多（因为&可以自动deref，而*不能自动ref）

    //let r = a.as_ptr() as *const u16;
    let r = &a as *const u16; // &a，&a[0]等同于as_ptr()，即将a的引用（地址）转化成裸指针（地址）
    //let r = &a[0] as *const u16;
    unsafe {
        println!("{:?}", *r); // 对裸指针解引用，即得到a
        println!("{:?}", &&&*r); // &是对*r的引用，即对a的&
        println!("{:?}", r); // 祼指针保存的地址，即a的地址
        println!("{:?}", &&&r); // &是对祼指针的引用，不是多级指针！
                                // 且无论多少次&，都可以自动deref（自动deref值是r，不是*r）

        let s = r.offset(1); // offset即为指针计算，结果仍为裸指针
        println!("{:?}", *s);
        println!("{:?}", *r.offset(1));
        println!("{:?}", s);  // offset的计算单位为sizeof(T=u16)
        println!("{:?}", r.offset(1));
    }
}


pub fn run() {
    oprt_borrow_deref();
}
