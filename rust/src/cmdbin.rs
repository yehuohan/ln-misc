//! 使用命令行参数的rust程序

// 将功能代码放到一个模块中
pub mod minigrep {
    use std::fs;
    use std::env;
    use std::error;

    #[derive(Debug)]
    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool,
    }

    impl Config {
        pub fn new(args: &[String]) -> Config {
            let query = match args.get(1) {
                Some(_) => args[1].clone(),
                None => String::from("are"),
            };
            let filename = match args.get(2) {
                Some(_) => args[2].clone(),
                None => String::from("poem.txt")
            };
            let case_sensitive = env::var("CASE_SENSITIVE").is_ok(); // 根据环境变量设置参数
            Config {query, filename, case_sensitive} // 字段初始化，变量和结构体域段同名
        }

        /// 使用迭代器改进new的实现，避使Vec和clone
        pub fn new_i(mut args: std::env::Args) -> Config {
            args.next();
            let query = match args.next() {
                Some(a) => a,
                None => String::from("are"),
            };
            let filename = match args.next() {
                Some(a) => a,
                None => String::from("poem.txt")
            };
            let case_sensitive = env::var("CASE_SENSITIVE").is_ok(); // 根据环境变量设置参数
            Config {query, filename, case_sensitive} // 字段初始化，变量和结构体域段同名
        }
    }

    pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
        let contents = fs::read_to_string(config.filename)?; // 若出错，则返回Err值
        println!("text:\n {}", contents);

        Ok(())
    }
}

pub fn main() {
    use std::env;
    use std::process;
    use minigrep::*; // 引入minigrep模块

    let args: Vec<String> = env::args().collect(); // 获取命令行参数
    let config = Config::new(&args); // 解析命令行参数
    println!("{:?}", config);

    let config = Config::new_i(env::args()); // 解析命令行参数
    println!("{:?}", config);

    if let Err(e) = run(config) {
        eprintln!("App error: {}", e); // eprintln将错误信息写入stderr，而不是stdout
        process::exit(1);
    }
}
