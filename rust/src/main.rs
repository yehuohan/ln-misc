use lnrust::info;
use lnrust::basic;
use lnrust::ownership;
use lnrust::structure;
use lnrust::enumeration;
use lnrust::collections;
use lnrust::panic;
use lnrust::generics;
use lnrust::traits;
use lnrust::lifetime;

pub fn main() {
    println!("{}, {}", info::name(), info::ver());

    basic::run();
    ownership::run();

    structure::run();
    enumeration::run();
    collections::run();

    panic::run();

    generics::run();
    traits::run();
    lifetime::run();
}
