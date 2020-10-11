//! 自动化单元测试


fn func_add(a: i32, b: i32) -> i32 {
    a + b
}

fn func_max(a: i32, b: i32) -> i32 {
    if a >= b { a } else { b }
}

pub fn run() -> bool{
    println!("add(1, 2) = {}", func_add(1, 2));
    println!("max(1, 2) = {}", func_max(1, 2));
    true
}


/// tests是用于测试autotests模块的子模块
#[cfg(test)] //cargo test时才编译此模块
mod tests {
    use super::*; // 测试autotests模块的内部函数

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn func_works() {
        assert_eq!(func_add(1, 1), 2);
        //assert_eq!(func_add(2, 2), 3, "add({},{}) should return {}", 2, 2, 4);
    }

    #[test]
    fn by_result() -> Result<(), String> { // 使Result编写测试，测试通过时返回Ok，否则返回Err
        if func_max(1, 2) >= 1 {
            Ok(())
        } else {
            Err(String::from("max(1, 2) should >= 2"))
        }
    }

    #[test]
    //#[should_panic] // 要求测试发生painc
    #[should_panic(expected="my panic")] // 要求expected是panic的信息的子串
    //#[should_panic(expected="not my panic")]
    fn it_painc() {
        panic!("this is my panic");
    }
}
