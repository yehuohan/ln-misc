//! Vec<T>


pub fn run() {
    // 创建空列表
    let v: Vec<i32> = Vec::new();
    println!("{}", v.len());

    // 创建初始值列表
    let mut v = vec![1, 2, 3];
    println!("{}", v.len());
    
    // 列表添加删除元素
    v.push(10);
    let first = &v[0];
    println!("vec[0]: {}, vec[3]: {}", first, v[3]);
    v.pop();
    match v.get(3) {
        Some(res) => println!("get:{}", res),
        None => println!("get None"),
    }

    // 遍历列表
    for k in &v {
        print!("{} ", k);
    }
}
