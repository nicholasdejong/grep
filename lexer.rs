// A regex lexer

/// A regex token, the smallest unit of information
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Beginning,
    End,
    /// The start of a charset, marked by [.
    /// Note that you can't have nested charsets.
    CharsetOpen,
    /// It's possible for a charset to be negated.
    NegateCharset,
    /// The end of a charset, marked by ].
    CharsetClose,
    /// A valid Ascii character.
    Char(char),
    /// A number from 0..=9.
    Number(char),
    /// Any character except line breaks.
    Dot,
}

pub fn lex_str(s: &str) -> Vec<Token> {
    use Token::*;

    let mut in_charset = false;
    // Note: `Vec` is probably not the best way to store tokens.
    let mut tokens = Vec::new();

    for char in s.chars() {
        match char {
            '\\' => tokens.push(Char('\\')),
            '^' => {
                if in_charset {
                    let prev = tokens.last().unwrap();
                    if *prev == CharsetOpen {
                        tokens.push(NegateCharset);
                    } else {
                        tokens.push(Char('^'));
                    }
                } else {
                    tokens.push(Beginning);
                }
            }
            '$' => {
                if in_charset {
                    tokens.push(Char('$'));
                } else {
                    tokens.push(End);
                }
            }
            '[' => {
                if in_charset {
                    tokens.push(Char('['))
                } else {
                    tokens.push(CharsetOpen);
                    in_charset = true;
                }
            }
            ']' => {
                if in_charset {
                    tokens.push(CharsetClose);
                    in_charset = false;
                } else {
                    tokens.push(Char(']'));
                }
            }
            '.' => {
                if in_charset {
                    tokens.push(Char('.'));
                } else {
                    tokens.push(Dot);
                }
            }
            '0'..='9' => tokens.push(Number(char)),
            x => {
                tokens.push(Char(x));
            }
        }
    }

    tokens
}
