//! panic处理


use std::io;
use std::io::ErrorKind;
use std::fs::File;


/// 传播（propagating）错误：不好处理panic以不想中断，可以返回Result
#[allow(unused_must_use)]
fn result_return() -> Result<String, io::Error> {
    // 返回Ok或Err
    match File::open("test.rst") {
        Ok(_f) => Ok(String::from("Ok")),
        Err(e) => Err(e),
    }

    // Result之后的?运算符，可以在Result为Err时，直接将Err作为整个函数的返回
    //File::open("test.rst")?;
    //Ok(String::from("Ok"))
}

/// Result<T,E>枚举有两个值，Ok<T>和Err<E>
fn result() {
    let _f = match File::open("test.swp") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.swp") {
                Ok(fc) => fc,
                Err(e) => panic!("failed create: {:?}", e),
            },
            _ => panic!("failed open: {:?}", error),
        },
    };
    //let _f = File::open("test.md").unwrap(); // panic: unwrap用于获取Result中包含的值
                                               //        为Ok时，返回File
                                               //        为Err时，会调用panic!()
    //let _f = File::open("test.md").expect("failed open"); // panic: 类似于unwrap，可以自定义panic!的信息
}

pub fn run() {
    //panic!("just panic"); // panic: 直接中断程序运行
    result();
    if let Err(e) = result_return() {
        println!("return: {:?}", e);
    }
}
