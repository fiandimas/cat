use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: cat <filename>");
        process::exit(1);
    }

    for file in &args[1..] {
        match fs::read_to_string(file) {
            Ok(contents) => print!("{}", contents),
            Err(error) => {
                eprintln!("error reading '{}': {}", file, error);
                process::exit(1);
            }
        }
    }
}
