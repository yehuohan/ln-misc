//! rust所有权机制
//!
//! 理解所有权，需要对stack和heap有一定了解，以C语言为例：
//! - stack: 函数传参、临时变量等保存在栈中
//! - heap: 使用malloc等申请的内存保存在堆中
//!
//! 在rust中，对于"值"和"变量"有以下规则：
//! - rust中的每一个值都有一个被称为其"所有者（owner）"的变量；
//! - 值在任一时刻有且只有一个所有者；
//! - 当所有者（变量）离开作用域，这个值将被丢弃。



/// 变量与值的交互
/// - move: 所有权转移
/// - clone: 复制heap中的数据
/// - copy: 复制stack上的数据
fn var_move_clone_copy() {
    let a = String::from("move"); // "move"保存在heap中，a保存在stack中
    let b = a;
    let c = b.clone(); // 在heap中复制了一份"move"，所有者为c
    //println!("{}", a); // error: "move"的所有权转给了b，通过a访问不了heap中的"move"
    println!("{}, {}", b, c);

    let x = 5;
    let y = x; // 在stack的数据会复制一份
    println!("{}, {}", x, y);
}

fn _args_ret(a: i32, s: String) -> (i32, String, String) {
    println!("func: {}, {}", a, s); // s在heap中的"hello"数据，会在函数作用域结束删除
    let r = String::from("new"); // "new"的所有权会随函数return而转移
    return (a, s, r);
}

/// 函数传参的所有权变化，与变量、值之间的交互类传
fn func_move_copy() {
    let a = 123;
    let s = String::from("hello");
    let (aa, ss, rr) = _args_ret(a, s);
    println!("call: {}", a);
    //println!("{}", s); // error: "hello"的所有权转给了函数
    println!("ret: {}, {}, {}", aa, ss, rr);
}

fn _mut_args_ret(s:&String, r:&mut String) {
    println!("func: {}, {}", s, r);
    //s.push_str("-fix"); // error: &没有mut修饰，不能修改
    r.push_str("-fix");

    //let t = String::from("ret");
    //return &t; // error: 函数不能返回引用，因为出了函数作用域，需要删除"borrow"数据
}

/// 函数传参的引用与借用(Reference, Borrow)
fn func_mut() {
    let s = String::from("borrow");
    let mut r = String::from("mutborrow");
    println!("call: {}, {}", s, r);
    _mut_args_ret(&s, &mut r); // 类似于(const string&, string& str)
    println!("ret: {}, {}", s, r); // &表示引用，使用引用的函数参数称为借用，所有权没有转移

    let r1 = &mut r;
    //let r2 = &mut r; // error: 防止Data Race，在特定作用域的特定数据，只能有一个引用
    println!("{}", r1);
    //println!("{}", r2);
}

fn _string_slice<'a>(s: &'a str, r: &'a mut String) -> (&'a str, &'a str) {
    // 'a 表示生命周期
    r.push_str("-fix");
    return (&s[0..4], &r[0..4]); // 字符串slice为&str类型
}

/// slice允许引用集合中一段连续的元素序列，而不用引用整个集合，同时不转移所有权。
fn slices() {
    let s = "&str type"; // &str类型，字面常量保存在二进制文件中
    let mut r = String::from("string type");
    _string_slice(&s, &mut r);
    println!("{}, {}", s, r)
}

pub fn run() {
    var_move_clone_copy();
    func_move_copy();
    func_mut();
    slices();
}
