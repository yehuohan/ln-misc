//! rust模式


fn foo(&(x, y): &(i32, i32)) {
    println!("foo: {}, {}", x, y);
}

fn ln_let() {
    // let可以和if语句配合，用于解构
    let a: Option<&str> = None;
    let b: bool = false;
    let c: Result<u8, _> = "18".parse();
    if let Some(aa) = a {
        println!("if let: {}", aa);
    } else if b {
        println!("else if: {}", b);
    } else if let Ok(cc) = c {
        println!("else if let: {}", cc);
    } else {
        println!("else");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("while let: {}", top);
    }

    // for
    let chars = vec!['a', 'b', 'c'];
    for (i, v) in chars.iter().enumerate() {
        println!("for: {}, {}", i, v);
    }

    // let
    let (x, y) = (1, 2);
    println!("let: {}, {}", x, y);
    let point = (x, y);
    foo(&point);
}

fn ln_match_literals() {
    // 字面值
    let a = 2;
    match a {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }
}

fn ln_match_named_variables() {
    // 命名变量
    let a = Some(5);
    let b = 10;
    match a {
        Some(0) => println!("got 0"),
        Some(b) => println!("b = {:?}", b),
        _ => println!("a = {:?}", a),
    }
    println!("b = {:?}", b);
}

fn ln_match_multiple_patterns() {
    // 多个模式
    let a = 2;
    match a {
        1 | 2 | 3 => println!("one or two or three"),
        _ => println!("anything"),
    }
}

fn ln_match_range() {
    // 范围
    let a = 'a';
    match a {
        // ..= 相当于['a', ..., 'z']
        // .. 相当于['a', ..., 'z')
        'a'..='z' => println!("lower case"),
        _ => println!("anything"),
    }
}

fn ln_match_destructure() {
    // 结构体、元组
    use super::generics::syntax::Point;
    let ((a, b), Point{x: c, y: d}) = ((0, 1), Point{x: 2, y: 3});
    println!("struct and enum: {}, {}, {}, {}", a, b, c, d);

    // 枚举
    use super::basic::enumeration::Color;
    let a = Color::RGB{r:0, g:1, b:2};
    match a {
        Color::GRAY(v) => println!("color: {}", v),
        Color::RGB{r: rr, g: gg, b: bb} => println!("color: {}, {}, {}", rr, gg, bb),
        Color::HSV(h, s, v) => println!("color: {}, {}, {}", h, s, v),
        _ => (),
    }
}

fn ln_match_ignore() {
    // _忽略单个值
    let a = Some(0);
    let b: Option<i32> = None;
    match (a, b) {
        (Some(_), Some(_)) => println!("double some"),
        (Some(_), _) => println!("left some"),
        (_, Some(_)) => println!("left some"),
        _ => println!("double none"),
    }

    // ..忽略剩余值
    use super::basic::enumeration::Color;
    let a = Color::RGB{r:0, g:1, b:2};
    match a {
        Color::RGB{r: 0, ..} => println!("color-r = 0"),
        _ => (),
    }
    let a = (0, 1, 2, 3, 4, 5);
    match a {
        (0, .., 5) => println!("0 to 5"),
        _ => (),
    }
}

fn ln_match_extra_conditions() {
    // 匹配守卫（match guard）是一个指定于match分支模式之后的额外if条件
    let a = Some(9);
    let b = 9;
    match a {
        Some(n) if n < 5 => println!("less than five: {}", n),
        Some(n) if n == b => println!("equal to b: {}", b),
        Some(n) => println!("{}", n),
        _ => (),
    }
}

fn ln_match_at_bindings() {
    // 运算符（@）允许我们在创建一个存放值变量的同时，测试其值是否匹配模式
    use super::basic::enumeration::Color;
    let a = Color::RGB{r:0, g:1, b:2};
    match a {
        Color::RGB{r: _, g: _, b: bb @ 1..=3} => println!("Color-b: {}", bb),
        // 两者是等价的
        //Color::RGB{r: _, g: _, b: bb} if (1 <= bb && bb <= 3) => println!("color-b: {}", bb),
        _ => (),
    }
}

pub fn run() {
    ln_let();
    ln_match_literals();
    ln_match_named_variables();
    ln_match_multiple_patterns();
    ln_match_range();
    ln_match_destructure();
    ln_match_ignore();
    ln_match_extra_conditions();
    ln_match_at_bindings();
}
