//! Box<T>是最简单的智能指针，允许将数据放在堆上，而不是栈上；
//! 留在栈上的是指向堆数据的指针。


/// Box<T>的基本使用
fn ln_box() {
    let b = Box::new(5); // 数据5存储在堆上
    println!("b: {}", b);
} // 离开作用域时，在堆上的数据5会释放掉

/// 使用Box创建递归类型；
/// 递归类型可以按C语言链表来理解，List包含一个指向List类型的指针。
/// ```
/// struct List {
///     struct List* next;
///     int data;
/// };
/// ```
struct ListS {
    next: Option<Box<ListS>>,
    data: i32,
}

enum ListE {
    Cons(i32, Box<ListE>),
    Nil,
}

fn recursive_type() {
    // 使用ListS
    let list = ListS {
        next: Some(Box::new(ListS{
            next: Some(Box::new(ListS{
                next: None,
                data: 3,
            })),
            data: 2,
        })),
        data: 1,
    };
    if let Some(n) = list.next {
        println!("ListS: {}, {}", list.data, n.data);
    }

    // 使用ListE
    use self::ListE::{Cons, Nil};
    let list = Cons(1,
        Box::new(Cons(2,
                Box::new(Cons(3,
                        Box::new(Nil))))));
    if let Cons(i, n) = list {
        if let Cons(ii, _) = *n {
            println!("ListE: {}, {}", i, ii);
        }
    }
}


pub fn run() {
    ln_box();
    recursive_type();
}
