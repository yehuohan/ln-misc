//! rust结构体


/// 元组结构体Size
#[derive(Debug)] // 派生std::fmt::Debug trait
struct Size(u32, u32);

/// Img结构体
#[derive(Debug)]
pub struct Img {
    // 定义结构体{key:type, ...}
    pub tag: String, // 添加pub属性，可以由结构体直接访问
    dep: u8,
    size: Size,
    channel: u8,
}

impl Img {
    /// 关联函数，使用'::'访问
    pub fn new(tag: String) -> Img {
        // 创建结构体{key:value, ...}
        Img {
            tag, // 字段初始化，变量和结构体域段同名
            dep: 2,
            size: Size(5, 5),
            channel: 3,
        }
    }

    /// 结构体方法，使用'.'访问
    /// 自动引用和解引用：使用obj.call()时，rust自动为obj添加&, &mut或*，使用call匹配
    pub fn bytes(&self) -> usize {
        return (self.size.0 as usize) * (self.size.1 as usize) * (self.dep as usize);
    }
}


pub fn run() {
    let s = Size(2, 3);
    println!("{}, {}", s.0, s.1); // 元组结构体通过下标访问
    let Size(r, c) = s; // 元组结构体可以像tuple一样解构
    println!("{}, {}, {:?}", r, c, s);

    let v = Img::new(String::from("abc"));
    let w = Img {
        tag: String::from("xyz"),
        ..v // 使用..赋值剩余字段，会转移所有权
    };
    println!("{:?}, bytes:{}", w, w.bytes());
}
