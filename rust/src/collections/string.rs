//! String

pub fn run() {
    // 创建String
    let s = String::from("This is: ♥");
    println!("{}", s);

    // 从字符串常量创建String
    let mut s = "chars".to_string();

    // 字符串连接
    s.push_str(", push");
    let r = String::from(", add");
    let t = s + &r;
    //println!("{}", s); // error: s的所有权转移到了t
    println!("{}", t);

    // 遍历String
    let s = String::from("char字符");
    println!("{}", &s[0..4]);
    //println!("{}", &s[0..5]); // error: slice是字节索引，String使用UTF-8编码，'字'需要多个字节保存
    println!("{}", &s[0..7]);
    for c in s.chars() { // 使用chars()遍历字符
        print!("{}", c);
    }
    println!();
}
