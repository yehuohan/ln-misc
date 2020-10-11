//! HashMap

use std::collections::HashMap;

pub fn run() {
    // 创建空HashMap
    let mut map = HashMap::new();
    map.insert(String::from("B"), 10);
    map.insert(String::from("G"), 20);
    map.insert(String::from("R"), 30);
    if let Some(val) = map.get(&String::from("R")) {
        println!("{}", val);
    }

    // 遍历HashMap
    for (key, val) in &map {
        println!("{}: {}", key, val);
    }

    // 基于Vec创建HashMap
    let keys = vec![String::from("R"), String::from("G"), String::from("B")];
    let vals = vec![10, 20, 30];
    let map: HashMap<_,_> = keys.iter().zip(vals.iter()).collect();
    if let Some(val) = map.get(&String::from("R")) {
        println!("{}", val);
    }

    // HashMap的所有权
    let mut map = HashMap::new();
    let key = String::from("R");
    let val = 10;
    map.insert(key, val);
    //println!("{}", key); // error: String的所有权会移动到HashMap，interger的不会
    println!("{}", val);
}
