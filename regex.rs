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
    /// Any character except line breaks.
    Dot,
    /// A range between two characters.
    Range(char, char),
    /// An illegal token used to construct ranges.
    RangeBuilder,
    // Word,
    // Digit,
    // Whitespace,
    // NotWord,
    // NotDigit,
    // NotWhitespace,
    // /// A reference to a predefined group
    // Backreference(u8),
    /// Alternator (Boolean OR)
    Or,
}

enum NumType {
    Octal(u8),
    Hexadecimal(u8),
    Unicode(u16),
    Backreference(u8),
    Escaped(char),
}

pub fn lex_str(s: &str) -> Vec<Token> {
    use Token::*;
    let mut in_charset = false;

    let mut range = false;

    // Escaping and backreferencing
    let mut escape = false;
    let mut groups: u8 = 0;
    let mut current_sequence = String::new();

    let mut tokens = Vec::new();
    for char in s.chars() {
        if range {
            if let Char(_) = tokens.last().unwrap() {
                let Char(op2) = tokens.pop().unwrap() else {
                    unreachable!()
                };
                // token builder
                tokens.pop().unwrap();
                let Char(op1) = tokens.pop().unwrap() else {
                    unreachable!()
                };
                tokens.push(Range(op1, op2));
                range = false;
            }
        }
        if escape {
            match char {
                x if x.is_ascii_digit() => {
                    current_sequence.push(x);
                }
                _ => {}
            }
            escape = false;
            tokens.push(Char(char));
            continue;
        }
        match char {
            '\\' => {

                // // Note: only works for single chars
                // if escape {
                //     tokens.push(Char('\\'))
                // }
                // escape = !escape;
            }
            x if '0' <= x && x <= '9' => {
                if escape {
                    todo!();
                }
                tokens.push(Char(x));
            }
            '^' => {
                // if range {
                //     tokens.pop().unwrap();
                //     let Char(op1) = tokens.pop().unwrap() else {
                //         unreachable!()
                //     };
                //     tokens.push(Range(op1, '^'));
                // }
                if let Some(token) = tokens.last() {
                    if *token == CharsetOpen {
                        tokens.push(NegateCharset);
                        continue;
                    }
                }
                if in_charset {
                    tokens.push(Char('^'));
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
                    let token = tokens.last().unwrap();
                    if *token == RangeBuilder {
                        tokens.pop().unwrap();
                        tokens.push(Char('-'));
                        range = false;
                    }
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
            '|' => {
                if in_charset {
                    tokens.push(Char('|'));
                } else {
                    tokens.push(Or);
                }
            }
            '-' => {
                if in_charset {
                    // Range only possible if prev is char
                    let token = tokens.last().unwrap();
                    if let Char(_) = token {
                        range = true;
                        tokens.push(RangeBuilder);
                    } else if *token == RangeBuilder {
                        tokens.pop().unwrap();
                        let Char(op1) = tokens.pop().unwrap() else {
                            unreachable!()
                        };
                        tokens.push(Range(op1, '-'));
                        range = false;
                    } else {
                        tokens.push(Char('-'));
                    }
                } else {
                    tokens.push(Char('-'));
                }
            }
            'n' => {
                tokens.push(match escape {
                    true => {
                        escape = false;
                        Char('\n')
                    }
                    false => Char('n'),
                });
            }
            't' => {
                tokens.push(match escape {
                    true => {
                        escape = false;
                        Char('\t')
                    }
                    false => Char('t'),
                });
            }
            x => {
                tokens.push(Char(x));
            }
        }
    }
    tokens
}
