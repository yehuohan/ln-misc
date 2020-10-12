//! rust高级特征

pub mod unsafecode;
pub mod traits;
pub mod types;
pub mod funcs;

pub fn run() {
    unsafecode::run();
    traits::run();
    types::run();
    funcs::run();
}
