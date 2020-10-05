//! GUI示例


pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    /// Box<dyn Draw>是一个trait对象，是Box中任何实现Draw trait的类型的替身。
    /// 只有对象安全（object safe）的trait才可以作为trait对象；
    /// 关于对象安全的更多细节，可参考[Rust RFC 255](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)。
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button({})", self.label);
    }
}
