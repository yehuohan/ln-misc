//!
//! Resource Acquisition Is Initialization
//!
//! ```
//! // 在c++中通过构造和析构函数实现
//! #include <iostream>
//!
//! class Raii {
//! private:
//!     char* raw;
//!
//! public:
//!     Raii() {
//!         this->raw = new char[128];
//!         std::cout << "Construct ...\n";
//!     }
//!
//!     ~Raii() {
//!         delete[] this->raw;
//!         std::cout << "Destruct ...\n";
//!     }
//! };
//!
//! int main(void) {
//!     {
//!         Raii r = Raii();
//!     }
//!     return 0;
//! }
//! ```


struct Raii {
    raw: &'static [u8]
}

impl Raii {
    pub fn new(raw: &'static [u8]) -> Self {
        println!("Construct ...");
        Self {
            raw
        }
    }
}

impl std::ops::Drop for Raii {
    fn drop(&mut self) {
        println!("Destruct ...: {}", self.raw[0]);
    }
}

pub fn run() {
    println!("Hello, RAII");

    {
        let _r = Raii::new(&[1, 2, 3]);
        // 退出作用域时，会自动调用drop
    }
    let r = Raii::new(&[5, 4, 3, 2, 1]);
    std::mem::drop(r); // 手动调用drop

    println!("Bye, RAII");
}
