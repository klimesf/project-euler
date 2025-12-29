use crate::e1::e1;
use crate::e10::e10;
use crate::e11::e11;
use crate::e12::e12;
use crate::e13::e13;
use crate::e14::e14;
use crate::e15::e15;
use crate::e16::e16;
use crate::e17::e17;
use crate::e18::e18;
use crate::e19::e19;
use crate::e2::e2;
use crate::e20::e20;
use crate::e21::e21;
use crate::e22::e22;
use crate::e23::e23;
use crate::e24::e24;
use crate::e25::e25;
use crate::e26::e26;
use crate::e27::e27;
use crate::e28::e28;
use crate::e29::e29;
use crate::e3::e3;
use crate::e30::e30;
use crate::e31::e31;
use crate::e32::e32;
use crate::e33::e33;
use crate::e34::e34;
use crate::e35::e35;
use crate::e36::e36;
use crate::e37::e37;
use crate::e38::e38;
use crate::e39::e39;
use crate::e4::e4;
use crate::e40::e40;
use crate::e41::e41;
use crate::e42::e42;
use crate::e43::e43;
use crate::e44::e44;
use crate::e45::e45;
use crate::e46::e46;
use crate::e47::e47;
use crate::e48::e48;
use crate::e49::e49;
use crate::e5::e5;
use crate::e50::e50;
use crate::e54::e54;
use crate::e56::e56;
use crate::e58::e58;
use crate::e59::e59;
use crate::e6::e6;
use crate::e67::e67;
use crate::e7::e7;
use crate::e79::e79;
use crate::e8::e8;
use crate::e81::e81;
use crate::e82::e82;
use crate::e83::e83;
use crate::e9::e9;
use crate::e92::e92;
use crate::e96::e96;
use colored::Colorize;
use std::env;
// use crate::e858::e858;

mod e1;
mod e10;
mod e11;
mod e12;
mod e13;
mod e14;
mod e15;
mod e16;
mod e17;
mod e18;
mod e19;
mod e2;
mod e20;
mod e21;
mod e22;
mod e23;
mod e24;
mod e25;
mod e26;
mod e27;
mod e28;
mod e29;
mod e3;
mod e30;
mod e31;
mod e32;
mod e33;
mod e34;
mod e35;
mod e36;
mod e37;
mod e38;
mod e39;
mod e4;
mod e40;
mod e41;
mod e42;
mod e43;
mod e44;
mod e45;
mod e46;
mod e47;
mod e48;
mod e49;
mod e5;
mod e50;
mod e54;
mod e56;
mod e58;
mod e59;
mod e6;
mod e67;
mod e7;
mod e79;
mod e8;
mod e81;
mod e82;
mod e83;
mod e858;
mod e9;
mod e92;
mod e96;

mod utils {
    pub mod toolbox;
}

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

    if args.contains(&"all".to_string()) || args.contains(&"e19".to_string()) {
        println!("{}", format!("--- 19:").underline().green());
        measure!(e19());
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

    if args.contains(&"all".to_string()) || args.contains(&"e23".to_string()) {
        println!("{}", format!("--- 23:").underline().green());
        measure!(e23());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e24".to_string()) {
        println!("{}", format!("--- 24:").underline().green());
        measure!(e24());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e25".to_string()) {
        println!("{}", format!("--- 25:").underline().green());
        measure!(e25());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e26".to_string()) {
        println!("{}", format!("--- 26:").underline().green());
        measure!(e26());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e27".to_string()) {
        println!("{}", format!("--- 27:").underline().green());
        measure!(e27());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e28".to_string()) {
        println!("{}", format!("--- 28:").underline().green());
        measure!(e28());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e29".to_string()) {
        println!("{}", format!("--- 29:").underline().green());
        measure!(e29());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e30".to_string()) {
        println!("{}", format!("--- 30:").underline().green());
        measure!(e30());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e31".to_string()) {
        println!("{}", format!("--- 31:").underline().green());
        measure!(e31());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e32".to_string()) {
        println!("{}", format!("--- 32:").underline().green());
        measure!(e32());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e33".to_string()) {
        println!("{}", format!("--- 33:").underline().green());
        measure!(e33());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e34".to_string()) {
        println!("{}", format!("--- 34:").underline().green());
        measure!(e34());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e35".to_string()) {
        println!("{}", format!("--- 35:").underline().green());
        measure!(e35());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e36".to_string()) {
        println!("{}", format!("--- 36:").underline().green());
        measure!(e36());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e37".to_string()) {
        println!("{}", format!("--- 37:").underline().green());
        measure!(e37());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e38".to_string()) {
        println!("{}", format!("--- 38:").underline().green());
        measure!(e38());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e39".to_string()) {
        println!("{}", format!("--- 39:").underline().green());
        measure!(e39());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e40".to_string()) {
        println!("{}", format!("--- 40:").underline().green());
        measure!(e40());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e41".to_string()) {
        println!("{}", format!("--- 41:").underline().green());
        measure!(e41());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e42".to_string()) {
        println!("{}", format!("--- 42:").underline().green());
        measure!(e42());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e43".to_string()) {
        println!("{}", format!("--- 43:").underline().green());
        measure!(e43());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e44".to_string()) {
        println!("{}", format!("--- 44:").underline().green());
        measure!(e44());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e45".to_string()) {
        println!("{}", format!("--- 45:").underline().green());
        measure!(e45());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e46".to_string()) {
        println!("{}", format!("--- 46:").underline().green());
        measure!(e46());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e47".to_string()) {
        println!("{}", format!("--- 47:").underline().green());
        measure!(e47());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e48".to_string()) {
        println!("{}", format!("--- 48:").underline().green());
        measure!(e48());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e49".to_string()) {
        println!("{}", format!("--- 49:").underline().green());
        measure!(e49());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e50".to_string()) {
        println!("{}", format!("--- 50:").underline().green());
        measure!(e50());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e54".to_string()) {
        println!("{}", format!("--- 54:").underline().green());
        measure!(e54());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e56".to_string()) {
        println!("{}", format!("--- 56:").underline().green());
        measure!(e56());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e58".to_string()) {
        println!("{}", format!("--- 58:").underline().green());
        measure!(e58());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e59".to_string()) {
        println!("{}", format!("--- 59:").underline().green());
        measure!(e59());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e67".to_string()) {
        println!("{}", format!("--- 67:").underline().green());
        measure!(e67());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e79".to_string()) {
        println!("{}", format!("--- 67:").underline().green());
        measure!(e79());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e81".to_string()) {
        println!("{}", format!("--- 81:").underline().green());
        measure!(e81());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e82".to_string()) {
        println!("{}", format!("--- 82:").underline().green());
        measure!(e82());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e83".to_string()) {
        println!("{}", format!("--- 83:").underline().green());
        measure!(e83());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e92".to_string()) {
        println!("{}", format!("--- 92:").underline().green());
        measure!(e92());
    }

    if args.contains(&"all".to_string()) || args.contains(&"e96".to_string()) {
        println!("{}", format!("--- 96:").underline().green());
        measure!(e96());
    }

    // if args.contains(&"all".to_string()) || args.contains(&"e858".to_string()) {
    //     println!("{}", format!("--- 858:").underline().green());
    //     measure!(e858());
    // }
}
