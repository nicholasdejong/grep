// Returns all matches with a regex
use crate::{lex_str, parse_tokens};

pub fn find_matches(haystack: &str, regex: &str) -> Vec<usize> {
    let tokens = lex_str(regex);
    let root = parse_tokens(tokens);
    let mut matches: Vec<usize> = Vec::new();
    let mut i = 0;
    'outer: while i < haystack.len() {
        let mut j = 0;
        while j < root.len() {
            if i + j >= haystack.len() {
                i += 1;
                continue 'outer;
            }
            if !root[j].matches(haystack.chars().nth(i + j).unwrap() as u8) {
                i += 1;
                continue 'outer;
            }
            j += 1;
        }
        matches.push(i);
        i += root.len()
    }
    matches
}
