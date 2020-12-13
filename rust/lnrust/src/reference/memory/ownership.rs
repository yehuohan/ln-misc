

fn push(s: String) {
    println!("func: {}", s);
}

pub fn run() {
    push(String::from("world")); // 类似于push(string&&)

    let s = String::from("hello");
    push(s); // 类似于push(std::move(s))
}
