//! 闭包：匿名函数


/// 闭包语法
fn ln_closure() {
    fn  ret1   (x: u32) -> u32 { x }  // 定义函数
    let ret2 = |x: u32| -> u32 { x }; // 带完整类型注解的闭包
    let ret3 = |x|             { x }; // 省略类型注解的闭包，根据闭包调用，rust可推断类型
    let ret4 = |x|               x  ; // 闭包只有一行代码，省略大括号
    println!("closure: {}, {}, {}, {}", ret1(1), ret2(2), ret3(3), ret4(4));
    //println!("closure: {}", ret3('3')); // error: 2次调用ret3的参数类型不同，会出现类型错误
}

/// 结构体储存闭包，需要指定闭包的类型，即实现Fn系列trait
struct Calculator<T>
where T: Fn(u32) -> u32 {
    calculating: T,
    value: Option<u32>,
}

/// 使用闭包和结构体，实现惰性求值
impl<T> Calculator<T>
where T: Fn(u32) -> u32 {
    /// 泛型T是一个使用Fn trait的闭包
    fn new(calculating: T) -> Calculator<T> {
        Calculator {
            calculating,
            value: None, //执行闭包前，value的值为None
        }
    }

    /// value的函数signature应当和T一样
    /// 惰性求值：value为None时，调用calculating计算，将将结果保存在value中
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(i) => i,
            None => {
                let ret = (self.calculating)(arg);
                self.value = Some(ret);
                ret
            }
        }

    }
}

fn func_closure() {
    // 直接使用闭包
    let calc_closure = |num| {
        print!("calculating ... ");
        num * num
    };
    println!("call closure1: {}", calc_closure(5));
    println!("call closure2: {}", calc_closure(6)); // 第2次调用仍执行闭包代码

    // 使用惰性求值
    let mut calc_value = Calculator::new(|num| {
        print!("calculating ... ");
        num * num
    });
    println!("call value1: {}", calc_value.value(5));
    println!("call value2: {}", calc_value.value(6)); // 第2次调用没有执行闭包代码，但是value也无法再更改
}

/// 闭包捕获环境变量，有3种捕方式，对应3个Fn trait；
/// FnOnce: 从环境获取所有权
/// FnMut: 从环境获取可变的借用值
/// Fn: 从环境获取不可变的借用值
fn env_closure() {
    let a = String::from("FnOnce");
    let f = move || println!("{}", a); // 使用move关键字，强制转移所有权
    //println!("{}", a); // error: a的所有权已经转移
    f();

    let mut b = String::from("Fn");
    let mut f = || b.push_str("Mut"); // 获取可变借用，可以修改b的值
    f();
    println!("{}", b);
}


pub fn run() {
    ln_closure();
    func_closure();
    env_closure();
}
