#![allow(unused_imports)]

use lnrust::info;
use lnrust::basic;
use lnrust::collections;
use lnrust::panic;
use lnrust::generics;
use lnrust::autotests;
use lnrust::cmdbin;
use wkslib;
use lnrust::functional;
use lnrust::smart_pointer;
use lnrust::concurrency;
use lnrust::oop;
use lnrust::pattern;
use lnrust::advanced;


pub fn main() {
    println!("{}, {}", info::name(), info::ver());

    basic::run();
    collections::run();
    panic::run();
    generics::run();

    autotests::run();
    cmdbin::main();
    println!("wks: {}", wkslib::add_one(1));

    functional::run();
    smart_pointer::run();
    concurrency::run();
    oop::run();
    pattern::run();
    advanced::run();
}
