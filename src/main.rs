use std::env;
use colored::Colorize;
use crate::e1::e1;

mod e1;

mod utils { pub mod toolbox; }

fn main() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Project Euler").red());
    println!();

    if args.contains(&"all".to_string()) || args.contains(&"e0001".to_string()) {
        println!("{}", format!("--- 1:").underline().green());
        measure!(e1());
    }
}
