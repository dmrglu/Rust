use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;



/*******************************************************************************/
// from: https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html
// 1. Reading the Argument Values

fn main() {
    // 'collect' is a function to create many kinds of collections, so we explicitly annotate the type of args to specify that we want a vector of strings
    // Although we very rarely need to annotate types in Rust, collect is one function you do often need to annotate because Rust isn’t able to infer the kind of collection you want.
    let args: Vec<String>  = env::args().collect();
    // dbg!(args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1); // process::exit function will stop the program immediately and return the number that was passed as the exit status code
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

