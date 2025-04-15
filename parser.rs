// Parses the tokens into a syntax tree
use crate::Token;

/// A flexible type for comparing Ascii characters
/// Note: Only for the Simple Ascii Charset (0 ~ 127)
/// Can later be extended up to Unicode characters
#[derive(Debug)]
pub struct AsciiNode(u128);

impl AsciiNode {
    const fn empty() -> Self {
        Self(0)
    }
    const fn new(x: u128) -> Self {
        Self(1 << x)
    }
    const fn is_empty(&self) -> bool {
        self.0 == 0
    }
    const fn or(&mut self, x: u8) {
        self.0 |= 1 << x;
    }
    const fn or_range(&mut self, a: u8, b: u8) {
        self.0 |= (2 << b) - (1 << a);
    }
    const fn and(&mut self, x: u8) {
        self.0 &= 1 << x;
    }
    const fn and_range(&mut self, a: u8, b: u8) {
        self.0 &= (2 << b) - (1 << a);
    }
    pub const fn matches(&self, x: u8) -> bool {
        self.0 & (1 << x) > 0
    }
    const fn matches_range(&self, a: u8, b: u8) -> bool {
        self.0 & ((2 << b) - (1 << a)) > 0
    }
    pub const fn invert(&mut self) {
        self.0 = !self.0;
    }
}

impl From<char> for AsciiNode {
    fn from(value: char) -> Self {
        Self::new(value as u128)
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> Vec<AsciiNode> {
    use Token::*;
    let mut root: Vec<AsciiNode> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        match token {
            Char(x) | Number(x) => root.push((*x).into()),
            CharsetOpen => {
                let mut node = AsciiNode::empty();
                let mut j = i + 1;
                let mut negated = false;
                while j < tokens.len() {
                    let token = &tokens[j];
                    match token {
                        NegateCharset => {
                            negated = true;
                        }
                        CharsetClose => {
                            if negated {
                                node.invert();
                            }
                            root.push(node);
                            i = j;
                            break;
                        }
                        Char(x) | Number(x) => {
                            node.or(*x as u8);
                        }
                        _ => todo!(),
                    }
                    j += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    root
}
