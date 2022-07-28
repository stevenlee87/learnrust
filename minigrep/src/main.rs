use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
    在上面的示例中，使用了一个之前没有详细说明的方法：unwrap_or_else，它定义于标准库的 Result<T, E> 上。
    使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理。当 Result 是 Ok 时，
    这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。然而，当其值是 Err 时，该方法会调用一个 闭包（closure），
    也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。第十三章 会更详细的介绍闭包。
    现在你需要理解的是 unwrap_or_else 会将 Err 的内部值，也就是示例 12-9 中增加的 not enough arguments
    静态字符串的情况，传递给闭包中位于两道竖线间的参数 err。闭包中的代码在其运行时可以使用这个 err 值。

    我们新增了一个 use 行来从标准库中导入 process。在错误的情况闭包中将被运行的代码只有两行：我们打印出了 err 值，
    接着调用了 std::process::exit。process::exit 会立即停止程序并将传递给它的数字作为退出状态码。
    这类似于示例 12-8 中使用的基于 panic! 的错误处理，除了不会再得到所有的额外输出了。
     */
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
