use lnrust::info;
use lnrust::basic;
use lnrust::ownership;
use lnrust::structure;
use lnrust::enumeration;

pub fn main() {
    println!("{}, {}", info::name(), info::ver());
    basic::run();
    ownership::run();
    structure::run();
    enumeration::run();
}
