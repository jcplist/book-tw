use std::env;
use std::fs;
// ANCHOR: here
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析引數時出現問題：{}", err);
        process::exit(1);
    });

    // --省略--
    // ANCHOR_END: here

    println!("搜尋 {}", config.query);
    println!("目標檔案爲 {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("讀取檔案時發生了錯誤");

    println!("文字內容：\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引數不足");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
