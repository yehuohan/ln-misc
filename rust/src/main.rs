use lnrust::info;
use lnrust::basic;
use lnrust::collections;
use lnrust::panic;
use lnrust::generics;
use lnrust::autotests;
use lnrust::cmdbin;

pub fn main() {
    println!("{}, {}", info::name(), info::ver());

    basic::run();
    collections::run();
    panic::run();
    generics::run();

    autotests::run();
    cmdbin::main();
}

/*
fn main() {

    // closures (lambda?)
    //generate_workout(10, 5);

    // iterator
    let v = vec![1, 2, 3];
    let mut vi = v.iter();
    assert_eq!(vi.next(), Some(&1));
    assert_eq!(vi.next(), Some(&2));
    assert_eq!(vi.next(), Some(&3));
    assert_eq!(vi.next(), None);
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    //let buf: &mut [i32];
    //let cof: [i64; 12];
    //let shf: i16 = 1;
    //for i in 12..buf.len() {
    //    let pred = cof.iter()
    //                  .zip(&buf[i - 12..i])
    //                  .map(|(&c, &s)| c * s as i64)
    //                  .sum::<i64>() >> shf;
    //}

    // smart pointor
    let b = Box::new(5);
    println!("{}", *b);
    hello("World");
    //hello(Box::new(String::from("Rust")));
    let m = String::from("Rust");
    hello(&m);
    let b = Box::new(m);
    hello(&b);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[allow(dead_code)]
fn generate_workout(intensity: u32, rand_num: u32) {
    //let expensive_closure = |num: u32| -> u32 {
    let expensive_closure = |num| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    };  // 'expensive_closure' only calculated when being called

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating with trait...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("call closure: {}", expensive_closure(intensity));
        println!("call closure: {}", expensive_closure(intensity));
        println!("call result: {}", expensive_result.value(intensity));
        println!("call result: {}", expensive_result.value(intensity));
    } else {
        if rand_num >= 3 {
            println!("Take a break");
        } else {
            println!("work out");
            expensive_closure(intensity);
            expensive_result.value(intensity);
        }
    }
}
*/
