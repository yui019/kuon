use std::str::Chars;

use token::{Token, TokenData};

pub mod token;

#[derive(Clone)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
    current_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn from_string(source: &'a String) -> Self {
        Self {
            chars: source.chars(),
            current_line: 1,
        }
    }

    pub fn peek(&self) -> Option<Token> {
        self.clone().next()
    }

    /// Wrapper over calling .next on the chars field which increments the line
    /// counter if needed
    pub fn chars_next(&mut self) -> Option<char> {
        let c = self.chars.next();

        if c == Some('\n') {
            self.current_line += 1;
        }

        c
    }

    /**
     * Returns all collected chars in a string, as well as whether EOF was
     * reached while collecting
     */
    fn collect_chars(&mut self, n: u32) -> (String, bool) {
        let mut result = String::new();
        let mut chars_clone = self.chars.clone();

        for _ in 0..n {
            let c = chars_clone.next();

            if c.is_some() {
                result += &c.unwrap().to_string();
            } else {
                return (result, true);
            }
        }

        return (result, false);
    }

    /**
     * Returns all collected chars in a string, as well as whether EOF was
     * reached while collecting
     */
    fn collect_while<F>(&mut self, rule: F) -> (String, bool)
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        let mut chars_clone = self.chars.clone();

        loop {
            let c = chars_clone.next();

            if c.is_some() {
                if !rule(c.unwrap()) {
                    break;
                }

                result += &c.unwrap().to_string();
            } else {
                return (result, true);
            }
        }

        return (result, false);
    }

    fn skip_chars(&mut self, n: usize) {
        for _ in 0..n {
            self.chars_next();
        }
    }

    fn skip_to_end(&mut self) {
        loop {
            let new_char = self.chars_next();

            if new_char.is_none() {
                break;
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.chars_next();

        // handle end of file
        if char.is_none() {
            return None;
        }

        let mut char = char.unwrap();

        // skip whitespace
        while char.is_whitespace() {
            let new_char = self.chars_next();
            // handle end of file
            if new_char.is_none() {
                return None;
            }

            char = new_char.unwrap();
        }

        match char {
            '!' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::ExclamationEquals,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::ExclamationMark,
                    line: self.current_line,
                });
            }

            '(' => {
                return Some(Token {
                    data: TokenData::LeftParenNormal,
                    line: self.current_line,
                })
            }
            ')' => {
                return Some(Token {
                    data: TokenData::RightParenNormal,
                    line: self.current_line,
                })
            }

            '*' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::StarEquals,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::Star,
                    line: self.current_line,
                });
            }

            '+' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::PlusEquals,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::Plus,
                    line: self.current_line,
                });
            }

            ',' => {
                return Some(Token {
                    data: TokenData::Comma,
                    line: self.current_line,
                })
            }

            '-' => {
                let (next1, _) = self.collect_chars(1);

                if next1 == "=" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::MinusEquals,
                        line: self.current_line,
                    });
                }

                if next1 == ">" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::MatchArrow,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::Minus,
                    line: self.current_line,
                });
            }

            '.' => {
                if self.collect_chars(2).0 == ".." {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::ThreeDots,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::Dot,
                    line: self.current_line,
                });
            }

            '/' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::SlashEquals,
                        line: self.current_line,
                    });
                }

                // if it's a comment, skip until end of line and return the
                // result of calling the function again
                if self.collect_chars(1).0 == "/" {
                    let (rest, _) = self.collect_while(|c| c != '\n');
                    self.skip_chars(rest.len() + 1);

                    return self.next();
                }

                return Some(Token {
                    data: TokenData::Slash,
                    line: self.current_line,
                });
            }

            ':' => {
                if self.collect_chars(1).0 == ":" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::DoubleColon,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::Colon,
                    line: self.current_line,
                });
            }

            ';' => {
                return Some(Token {
                    data: TokenData::Semicolon,
                    line: self.current_line,
                })
            }

            '<' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::LessThanOrEqual,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::LessThan,
                    line: self.current_line,
                });
            }

            '=' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::EqualsEquals,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::Equals,
                    line: self.current_line,
                });
            }

            '>' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::GreaterThanOrEqual,
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::GreaterThan,
                    line: self.current_line,
                });
            }

            '?' => {
                return Some(Token {
                    data: TokenData::QuestionMark,
                    line: self.current_line,
                })
            }

            'T' => {
                if self.collect_chars(3).0 == "his" {
                    self.skip_chars(3);

                    return Some(Token {
                        data: TokenData::ThisCapital,
                        line: self.current_line,
                    });
                }
            }

            '[' => {
                return Some(Token {
                    data: TokenData::LeftParenSquare,
                    line: self.current_line,
                })
            }
            ']' => {
                return Some(Token {
                    data: TokenData::RightParenSquare,
                    line: self.current_line,
                })
            }

            'a' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "nd" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::And,
                        line: self.current_line,
                    });
                }

                if next == "ny" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::Any,
                        line: self.current_line,
                    });
                }

                if next == "rray" {
                    self.skip_chars(4);

                    return Some(Token {
                        data: TokenData::Array,
                        line: self.current_line,
                    });
                }

                if next == "s" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::As,
                        line: self.current_line,
                    });
                }
            }

            'b' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "ool" {
                    self.skip_chars(3);

                    return Some(Token {
                        data: TokenData::Bool,
                        line: self.current_line,
                    });
                }
            }

            'c' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "har" {
                    self.skip_chars(3);

                    return Some(Token {
                        data: TokenData::Char,
                        line: self.current_line,
                    });
                }
            }

            'e' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "lse" {
                    self.skip_chars(3);

                    return Some(Token {
                        data: TokenData::Else,
                        line: self.current_line,
                    });
                }

                if next == "num" {
                    self.skip_chars(3);

                    return Some(Token {
                        data: TokenData::Enum,
                        line: self.current_line,
                    });
                }
            }

            'f' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "loat" {
                    self.skip_chars(4);

                    return Some(Token {
                        data: TokenData::Float,
                        line: self.current_line,
                    });
                }

                if next == "alse" {
                    self.skip_chars(4);

                    return Some(Token {
                        data: TokenData::False,
                        line: self.current_line,
                    });
                }

                if next == "un" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::Fun,
                        line: self.current_line,
                    });
                }
            }

            'i' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "f" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::If,
                        line: self.current_line,
                    });
                }

                if next == "nterface" {
                    self.skip_chars(8);

                    return Some(Token {
                        data: TokenData::Interface,
                        line: self.current_line,
                    });
                }

                if next == "nt" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::Int,
                        line: self.current_line,
                    });
                }
            }

            'm' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "atch" {
                    self.skip_chars(4);

                    return Some(Token {
                        data: TokenData::Match,
                        line: self.current_line,
                    });
                }

                if next == "ap" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::Map,
                        line: self.current_line,
                    });
                }
            }

            'n' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "ot" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::Not,
                        line: self.current_line,
                    });
                }

                if next == "ullable" {
                    self.skip_chars(7);

                    return Some(Token {
                        data: TokenData::Nullable,
                        line: self.current_line,
                    });
                }

                if next == "ull" {
                    self.skip_chars(3);

                    return Some(Token {
                        data: TokenData::Null,
                        line: self.current_line,
                    });
                }
            }

            'o' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "r" {
                    self.skip_chars(1);

                    return Some(Token {
                        data: TokenData::Or,
                        line: self.current_line,
                    });
                }
            }

            'r' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "ef" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::Ref,
                        line: self.current_line,
                    });
                }
            }

            's' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "tring" {
                    self.skip_chars(5);

                    return Some(Token {
                        data: TokenData::String,
                        line: self.current_line,
                    });
                }

                if next == "truct" {
                    self.skip_chars(5);

                    return Some(Token {
                        data: TokenData::Struct,
                        line: self.current_line,
                    });
                }
            }

            't' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "his" {
                    self.skip_chars(3);

                    return Some(Token {
                        data: TokenData::ThisNoncapital,
                        line: self.current_line,
                    });
                }

                if next == "rue" {
                    self.skip_chars(3);

                    return Some(Token {
                        data: TokenData::True,
                        line: self.current_line,
                    });
                }
            }

            'v' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "al" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::Val,
                        line: self.current_line,
                    });
                }

                if next == "ar" {
                    self.skip_chars(2);

                    return Some(Token {
                        data: TokenData::Var,
                        line: self.current_line,
                    });
                }
            }

            '{' => {
                return Some(Token {
                    data: TokenData::LeftParenCurly,
                    line: self.current_line,
                })
            }
            '}' => {
                return Some(Token {
                    data: TokenData::RightParenCurly,
                    line: self.current_line,
                })
            }

            '"' => {
                let (value, reached_eof) = self.collect_while(|c| c != '"');
                if reached_eof {
                    self.skip_to_end();

                    return Some(Token {
                        data: TokenData::Error(
                            "Unmatched \" at end of file".to_string(),
                        ),
                        line: self.current_line,
                    });
                }

                self.skip_chars(value.len() + 1);

                return Some(Token {
                    data: TokenData::ValueString(value),
                    line: self.current_line,
                });
            }

            '\'' => {
                let (value, reached_eof) = self.collect_while(|c| c != '\'');
                if reached_eof {
                    self.skip_to_end();

                    return Some(Token {
                        data: TokenData::Error(
                            "Unmatched ' at end of file".to_string(),
                        ),
                        line: self.current_line,
                    });
                }

                self.skip_chars(value.len() + 1);

                if value.len() == 0 {
                    return Some(Token {
                        data: TokenData::Error(
                            "Empty char is invalid".to_string(),
                        ),
                        line: self.current_line,
                    });
                }

                if value.len() > 1 {
                    return Some(Token { data: TokenData::Error(
                        "Only one character is allowed inside single quotes"
                            .to_string(),
                    ), line: self.current_line });
                }

                let char = value.chars().next().unwrap();
                return Some(Token {
                    data: TokenData::ValueChar(char),
                    line: self.current_line,
                });
            }

            '0'..='9' => {
                let (digits, _) = self.collect_while(|c| c.is_numeric());
                self.skip_chars(digits.len());

                // add first char to digits
                let digits = char.to_string() + &digits;

                let (next, _) = self.collect_chars(1);

                if next == "." {
                    self.skip_chars(1);

                    let (fractional_part, _) =
                        self.collect_while(|c| c.is_numeric());

                    self.skip_chars(fractional_part.len());

                    let full_float = digits + "." + &fractional_part;
                    return Some(Token {
                        data: TokenData::ValueFloat(
                            full_float.parse::<f64>().unwrap(),
                        ),
                        line: self.current_line,
                    });
                }

                return Some(Token {
                    data: TokenData::ValueInt(digits.parse::<i64>().unwrap()),
                    line: self.current_line,
                });
            }

            _ => {}
        }

        if char.is_alphabetic() {
            let (identifier, _) =
                self.collect_while(|c| c.is_alphanumeric() || c == '_');
            self.skip_chars(identifier.len());

            // add first char to identifier
            let identifier = char.to_string() + &identifier;

            return Some(Token {
                data: TokenData::ValueIdentifier(identifier),
                line: self.current_line,
            });
        }

        return Some(Token {
            data: TokenData::Error("Unknown token".to_string()),
            line: self.current_line,
        });
    }
}
