use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    // let args: Vec<String> = env::args().collect();
    //
    // println!("args is：{:?}", args);
    // args is：["target/debug/minigrep", "to", "poem.txt"]

    // let config = Config::new(&args).unwrap_or_else(|err| {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        // --snip--
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
