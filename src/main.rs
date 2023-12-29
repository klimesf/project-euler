use std::env;
use colored::Colorize;
use crate::e1::e1;
use crate::e2::e2;
use crate::e3::e3;
use crate::e4::e4;
use crate::e5::e5;
use crate::e6::e6;
use crate::e7::e7;
use crate::e8::e8;
use crate::e9::e9;
use crate::e10::e10;
use crate::e11::e11;
use crate::e12::e12;
use crate::e13::e13;
use crate::e14::e14;
use crate::e15::e15;
use crate::e16::e16;
use crate::e17::e17;
use crate::e18::e18;
use crate::e20::e20;
use crate::e21::e21;
use crate::e22::e22;
use crate::e24::e24;
use crate::e25::e25;
use crate::e67::e67;
// use crate::e858::e858;

mod e1;
mod e2;
mod e3;
mod e4;
mod e5;
mod e6;
mod e7;
mod e8;
mod e9;
mod e10;
mod e11;
mod e12;
mod e13;
mod e14;
mod e15;
mod e16;
mod e17;
mod e18;
mod e20;
mod e21;
mod e22;
mod e24;
mod e25;
mod e67;
mod e858;

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

    if args.contains(&"all".to_string()) || args.contains(&"e6".to_string()) {
        println!("{}", format!("--- 6:").underline().green());
        measure!(e6());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e7".to_string()) {
        println!("{}", format!("--- 7:").underline().green());
        measure!(e7());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e8".to_string()) {
        println!("{}", format!("--- 8:").underline().green());
        measure!(e8());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e9".to_string()) {
        println!("{}", format!("--- 9:").underline().green());
        measure!(e9());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e10".to_string()) {
        println!("{}", format!("--- 10:").underline().green());
        measure!(e10());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e11".to_string()) {
        println!("{}", format!("--- 11:").underline().green());
        measure!(e11());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e12".to_string()) {
        println!("{}", format!("--- 12:").underline().green());
        measure!(e12());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e13".to_string()) {
        println!("{}", format!("--- 13:").underline().green());
        measure!(e13());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e14".to_string()) {
        println!("{}", format!("--- 14:").underline().green());
        measure!(e14());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e15".to_string()) {
        println!("{}", format!("--- 15:").underline().green());
        measure!(e15());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e16".to_string()) {
        println!("{}", format!("--- 16:").underline().green());
        measure!(e16());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e17".to_string()) {
        println!("{}", format!("--- 17:").underline().green());
        measure!(e17());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e18".to_string()) {
        println!("{}", format!("--- 18:").underline().green());
        measure!(e18());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e20".to_string()) {
        println!("{}", format!("--- 20:").underline().green());
        measure!(e20());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e21".to_string()) {
        println!("{}", format!("--- 21:").underline().green());
        measure!(e21());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e22".to_string()) {
        println!("{}", format!("--- 22:").underline().green());
        measure!(e22());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e24".to_string()) {
        println!("{}", format!("--- 24:").underline().green());
        measure!(e24());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e25".to_string()) {
        println!("{}", format!("--- 25:").underline().green());
        measure!(e25());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e67".to_string()) {
        println!("{}", format!("--- 67:").underline().green());
        measure!(e67());
    }

    // if args.contains(&"all".to_string()) || args.contains(&"e858".to_string()) {
    //     println!("{}", format!("--- 858:").underline().green());
    //     measure!(e858());
    // }
}
