use std::fmt;

use crate::FilePos;

#[derive(Debug, PartialEq, Clone)]
pub enum GroupType {
    Paren,
    Brack,
    Curly,
}

impl From<char> for GroupType {
    fn from(c: char) -> Self {
        match c {
            '(' | ')' => GroupType::Paren,
            '[' | ']' => GroupType::Brack,
            '{' | '}' => GroupType::Curly,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    StringLit(String), // "hello"
    CharLit(char),     // 'c'
    Number(f64),       // 69
    String(String),    // foo
    Char(char),        // +
    Group {
        r#type: GroupType,
        tokens: Vec<Token>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub pos: FilePos,
}

#[derive(Debug, PartialEq)]
pub struct TokenError {
    pub msg: String,
    pub pos: FilePos,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.pos, self.msg)
    }
}

impl TokenError {
    pub fn new(msg: &str, pos: &FilePos) -> Self {
        Self {
            msg: msg.to_string(),
            pos: pos.clone(),
        }
    }
}

pub fn tokenize_str(str: &str, pos: FilePos) -> Result<Vec<Token>, TokenError> {
    let mut chars = str.chars().peekable();

    let mut row = pos.row;
    let mut col = pos.col;

    let mut group_stack = Vec::new();

    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        let pos = FilePos::new(&pos.file, row, col);
        let token = match c {
            ' ' | '\t' => {
                col += 1;
                continue;
            }
            '\n' => {
                col = 1;
                row += 1;
                continue;
            }
            '#' => {
                while let Some(char) = chars.peek() {
                    if char != &'\n' {
                        chars.next();
                    } else {
                        break;
                    }
                }
                continue;
            }
            '(' | '[' | '{' => {
                group_stack.push(Token {
                    r#type: TokenType::Group {
                        r#type: GroupType::from(c),
                        tokens: Vec::new(),
                    },
                    pos,
                });
                col += 1;
                continue;
            }
            ')' | ']' | '}' => {
                let group = group_stack
                    .pop()
                    .ok_or(TokenError::new("Mismatched parenthesis", &pos))?;
                if let TokenType::Group { ref r#type, .. } = group.r#type {
                    if r#type == &GroupType::from(c) {
                        group
                    } else {
                        return Err(TokenError {
                            // TODO: Get the name right
                            msg: "Mismatched parenthesis".to_string(),
                            pos,
                        });
                    }
                } else {
                    unreachable!()
                }
            }
            '0'..='9' => {
                // TODO: This is fucking ugly
                let mut number_str = String::from(c);
                let mut trail_dot = false;
                while let Some(char) = chars.peek() {
                    if char >= &'0' && char <= &'9' {
                        number_str.push(chars.next().unwrap());
                        col += 1;
                    } else if char == &'.' {
                        chars.next().unwrap(); // Consume '.'
                        if let Some(char) = chars.peek() {
                            if char >= &'0' && char <= &'9' {
                                number_str.push('.');
                            } else {
                                trail_dot = true;
                                let r#type = TokenType::Number(
                                    number_str
                                        .parse()
                                        .map_err(|_err| TokenError::new("Bad int", &pos))?,
                                );
                                let token = Token { r#type, pos: pos.clone() };
                                if group_stack.is_empty() {
                                    tokens.push(token);
                                } else {
                                    let mut group = group_stack.pop().unwrap();
                                    if let TokenType::Group { ref mut tokens, .. } = group.r#type {
                                        tokens.push(token);
                                        group_stack.push(group);
                                    }
                                }
                                break;
                            }
                        } else {
                            return Err(TokenError {
                                msg: "Unexpected end of number".to_string(),
                                pos,
                            });
                        }
                    } else {
                        break;
                    }
                }
                if trail_dot {
                    let r#type = TokenType::Char('.');
                    Token { r#type, pos }
                } else {
                    let r#type = TokenType::Number(
                        number_str
                            .parse()
                            .map_err(|_err| TokenError::new("Bad int", &pos))?,
                    );
                    Token { r#type, pos }
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut str = String::from(c);
                while let Some(char) = chars.peek() {
                    if (char >= &'a' && char <= &'z')
                        || (char >= &'A' && char <= &'Z')
                        || char == &'_'
                    {
                        str.push(chars.next().unwrap());
                        col += 1;
                    } else {
                        break;
                    }
                }
                let r#type = TokenType::String(str);
                Token { r#type, pos }
            }
            '"' => {
                let mut str = String::new();
                let mut matched = false;
                while let Some(c) = chars.next() {
                    if c == '"' {
                        matched = true;
                        break;
                    } else if c == '\\' {
                        let ch = if let Some(c) = chars.next() {
                            match c {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                '"' => '"',
                                _ => {
                                    return Err(TokenError {
                                        msg: "Unknown escaped character".to_string(),
                                        pos,
                                    })
                                }
                            }
                        } else {
                            return Err(TokenError {
                                msg: "Expected an escaped character but found nothing".to_string(),
                                pos,
                            });
                        };
                        str.push(ch);
                    } else {
                        str.push(c);
                        col += 1;
                    }
                }
                if !matched {
                    return Err(TokenError {
                        msg: "Mismatched double quotation mark".to_string(),
                        pos,
                    });
                }
                Token {
                    r#type: TokenType::StringLit(str),
                    pos,
                }
            }
            '\'' => {
                let ch = if let Some(char) = chars.next() {
                    if char == '\\' {
                        if let Some(char) = chars.next() {
                            match char {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                '\'' => '\'',
                                _ => {
                                    return Err(TokenError {
                                        msg: "Unknown escaped character".to_string(),
                                        pos,
                                    })
                                }
                            }
                        } else {
                            return Err(TokenError {
                                msg: "Expected an escaped character but found nothing".to_string(),
                                pos,
                            });
                        }
                    } else {
                        char
                    }
                } else {
                    return Err(TokenError {
                        msg: "Mismatched single quotation mark".to_string(),
                        pos,
                    });
                };

                if let Some(char) = chars.next() {
                    if char == '\'' {
                        Token {
                            r#type: TokenType::CharLit(ch),
                            pos,
                        }
                    } else {
                        return Err(TokenError {
                            msg: "Mismatched single quotation mark".to_string(),
                            pos,
                        });
                    }
                } else {
                    return Err(TokenError {
                        msg: "Mismatched single quotation mark".to_string(),
                        pos,
                    });
                }
            }
            c => Token {
                r#type: TokenType::Char(c),
                pos,
            },
        };

        if group_stack.is_empty() {
            tokens.push(token);
        } else {
            let mut group = group_stack.pop().unwrap();
            if let TokenType::Group { ref mut tokens, .. } = group.r#type {
                tokens.push(token);
                group_stack.push(group);
            }
        }

        col += 1;
    }

    if !group_stack.is_empty() {
        return Err(TokenError {
            msg: "Mismatched parenthesis".to_string(),
            pos,
        });
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_string_lit() {
        let tokens = tokenize_str("\"hello\"", FilePos::internal()).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::StringLit("hello".to_string()))
    }

    #[test]
    fn tokenize_string_lit_newline() {
        let tokens = tokenize_str("\"he\\nllo\"", FilePos::internal()).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::StringLit("he\nllo".to_string()))
    }

    #[test]
    fn tokenize_string_lit_escaped_quote() {
        let tokens = match tokenize_str("\"he\\\"llo\"", FilePos::internal()) {
            Ok(tokens) => tokens,
            Err(err) => panic!("{}", err),
        };
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::StringLit("he\"llo".to_string()))
    }

    #[test]
    fn tokenize_char_lit() {
        let tokens = tokenize_str("'c'", FilePos::internal()).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::CharLit('c'))
    }

    #[test]
    fn tokenize_char_lit_newline() {
        let tokens = match tokenize_str("'\\n'", FilePos::internal()) {
            Ok(tokens) => tokens,
            Err(err) => panic!("{}", err),
        };
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::CharLit('\n'))
    }

    #[test]
    fn tokenize_char_lit_escaped_quote() {
        let tokens = match tokenize_str("'\\''", FilePos::internal()) {
            Ok(tokens) => tokens,
            Err(err) => panic!("{}", err),
        };
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::CharLit('\''))
    }

    #[test]
    fn tokenize_number_int() {
        let tokens = tokenize_str("5", FilePos::internal()).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::Number(5.0))
    }

    #[test]
    fn tokenize_number_float() {
        let tokens = tokenize_str("5.0", FilePos::internal()).unwrap();
        println!("Tokens: {:?}", tokens);
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::Number(5.0))
    }

    #[test]
    fn tokenize_number_complex1() {
        let tokens = tokenize_str("5. ", FilePos::internal()).unwrap();
        println!("Tokens: {:?}", tokens);
        assert!(tokens.len() == 2);
        assert!(tokens.get(0).unwrap().r#type == TokenType::Number(5.0));
        assert!(tokens.get(1).unwrap().r#type == TokenType::Char('.'));
    }

    #[test]
    fn tokenize_string() {
        let tokens = tokenize_str("foo", FilePos::internal()).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::String("foo".to_string()))
    }

    #[test]
    fn tokenize_string_complex() {
        let tokens = tokenize_str("fo_o", FilePos::internal()).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::String("fo_o".to_string()))
    }

    #[test]
    fn tokenize_char() {
        let tokens = tokenize_str(".", FilePos::internal()).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::Char('.'))
    }

    #[test]
    fn tokenize_group() {
        let tokens = tokenize_str("()", FilePos::internal()).unwrap();
        assert!(tokens.len() == 1);
        assert!(
            tokens.get(0).unwrap().r#type
                == TokenType::Group {
                    r#type: GroupType::Paren,
                    tokens: Vec::new(),
                }
        )
    }
}
