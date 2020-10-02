#![allow(unused_imports)]

use lnrust::info;
use lnrust::basic;
use lnrust::collections;
use lnrust::panic;
use lnrust::generics;
use lnrust::autotests;
use lnrust::cmdbin;
use lnrust::functional;

pub fn main() {
    println!("{}, {}", info::name(), info::ver());

    basic::run();
    collections::run();
    panic::run();
    generics::run();

    autotests::run();
    cmdbin::main();

    functional::run();
}
