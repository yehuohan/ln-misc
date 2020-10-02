//! 迭代器负责遍历序列中的每一项和决定序列何时结束的逻；
//! 迭代器实现了一个叫做Iterator的定义于标准库的trait。
//! Iterator trait需要实现next方法，返回Item类型，封装在Option中；
//! 当迭代器结束时，它返回None。
//!
//! ```
//! pub trait Iterator {
//!     type Item;
//!     fn next(&mut self) -> Option<Self::Item>;
//! }
//! ```
//!

/// 迭代器的next方法
fn iter_next() {
    // 不可变引用迭代器
    let v = vec![1, 2];
    let mut vi = v.iter(); // 使用迭代器的next方法，需要改变记录位置的状态，所以需要mut
    println!("iter: {:?}", v);
    assert_eq!(vi.next(), Some(&1));
    assert_eq!(vi.next(), Some(&2));
    assert_eq!(vi.next(), None);

    let mut v = vec![1, 2];
    let mut vi = v.iter_mut(); // 可变引用迭代器
    assert_eq!(vi.next(), Some(&mut 1));
    //println!("iter_mut: {:?}", v); // error: 可变引用vi还没用完，暂时不能借用v
    assert_eq!(vi.next(), Some(&mut 2));
    assert_eq!(vi.next(), None);
    println!("iter_mut: {:?}", v);
 
    let v = vec![1, 2];
    let mut vi = v.into_iter(); // 拥有所有权的迭代器
    //println!("into_iter: {:?}", v); // error: v的所有权转给了迭代器vi
    assert_eq!(vi.next(), Some(1));
    assert_eq!(vi.next(), Some(2));
    assert_eq!(vi.next(), None);
}

/// 实现自定议迭代器，即需要实现Iterator trait，以及next方法
struct Counter {
    pub cnt: u32,
    pub max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter{ cnt: 0 , max}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cnt += 1;

        if self.cnt < self.max {
            Some(self.cnt)
        } else {
            None
        }
    }
}


pub fn run() {
    iter_next();

    let v = vec![1, 2, 3];
    // 调用next的方法称为消费适配器（consuming adaptors）
    let total: i32 = v.iter().sum(); // 使用迭代器的sum方法来求和
    println!("iter.sum: {}", total);

    // 迭代器适配器（iterator adaptors），可以将迭代器变成不同类型的迭代器
    let v: Vec<i32> = vec![1, 2, 3];
    let vi = v.iter().map(|x| x * x); // 使用迭代器的map的方法来创建一个新的迭代器
    let u: Vec<_> = vi.collect(); // 迭代器是惰性求值的，需要使用collect方法将结果收集起来
    println!("{:?}", u);

    // 实现了Iterator trait，则可以使用标准库中定义的拥有默认实现的方法了
    let c: Vec<u32> = Counter::new(10).zip(Counter::new(9).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .collect();
    println!("cnt-vec: {:?}", c);
}
