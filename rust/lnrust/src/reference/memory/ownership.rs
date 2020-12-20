

fn push1(s: String) {
    println!("func: {}", s);
}

fn push2(s: &String) {
    println!("func: {}", s);
}

pub fn run() {
    // 类似于push(std::move(s))
    let s = String::from("hello");
    push1(String::from("hello"));
    push1(s);

    // 类似于右值引用push(string&&)，这里是对临时值的引用
    push2(&String::from("world"));
}
