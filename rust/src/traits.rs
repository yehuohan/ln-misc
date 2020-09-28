//! Trait

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;
use std::fmt::Debug;
use crate::structure::Img;
use crate::enumeration::Color;
use crate::generics::Point;

/// 定义trait
pub trait Painter {
    fn draw(&self) -> Vec<u8>;

    fn color(&self) -> (Color, Color) {
        // 为trait的方法提供默认行为
        (Color::Black, Color::White)
    }
}

/// 为struct实现trait
impl Painter for Img {
    fn draw(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        for i in 0..self.bytes() {
            v.push(i as u8);
        }
        v
    }
}

/// trait作为参数，plot支持任何实现了Painter的参数
/// 实际上impl trait是trait bound的语法糖
fn plot(item: impl Painter) {
    println!("{:?}", item.draw());
}

/// trait bound语法，T为实现了Painter的类型，而item为类型T
fn paint<T: Painter>(item: T) {
    // paint与plot是等价的
    println!("{:?}", item.draw());
}

/// 使用'+'指定多个trait
fn show1(item: impl Painter + Display) {  }
fn show2<T: Painter + Display>(item: T) {  }

/// 使用where简化trait bound语法
fn show3<T: Painter + Display, U: Clone + Debug>(a: T, b: U) {  }
fn show4<T, U>(a: T, b:U)
    where T: Painter + Display,
          U: Clone + Debug {  }

/// 函数返回实现了Painter的类型
fn static_img() -> impl Painter {
    Img::new(String::from(""))
    //Color::Black // error: Color未实现Painter
}

/// 使用带有trait bound的泛型参数的impl块，可以有条件地只为那些实现了特定trait的类型实现方法；
/// 只有实现PartialOrd和Display的T，Point<T>才有min_distance方法。
impl<T: PartialOrd + Display> Point<T> {
    pub fn min_distance(&self) {
        if self.get_x() > self.get_y() {
            println!("min distance: {}", self.get_x());
        } else {
            println!("min distance: {}", self.get_y());
        }
    }
}


pub fn run() {
    let img = Img::new(String::from("img"));
    println!("{:?}", img.draw());
    plot(img);
}
