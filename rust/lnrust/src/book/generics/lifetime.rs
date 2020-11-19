//! 泛型生命周期与引用
//!
//! 每一个引用都有一个生命周期，使用了引用的函数或结构体，均需要指定引用的生命周期；
//! 生命周期注解描述了多个引用生命周期的相互关系，而不影响引用生命周期的长短；
//! 生命周期注解语法：
//!     &i32        // 引用
//!     &'a i32     // 带有显式生命周期的引用
//!     &'a mut i32 // 带有显式生命周期的可变引用


/// 这里告诉rust，参数中的引用和返回值引用的生命周期是一致的；
/// 若是传入函数的参数，以及对函数返回值的使用，不满足上述生命周期的说明，则会编译出错。
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//fn longest(a: &str, b: &str) -> &str { // error: rust不知道返回值引用的生命周期，所以不能确定返回值引用是否有效
    if a.len() > b.len() {
        a
    } else {
        b
    }
}


/// 带有引用的结构体，也需要注解生命周期；
/// Sub实例与part的生命周期是一样的。
struct Sub<'a> {
    part: &'a str,
}


/// 生命周期省略规则：
/// (1)每一个是引用的参数都有它自己的生命周期参数
/// (2)如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
/// (3)如果方法有多个输入生命周期参数并且其中一个参数是&self或&mut self，
///    那么所有输出生命周期被赋予self的生命周期
/// 如果对一个函数或方法应用完3条规则后，rust仍不能计算出生命周期，则编译出错（即不能省略生命周期注解）。
#[allow(dead_code)]
fn rules(s: &str) -> &str {
// 应用规则(1): fn rules(s: &'a str) -> &str
// 应用规则(2): fn rules(s: &'a str) -> &'a str
// 应用规则(1)和(2)后，rust可以计算出生命周期
    return s;
}


/// 静态生命周期用'static注解，表示其生命周期存在于整个程序运行期间。
fn max(a: &'static str, b: &'static str) -> &'static str {
    if a > b {
        return a;
    } else {
        return b;
    }
}


/// 泛型参数, trait bound, 生命周期结合使用
fn combine<'a, T>(a: &'a str, b: &'a str, c: T) -> &'a str
    where T: std::fmt::Display {
    println!("{}", c);
    longest(a, b)
}


pub fn run() {
    // 函数与引用
    let s1 = String::from("This is longer");
    let s2 = String::from("short");
    let re = longest(s1.as_str(), s2.as_str());
    //{let re = longest(s1.as_str(), s2.as_str());} // error: 后面需要用到re，但re的作用域没那么长
    println!("{}", re);

    // 结构体与引用
    let ss = String::from("Hello.world");
    let sf = ss.split('.').next().expect("can NOT find '.'");
    {
        let s = Sub{ part: sf };
        println!("{}", s.part);
    }
    //println!("{}", s.part); // error: s的作用域到不这，自然part的作用域也不能到这（即使part指向sf）

    // 静态生命周期
    println!("{}", max("hello", "world")); // 字符量常量均有'static生命周期

    println!("{}", combine(&s1[..], &s2[..], "combine"));
}
