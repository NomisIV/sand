use crate::FilePos;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Let,
    Include,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GroupType {
    Paren,
    Curly,
}

// TODO: Make this implement PartialEq
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword(Keyword),
    Nope,
    String(String),
    Variable(String),
    // Object(String),
    Number(isize), // TODO: Allow for all numbers
    Bool(bool),
    Char(char),
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

impl TokenError {
    pub fn new(msg: &str, pos: &FilePos) -> Self {
        Self {
            msg: msg.to_string(),
            pos: pos.clone(),
        }
    }
}

pub fn tokenize_str(s: &str, f: &PathBuf, r: usize, c: usize) -> Result<Vec<Token>, TokenError> {
    let mut chars = s.chars().peekable();

    let mut row = r;
    let mut col = c;

    let mut group_stack = Vec::new();

    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        let pos = FilePos::new(f, row, col);
        let token = match c {
            ' ' => {
                col += 1;
                continue;
            }
            '\n' => {
                col = 1;
                row += 1;
                continue;
            }
            '(' => {
                group_stack.push(Token {
                    r#type: TokenType::Group {
                        r#type: GroupType::Paren,
                        tokens: Vec::new(),
                    },
                    pos,
                });
                col += 1;
                continue;
            }
            ')' => {
                let group = group_stack
                    .pop()
                    .ok_or(TokenError::new("Mismatched parenthesis", &pos))?;
                if let TokenType::Group {
                    r#type: GroupType::Paren,
                    ..
                } = group.r#type
                {
                    group
                } else {
                    return Err(TokenError {
                        // TODO: Get the name right
                        msg: "Mismatched parenthesis".to_string(),
                        pos,
                    });
                }
            }
            '{' => {
                group_stack.push(Token {
                    r#type: TokenType::Group {
                        r#type: GroupType::Curly,
                        tokens: Vec::new(),
                    },
                    pos,
                });
                col += 1;
                continue;
            }
            '}' => {
                let group = group_stack
                    .pop()
                    .ok_or(TokenError::new("Mismatched parenthesis", &pos))?;
                if let TokenType::Group {
                    r#type: GroupType::Curly,
                    ..
                } = group.r#type
                {
                    group
                } else {
                    return Err(TokenError::new("Mismatched parenthesis", &pos));
                }
            }
            '0'..='9' => {
                let mut number_str = String::from(c);
                while let Some(char) = chars.peek() {
                    if *char >= '0' && *char <= '9' {
                        number_str.push(chars.next().unwrap());
                        col += 1;
                    } else {
                        break;
                    }
                }
                let r#type = TokenType::Number(number_str.parse().unwrap()); // TODO: Unwrap
                Token { r#type, pos }
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
                let r#type = match str.as_str() {
                    "let" => TokenType::Keyword(Keyword::Let),
                    "include" => TokenType::Keyword(Keyword::Include),
                    "true" => TokenType::Bool(true),
                    "false" => TokenType::Bool(false),
                    "Nope" => TokenType::Nope,
                    _ => TokenType::Variable(str),
                };
                Token { r#type, pos }
            }
            '"' => {
                let mut str = String::new();
                let mut matched = false;
                while let Some(c) = chars.next() {
                    if c == '"' {
                        matched = true;
                        break;
                    } else {
                        str.push(c);
                        col += 1;
                    }
                }
                if !matched {
                    return Err(TokenError {
                        msg: "Mismatched quotation mark".to_string(),
                        pos,
                    });
                }
                Token {
                    r#type: TokenType::String(str),
                    pos,
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
            pos: FilePos::new(f, row, col),
        });
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_keyword_let() {
        let tokens = tokenize_str("let", &PathBuf::new(), 1, 1).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::Keyword(Keyword::Let))
    }

    #[test]
    fn tokenize_string() {
        let tokens = tokenize_str("\"hello\"", &PathBuf::new(), 1, 1).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::String("hello".to_string()))
    }

    #[test]
    fn tokenize_string_with_newline() {
        let tokens = tokenize_str("\"he\\nllo\"", &PathBuf::new(), 1, 1).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::String("he\nllo".to_string()))
    }

    #[test]
    fn tokenize_string_with_escaped_quote() {
        let tokens = tokenize_str("\"he\\\"llo\"", &PathBuf::new(), 1, 1).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::String("he\"llo".to_string()))
    }

    #[test]
    fn parse_variable1() {
        let tokens = tokenize_str("foo", &PathBuf::new(), 1, 1).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::Variable("foo".to_string()))
    }

    #[test]
    fn parse_variable2() {
        let tokens = tokenize_str("foo_bar", &PathBuf::new(), 1, 1).unwrap();
        assert!(tokens.len() == 1);
        assert!(tokens.get(0).unwrap().r#type == TokenType::Variable("foo_bar".to_string()))
    }
}
