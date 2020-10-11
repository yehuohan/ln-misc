//! enum枚举和match匹配


#[derive(Debug)]
pub enum Color {
    Black,
    White,
    GRAY(u8),
    RGB { r:u8, g:u8, b:u8 }, // 枚举成员可以关联数据
    HSV (f32, f32, f32),
}

impl Color {
    pub fn new() -> Color {
        Color::GRAY(0)
    }

    pub fn to_rgb(&self) -> Color {
        Color::RGB{r: 0, g: 0, b: 0}
    }
}

/// Option枚举定义于std中，包含Some(T)和None两个枚举值。
/// 使用Option，则表示一个数据类型T，如果有值，则包含于Some中，
/// 如果没值，则为None。
fn enum_option() {
    let a: u8 = 5;
    let b: Option<u8> = Some(5);
    //let c = a + b; // error: b是Option类型，rust不能确定它是None还是Some，不能执行加法
    let c = a + b.unwrap(); // 通过unwrap使用b包含的数据
    println!("{}", c);
}

fn enum_match() {
    let c = Color::GRAY(0xff);
    // match每个分支返回一个expression
    match c {
        Color::White => println!("White"),
        Color::Black => println!("Black"),
        Color::GRAY(0) => println!("It's Black"), // 可以精确匹配的包含在枚举中的值
        Color::GRAY(0xff) => println!("It's White"),
        Color::GRAY(val) => println!("Gray = {}", val), // 也可以将枚举中的值绑定给变量
        _ => (),
    }

    // if let语法，下面的match和if let语法等价
    let n: Option<u8> = Some(6);
    match n {
        Some(i) => println!("match {}", i),
        _ => (),
    }
    if let Some(i) = n { // n为Option枚举类型，判断Some是否包含值（即不为None）
        println!("if let: {}", i);
    }
    match n {
        Some(7) => println!("match 7"),
        _ => println!("match else"),
    }
    if let Some(7) = n { // n为Option枚举类型，判断Some包含的值是否为7
        println!("if let: 7");
    } else {
        println!("if let else");
    }
}

pub fn run() {
    enum_option();
    enum_match();
}
