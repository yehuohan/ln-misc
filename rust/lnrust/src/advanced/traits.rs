//! 高级trait

use std::ops;
use std::fmt;

struct Prime {
    pub val: u32,
}

impl Prime {
    fn name() -> String {
        String::from("from Prime")
    }

    /// 检查是否为素数
    fn check(&self) -> bool {
        for n in 2..=(self.val / 2) {
            if self.val % n == 0 {
                return false;
            }
        }
        return true;
    }

    /// 计算下一个素数
    fn next(&mut self) -> u32 {
        loop {
            self.val += 1;
            if self.check() { return self.val } 
        }

    }
}

/// 使用type定义关联类型，实现迭代器
trait Iterator1 {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn name() -> String;
}

impl Iterator1 for Prime {
    type Item = u32; // 实现trait时，需要指定关联类型

    fn next(&mut self) -> Option<Self::Item> {
        if self.val > 10000 { return None; }
        return Some(self.next());
    }

    fn name() -> String {
        String::from("from Iterator1")
    }
}

/// 使用范型实现迭代器，可以为Prime实现不同类型的Iterator2
trait Iterator2<T=u32> { // trait的范型参数可以带默认参数；
    fn next(&mut self) -> Option<T>;
    fn name() -> String;
}

impl Iterator2 for Prime { // 等同于 impl Iterator2<u32> for Prime
    fn next(&mut self) -> Option<u32> {
        if self.val > 10000 { return None; }
        return Some(self.next());
    }

    fn name() -> String {
        String::from("from Iterator2")
    }
}

impl Iterator2<String> for Prime {
    fn next(&mut self) -> Option<String> {
        if self.val > 10000 { return None; }
        Some(std::format!("S{}", self.next()))
    }

    fn name() -> String {
        String::from("from Iterator2-String")
    }
}

/// 运算符重载
impl ops::Add for Prime {
    type Output = Prime;

    fn add(self, rhs: Prime) -> Prime {
        Prime {
            val: self.val + rhs.val,
        }
    }
}

/// trait的父trait（supertrait）；
/// 任何想实现Print的struct，均要实现Display，这样，默认实现的函数中就可以用到Display；
trait Print: fmt::Display {
    fn print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "#".repeat(len + 4));
        println!("#{}#", " ".repeat(len + 2));
        println!("# {} #", output);
        println!("#{}#", " ".repeat(len + 2));
        println!("{}", "#".repeat(len + 4));
    }
}

impl fmt::Display for Prime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Prime: {}", self.val)
    }
}

impl Print for Prime {}

/// newtype模式；
/// 因为孤儿规则（orphan rule），不能为Vec<T>上实现Display，因为Vec<T>和Display均在crate外实现；
/// 可以使用元组结构体，来变相达到目的；
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run() {
    // 关联类型，完全限定语法
    let mut p = Prime { val: 1 };
    println!("Prime val: {}", p.next()); // 默认使用Prime自身的next实现
    println!("Prime val: {}", Iterator1::next(&mut p).unwrap()); // 不同的trait有相同的next方法名，需要明确指明使用哪个trait
    println!("Prime val: {}", Iterator2::<u32>::next(&mut p).unwrap()); // 需要指明trait和类型T
    println!("Prime val: {}", Iterator2::<String>::next(&mut p).unwrap());
    println!("Prime name: {}", Prime::name());
    println!("Prime name: {}", <Prime as Iterator1>::name()); // 对于同名关联方法，需要指明trait
    println!("Prime name: {}", <Prime as Iterator2>::name());

    // 运算符重载
    println!("Prime add: {}", (Prime {val: 1} + Prime {val: 2}).val);

    // 父trait
    p.print();

    // newtype模式
    let w = Wrapper(vec![
        String::from("Hello"),
        String::from("Rust"),
    ]);
    println!("w = {}", w);
}
