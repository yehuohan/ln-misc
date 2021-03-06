//! rust基本变量操作
#![allow(unused_variables)] // 允许未使用的变量（作用于整个mod）


/// Variables and Mutability
#[allow(unused_assignments)] // 允许未使用的赋值（作用于函数）
fn ln_mut() {
    let var = 1;
    //var = 2; // error: 非mut变量不能修改值
    let var = 3; // 但是可以为变量重新赋值
    println!("{}", var);

    let mut var = "string";
    var = "new string";
    println!("{}", var);
}

/// Data Types
fn ln_type() {
    /* integer */
    let a: u8 = b'A';
    let b: u16 = 0b1101_0011_1100_0000;
    let c: i32 = -999_666_333;
    let d: isize = 0xFF;
    let e: bool = true;
    println!("{}, {}, {}, {}, {}", a, b, c, d, e);

    /* float */
    let f: f64 = 0.34;
    let g: f32 = 0.12;
    println!("{}, {}", f, g);

    /* char */
    let h: char = 'ℤ';
    let i: char = '😻';
    println!("{}, {}", h, i);
    println!(r#"
        This is raw string.
    "#);

    /* tuple */
    let tup = (10, 1.23, "string");
    let (x, _, _) = tup;
    println!("tup: {}, {}", x, tup.1);

    /* array */
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 指定元素类型和长度
    let brr = [-1; 5]; // 元素初始为-1
    println!("arr: {},{}", arr[0], brr[2]);

    /* function */
    fn add(args: i32, brgs: u16) -> usize {
        //return (args + (brgs as i32)) as usize;   // statement末尾有';'
        (args + (brgs as i32)) as usize             // expression末尾没有';'，expression的值会返回，相当有return
    }
    println!("add(-2, 1): {}", add(-2, 1));
}

/// Control Flow
fn ln_ctrl(val: isize) {
    let mut ret: isize = 0;
    // if
    if val == 0 {
        ret += 10;
    } else if val == 1 {
        ret -= 20;
    } else {
        ret = 0;
    }
    let ret: char = if ret > 0 {'+'} else {'-'};
    println!("{}", ret);

    // loop
    let mut ret: i8 = loop {
        println!("loop");
        break 5;
    };

    // while
    while ret > 0 {
        print!("{} ", ret);
        ret -= 1;
    }
    println!("while");

    // for
    for e in ['a', 'b', 'c'].iter() {
        print!("{} ", e);
    }
    for i in 1..3 { // 1 <= i < 3
        print!("{} ", i);
    }
    for i in 3..=5 { // 3 <= i <= 5
        print!("{} ", i);
    }
    println!("for");
}

pub fn run() {
    ln_mut();
    ln_type();
    ln_ctrl(0);
}
