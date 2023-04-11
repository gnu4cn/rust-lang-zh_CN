use std::env;
use std::process;

use minigrep::data_structures::Config;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln! ("解析参数时遇到问题：{err}");
            process::exit(1);
        });

    println! ("在文件 {} 中检索：{}", config.file_path, config.query);

    if let Err(e) = minigrep::run(config) {
        eprintln! ("应用程序错误：{e}");
        process::exit(1);
    }
}
