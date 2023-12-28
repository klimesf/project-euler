use std::env;
use colored::Colorize;
use crate::e1::e1;
use crate::e2::e2;
use crate::e3::e3;
use crate::e4::e4;
use crate::e5::e5;

mod e1;
mod e2;
mod e3;
mod e4;
mod e5;

mod utils { pub mod toolbox; }

fn main() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Project Euler").red());
    println!();

    if args.contains(&"all".to_string()) || args.contains(&"e1".to_string()) {
        println!("{}", format!("--- 1:").underline().green());
        measure!(e1());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e2".to_string()) {
        println!("{}", format!("--- 2:").underline().green());
        measure!(e2());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e3".to_string()) {
        println!("{}", format!("--- 3:").underline().green());
        measure!(e3());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e4".to_string()) {
        println!("{}", format!("--- 4:").underline().green());
        measure!(e4());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e5".to_string()) {
        println!("{}", format!("--- 5:").underline().green());
        measure!(e5());
    }
}
