//! 使用unsafe关键字来切换到不安全rust，可以开启一个新的存放不安全代码的块；
//! 这里有五类可以在不安全rust中进行而不能用于安全rust的操作：
//!
//! - 解引用裸指针
//! - 调用不安全的函数或方法
//! - 访问或修改可变静态变量
//! - 实现不安全trait
//! - 访问union的字段（主要用于和C代码中的联合体交互）
//!
//! 注意：unsafe不会关闭借用检查器或禁用任何其他rust安全检查。
//!



/// rust编译器会确保引用总是有效的。
/// 但unsafe有两个被称为裸指针（raw pointers）的类似于引用的新类型；
/// 和引用一样，裸指针是不可变（*const T）或可变的（*mut T），注意：这里的星号不是解引用运算符，而是类型名称的一部分；
/// 在裸指针的上下文中，不可变意味着指针解引用之后不能直接赋值；
///
/// 裸指针与引用、智能指针的区别：
/// - 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
/// - 不保证指向有效的内存
/// - 允许为空
/// - 不能实现任何自动清理功能
fn ln_raw_pointer() {
    // 可以在安全代码中创建裸指针，但只能在unsafe中解引用裸指针
    let mut data: u8 = 5;
    let r1 = &data as *const u8; // 将不可变引用强制转换成不可变裸指针
    let r2 = &mut data as *mut u8; // 将可变引用强制转换成可变裸指针
    unsafe {
        //*r1 = 6; // error: 不可变raw pointer不能修改其指向的变量
        *r2 = 7;
        println!("data: {}, r1: {}, r2: {}", data, *r1, *r2);
    }

    let mut v: [i32; 2] = [1, 2];
    let r = &mut v[0] as *mut i32; // 创建一个指向数组首地址的祼指针
    unsafe {
        *r.add(1) *= 2;
        println!("r: {}, v: {:?}", *r.add(1), v);  // 裸指针计算
    }

    let addr: usize = 0x12345;
    let _r = addr as *mut i16; // 创建一个指向内存地址0x12345的祼指针，且将内存保存的值当作i16类型
    //unsafe { println!("r: {}", *_r); } // 不能访问0x12345
}


/// 调用不安全的函数或方法；
/// rust的关键字extern，可以用于创建和使用外部函数接口（Foreign Function Interface，FFI）；
fn ln_call() {
    unsafe {
        dangerous();
    }

    // 将unsafe代码封装到安全函数中
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v[..], 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("a: {:?}, b: {:?}", a, b);
    a[0] *= 10;
    b[0] *= 10;
    println!("v: {:?}", v);

    // 使用extern函数调用外部代码
    unsafe {
        println!("abs: {}", abs(-2));
    }
}

unsafe fn dangerous() {
    println!("This is dangerous code");
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // 获取slice的裸指针
    assert!(mid <= len);
    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid), // 使用一个裸指针和长度创建一个slice
         std::slice::from_raw_parts_mut(ptr.add(mid), len - mid)) // add用于裸指针偏移计算
    }
}

/// 在extern "C"块中，列出能够调用的另一个语言中的外部函数的签名和名称；
/// "C"部分定义了外部函数所使用的应用二进制接口（application binary interface，ABI）；
/// ABI定义了如何在汇编语言层面调用此函数；
/// "C" ABI是最常见的，即遵循C编程语言的ABI。
extern "C" {
    // 在rust中调用C标准库的abs函数
    fn abs(input: i32) -> i32;
}


/// 访问或修改可变静态变量；
/// 全局变量在rust中称为静态变量；
static mut COUNTER: u32 = 0;
fn ln_static() {
    unsafe {
        COUNTER += 10;
        println!("counter: {}", COUNTER);
    }
}


/// 实现不安全trait；
/// 当至少有一个方法中包含编译器不能验证的不变量时trait是不安全的。
fn ln_trait() {
    let a: i32 = 3;
    println!("i32 bar: {}", a.bar());
}

unsafe trait Foo { // 声明unsafe trait
    fn bar(self) -> i32;
}

unsafe impl Foo for i32 { // 实现unsafe trait
    fn bar(self) -> i32 {
        - self * self
    }
}

pub fn run() {
    ln_raw_pointer();
    ln_call();
    ln_static();
    ln_trait();
}
