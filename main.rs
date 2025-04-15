mod lexer;
mod matcher;
mod parser;
use lexer::*;
use matcher::find_matches;
use parser::*;
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    loop {
        println!("Haystack: ");
        stdin().read_line(&mut buf).expect("Failed to read line");
        let haystack = buf.clone();
        buf.clear();
        println!("Regex: ");
        stdin().read_line(&mut buf).expect("Failed to read line");
        let regex = buf.clone();
        buf.clear();
        println!("{haystack}, {regex}");
        println!("{:?}", find_matches(&haystack.trim(), &regex.trim()));
    }
}
