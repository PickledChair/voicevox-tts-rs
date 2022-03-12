use openjtalk::OpenJTalk;

use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (dic_path, text) = if args.len() > 2 {
        (PathBuf::from(args[1].clone()), args[2].clone())
    } else {
        println!("usage: cargo run --example fullcontext -- <open_jtalk_dic_path> <text>");
        std::process::exit(1);
    };

    let ojt = match OpenJTalk::new(&dic_path) {
        Ok(ojt) => ojt,
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(1);
        }
    };
    println!("OpenJTalk initialized");
    let labels = ojt.extract_fullcontext(text);
    for label in labels {
        println!("{}", label);
    }
    ojt.delete();
}
