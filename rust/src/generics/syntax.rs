//! 泛型基本语法

/// 范型结构体
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/// 范型结构体的方法
impl<T> Point<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }
    pub fn get_y(&self) -> &T {
        &self.y
    }
}

/// 为f32实现的方法
impl Point<f32> {
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


pub fn run() {
    let p = Point{x:3, y:4};
    println!("{:?}, x={}", p, p.get_x());
    let p = Point{x:3.0, y:4.0};
    println!("{:?}, x={}, len={}", p, p.get_x(), p.len());
}
