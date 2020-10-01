//! rust泛型

pub mod syntax;
pub mod traits;
pub mod lifetime;

pub fn run() {
    syntax::run();
    traits::run();
    lifetime::run();
}
