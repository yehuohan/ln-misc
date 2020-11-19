//! rust基本语法
//!
//! 文档注释：
//!
//! ```
//! 使用 //! 或 /*! */ 作为模块自身文档
//! 使用 /// 或 /** */ 作为模块内部结构体、函数等文档
//! 使用 //  或 /*  */ 作为普通注释
//! ```

pub mod concept;
pub mod ownership;
pub mod structure;
pub mod enumeration;

pub fn run() {
    concept::run();
    ownership::run();
    structure::run();
    enumeration::run();
}
