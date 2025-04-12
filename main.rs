mod regex;
use regex::*;
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    loop {
        stdin().read_line(&mut buf).expect("Failed to read line");
        let tokens = lex_str(&buf.trim_end());
        println!("{tokens:?}");
        buf.clear();
    }
}
