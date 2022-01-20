use std::{path::PathBuf, fs, fmt::{self, Debug, Display}};

#[derive(Debug)]
pub enum Keyword {
    Let
}

#[derive(Debug, PartialEq)]
pub enum GroupType {
    None,
    Paren,
    Curly,
}

#[derive(Debug)]
pub struct Group {
    r#type: GroupType,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum TokenType {
    Keyword(Keyword),
    String(String),
    Variable(String),
    // Object(String),
    Number(f64),
    Char(char),
    Group(Group),
}

pub struct CharPos {
    file: PathBuf,
    row: usize,
    r#col: usize,
}

impl Display for CharPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.row, self.col)
    }
}

impl CharPos {
    pub fn new(file: PathBuf, row: usize, col: usize) -> Self {
        Self { file, row, col }
    }
}

pub struct Token {
    token_type: TokenType,
    pos: CharPos,
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}

pub struct TokenError {
    msg: String,
    pos: CharPos,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.pos, self.msg)
    }
}

pub fn tokenize_file(f: PathBuf) -> Result<Vec<Token>, TokenError> {
    let file_name = f.file_name().unwrap().to_str().unwrap().to_owned();
    let file_contents = fs::read_to_string(&f).map_err(|err| TokenError {
        msg: format!("Cannot read file, because of:\n{}", err),
        pos: CharPos::new(file_name.clone().into(), 0, 0)
    })?;

    let mut chars = file_contents.chars().peekable();

    let mut row = 1;
    let mut col = 1;

    let mut group_stack = Vec::new();

    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        let token_type = match c {
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
                group_stack.push(Group { r#type: GroupType::Paren, tokens: Vec::new() });
                col += 1;
                continue;
            },
            ')' => {
                let group = group_stack.pop().ok_or(TokenError {
                    msg: "Mismatched parenthesis".to_string(),
                    pos: CharPos::new(file_name.clone().into(), row, col)
                })?;
                if group.r#type == GroupType::Paren {
                    TokenType::Group(group)
                } else {
                    return Err(TokenError {
                        msg: "Mismatched parenthesis".to_string(),
                        pos: CharPos::new(file_name.clone().into(), row, col)
                    })
                }
            }
            '{' => {
                group_stack.push(Group { r#type: GroupType::Curly, tokens: Vec::new()});
                col += 1;
                continue;
            },
            '}' => {
                let group = group_stack.pop().ok_or(TokenError {
                    msg: "Mismatched curly bracket".to_string(),
                    pos: CharPos::new(file_name.clone().into(), row, col)
                })?;
                if group.r#type == GroupType::Curly {
                    TokenType::Group(group)
                } else {
                    return Err(TokenError {
                        msg: "Mismatched curly bracket".to_string(),
                        pos: CharPos::new(file_name.clone().into(), row, col)
                    })
                }
            },
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
                TokenType::Number(number_str.parse().unwrap()) // TODO: Unwrap
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut str = String::from(c);
                while let Some(char) = chars.peek() {
                    if (char >= &'a' && char <= &'z') || (char >= &'A' && char <= &'Z') || char == &'_' {
                        str.push(chars.next().unwrap());
                        col += 1;
                    } else {
                        break;
                    }
                }
                match str.as_str() {
                    "let" => TokenType::Keyword(Keyword::Let),
                    _ => TokenType::Variable(str)
                }
            }
            '"' => {
                let mut str = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    } else {
                        str.push(c)
                    }
                }
                TokenType::String(str)
            }
            c => TokenType::Char(c)
        };

        let token = Token {
            token_type,
            pos: CharPos::new(f.clone().into(), row, col)
        };

        if group_stack.is_empty() {
            tokens.push(token);
        } else {
            let mut group = group_stack.pop().unwrap();
            group.tokens.push(token);
            group_stack.push(group);
        }

        col += 1;
    }
    Ok(tokens)
}
