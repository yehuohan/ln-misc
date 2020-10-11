
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// cargo test -p wkslib 可以运行workspace中指定crate的测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }
}
