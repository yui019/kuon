use std::str::Chars;

use token::Token;

pub mod token;

pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn from_string(source: &'a String) -> Self {
        Self {
            chars: source.chars(),
        }
    }

    /**
     * Returns all collected chars in a string, as well as whether EOF was
     * reached while collecting
     */
    fn collect_chars(&self, n: u32) -> (String, bool) {
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
    fn collect_while<F>(&self, rule: F) -> (String, bool)
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
            self.chars.next();
        }
    }

    fn skip_to_end(&mut self) {
        loop {
            if self.chars.next().is_none() {
                break;
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.chars.next();

        // handle end of file
        if char.is_none() {
            return None;
        }

        let mut char = char.unwrap();

        // skip whitespace
        while char.is_whitespace() {
            let new_char = self.chars.next();
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

                    return Some(Token::ExclamationEquals);
                }

                return Some(Token::ExclamationMark);
            }

            '(' => return Some(Token::LeftParenNormal),
            ')' => return Some(Token::RightParenNormal),

            '*' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token::StarEquals);
                }

                return Some(Token::Star);
            }

            '+' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token::PlusEquals);
                }

                return Some(Token::Plus);
            }

            ',' => return Some(Token::Comma),

            '-' => {
                let (next1, _) = self.collect_chars(1);

                if next1 == "=" {
                    self.skip_chars(1);

                    return Some(Token::MinusEquals);
                }

                if next1 == ">" {
                    self.skip_chars(1);

                    return Some(Token::MatchArrow);
                }

                return Some(Token::Minus);
            }

            '.' => {
                if self.collect_chars(2).0 == ".." {
                    self.skip_chars(2);

                    return Some(Token::ThreeDots);
                }

                return Some(Token::Dot);
            }

            '/' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token::SlashEquals);
                }

                // if it's a comment, skip until end of line and return the
                // result of calling the function again
                if self.collect_chars(1).0 == "/" {
                    let (rest, _) = self.collect_while(|c| c != '\n');
                    self.skip_chars(rest.len() + 1);

                    return self.next();
                }

                return Some(Token::Slash);
            }

            ':' => {
                if self.collect_chars(1).0 == ":" {
                    self.skip_chars(1);

                    return Some(Token::DoubleColon);
                }

                return Some(Token::Colon);
            }

            ';' => return Some(Token::Semicolon),

            '<' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token::LessThanOrEqual);
                }

                return Some(Token::LessThan);
            }

            '=' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token::EqualsEquals);
                }

                return Some(Token::Equals);
            }

            '>' => {
                if self.collect_chars(1).0 == "=" {
                    self.skip_chars(1);

                    return Some(Token::GreaterThanOrEqual);
                }

                return Some(Token::GreaterThan);
            }

            '?' => return Some(Token::QuestionMark),

            'T' => {
                if self.collect_chars(3).0 == "his" {
                    self.skip_chars(3);

                    return Some(Token::ThisCapital);
                }
            }

            '[' => return Some(Token::LeftParenSquare),
            ']' => return Some(Token::RightParenSquare),

            'a' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "nd" {
                    self.skip_chars(2);

                    return Some(Token::And);
                }

                if next == "ny" {
                    self.skip_chars(2);

                    return Some(Token::Any);
                }

                if next == "rray" {
                    self.skip_chars(4);

                    return Some(Token::Array);
                }

                if next == "s" {
                    self.skip_chars(1);

                    return Some(Token::As);
                }
            }

            'b' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "ool" {
                    self.skip_chars(3);

                    return Some(Token::Bool);
                }
            }

            'c' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "har" {
                    self.skip_chars(3);

                    return Some(Token::Char);
                }
            }

            'e' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "lse" {
                    self.skip_chars(3);

                    return Some(Token::Else);
                }

                if next == "num" {
                    self.skip_chars(3);

                    return Some(Token::Enum);
                }
            }

            'f' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "loat" {
                    self.skip_chars(4);

                    return Some(Token::Float);
                }

                if next == "un" {
                    self.skip_chars(2);

                    return Some(Token::Fun);
                }
            }

            'i' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "f" {
                    self.skip_chars(1);

                    return Some(Token::If);
                }

                if next == "nterface" {
                    self.skip_chars(8);

                    return Some(Token::Interface);
                }

                if next == "nt" {
                    self.skip_chars(2);

                    return Some(Token::Int);
                }
            }

            'm' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "atch" {
                    self.skip_chars(4);

                    return Some(Token::Match);
                }

                if next == "ap" {
                    self.skip_chars(2);

                    return Some(Token::Map);
                }
            }

            'n' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "ot" {
                    self.skip_chars(2);

                    return Some(Token::Not);
                }

                if next == "ullable" {
                    self.skip_chars(7);

                    return Some(Token::Nullable);
                }

                if next == "ull" {
                    self.skip_chars(3);

                    return Some(Token::Null);
                }
            }

            'o' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "r" {
                    self.skip_chars(1);

                    return Some(Token::Or);
                }
            }

            'r' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "ef" {
                    self.skip_chars(2);

                    return Some(Token::Ref);
                }
            }

            's' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "tring" {
                    self.skip_chars(5);

                    return Some(Token::String);
                }

                if next == "truct" {
                    self.skip_chars(5);

                    return Some(Token::Struct);
                }
            }

            't' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "his" {
                    self.skip_chars(3);

                    return Some(Token::ThisNoncapital);
                }
            }

            'u' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "int" {
                    self.skip_chars(3);

                    return Some(Token::Uint);
                }
            }

            'v' => {
                let (next, _) = self.collect_while(|c| c.is_alphabetic());

                if next == "al" {
                    self.skip_chars(2);

                    return Some(Token::Val);
                }

                if next == "ar" {
                    self.skip_chars(2);

                    return Some(Token::Var);
                }
            }

            '{' => return Some(Token::LeftParenCurly),
            '}' => return Some(Token::RightParenCurly),

            '"' => {
                let (value, reached_eof) = self.collect_while(|c| c != '"');
                if reached_eof {
                    self.skip_to_end();

                    return Some(Token::Error(
                        "Unmatched \" at end of file".to_string(),
                    ));
                }

                self.skip_chars(value.len() + 1);

                return Some(Token::ValueString(value));
            }

            '\'' => {
                let (value, reached_eof) = self.collect_while(|c| c != '\'');
                if reached_eof {
                    self.skip_to_end();

                    return Some(Token::Error(
                        "Unmatched ' at end of file".to_string(),
                    ));
                }

                self.skip_chars(value.len() + 1);

                if value.len() == 0 {
                    return Some(Token::Error(
                        "Empty char is invalid".to_string(),
                    ));
                }

                if value.len() > 1 {
                    return Some(Token::Error(
                        "Only one character is allowed inside single quotes"
                            .to_string(),
                    ));
                }

                let char = value.chars().next().unwrap();
                return Some(Token::ValueChar(char));
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
                    return Some(Token::ValueFloat(
                        full_float.parse::<f64>().unwrap(),
                    ));
                }

                return Some(Token::ValueInt(digits.parse::<i64>().unwrap()));
            }

            _ => {}
        }

        if char.is_alphabetic() {
            let (identifier, _) =
                self.collect_while(|c| c.is_alphanumeric() || c == '_');
            self.skip_chars(identifier.len());

            // add first char to identifier
            let identifier = char.to_string() + &identifier;

            return Some(Token::ValueIdentifier(identifier));
        }

        return Some(Token::Error("Unknown token".to_string()));
    }
}
