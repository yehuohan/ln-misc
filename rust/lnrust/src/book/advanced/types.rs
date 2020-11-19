//! rust高级类型


/// !是rust的特殊类型，称为never type；
/// 这里表示在函数从不返回的时候充当返回值
#[allow(dead_code)]
#[allow(unused_variables)]
fn foo() -> ! {
    let res = Result::<u32, String>::Ok(3);
    loop {
        let get = match res {
            Ok(num) => num,
            Err(_) => continue, // continue的值类型为!，不会对get赋值，同时rust可以推断get类型为u32
        };
        break;
    }
    loop {} // 没有break时，loop表达式的类型为!
}

/// Sized trait: 表示T必须是编译时已知大小的类型；
/// ?Sized trait: 表示T可能是Sized，也可能不是Sized；
/// rust会隐式的为每个泛型增加Sized bound；
fn bar<T: ?Sized + std::fmt::Display>(t: &T) {
    println!("{}", t);
}

pub fn run() {
    // 使用type定义类型别名，主要用于减少重复
    #[allow(non_camel_case_types)]
    type km = i32; // km将当作i32处理
    let a: i32 = 5;
    let b: km = 5;
    println!("a + b = {}", a + b);

    // 动态大小类型(Dynamically Sized Type, DST)
    // DST用于处理只有在运行时才知道大小的类型
    //let s: str = "Hello rust"; // error: str是一个DST，编译时不知道其大小，故不能创建str类型变量
    let s: &str = "Hello world"; // &str是字符串slice，存储了首地址和长度，可以在编译时确定大小
    bar("Hello rust");
    bar(s);
}
