use sha2::{Sha256, Digest};
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::str::FromStr;

fn validate_depth(arg_two: &str) -> i32 {
    if let Ok(n) = arg_two.parse::<i32>() {
        return n;
    }
    if let Ok(test_float) = arg_two.parse::<f64>() {
        return test_float.round() as i32;
    }
    0
}

fn validate_args(args: &[String]) -> (String, i32, bool) {
    if args.len() == 2 {
        (args[1].clone(), 1, true)
    } else if args.len() >= 3 && validate_depth(&args[2]) > 0 {
        (args[1].clone(), validate_depth(&args[2]), true)
    } else if args.len() > 2 && validate_depth(&args[2]) == 0 {
        ("Invalid depth supplied! Program will terminate here!".to_string(), 0, false)
    } else if args.len() == 2 {
        ("No depth has been supplied! Program will terminate here!".to_string(), 0, false)
    } else {
        ("".to_string(), 0, false)
    }
}

fn crunch(mstr: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(mstr.trim().as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn copy_to_clipboard(content: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(content.to_owned()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    const HELP: &str = "Help message here";
    const DOC: &str = "Documentation message here";
    const VERSION: &str = "Version message here";

    if args.len() == 1 {
        println!("Neither seed phrase nor depth nor flag has been supplied! Program will terminate here!\nFor help, run mastergen with one of these flags:\n{}", HELP);
    } else if args.len() > 1 {
        match args[1].as_str() {
            "-d" => println!("{}", DOC),
            "-v" => println!("{}", VERSION),
            "-h" => println!("{}", HELP),
            _ => {
                let (mstr, n, result) = validate_args(&args);
                if !result {
                    println!("{}", mstr);
                } else {
                    let mut mstr = mstr;
                    for _ in 1..=n {
                        mstr = crunch(&mstr);
                    }
                    let final_output = mstr;
                    println!("{}", final_output);
                    copy_to_clipboard(&final_output);
                }
            }
        }
    }
}
