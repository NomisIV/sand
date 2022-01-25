use crate::tokenizer::*;
use crate::types::*;
use crate::FilePos;
use crate::SandError;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub msg: String,
    pub pos: FilePos,
}

impl ParseError {
    fn new(msg: &str, tokens: Vec<Token>) -> Self {
        Self {
            msg: msg.to_string(),
            pos: tokens.get(0).unwrap().pos.clone(),
        }
    }
}

trait Parse {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>>
    where
        Self: Sized;
}

// TODO: Remove all instances of `?.ok()?`

impl Parse for Function {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>> {
        assert!(tokens.len() > 0);
        if tokens.len() != 2 {
            return None;
        }
        if let TokenType::Group {
            r#type: GroupType::Paren,
            tokens: args_tokens,
        } = &tokens.get(0).unwrap().r#type
        {
            let mut args_tokens = args_tokens.iter();
            let mut args = Vec::new();
            while let Some(token) = args_tokens.next() {
                if let Some(token2) = args_tokens.next() {
                    if let TokenType::Char(',') = token2.r#type {
                        args.push(Var::parse(&vec![token.clone()])?.ok()?);
                    } else {
                        return None; // TODO: This should be an error
                    }
                } else {
                    args.push(Var::parse(&vec![token.clone()])?.ok()?);
                }
            }

            if let TokenType::Group { r#type: GroupType::Curly, tokens: body_tokens } = &tokens.get(1).unwrap().r#type {
                let body = Statements::parse(&body_tokens)?.ok()?;
                    Some(Ok(Self {
                        args,
                        body,
                        pos: tokens.into(),
                    }))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Parse for Callable {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>> {
        Function::parse(tokens).map(|res| res.map(|fun| Callable::Fun(fun)))
    }
}

impl Parse for Literal {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>> {
        assert!(tokens.len() > 0);
        {
            if tokens.len() == 1 {
                match &tokens.get(0).unwrap().r#type {
                    TokenType::StringLit(s) => Some(Ok(Self::Str(s.clone()))),
                    TokenType::CharLit(c) => Some(Ok(Self::Char(c.clone()))),
                    TokenType::Number(n) => Some(Ok(Self::Num(n.clone()))),
                    TokenType::String(s) => match s.as_str() {
                        "Nope" => Some(Ok(Self::Nope)),
                        "True" => Some(Ok(Self::Bool(true))),
                        "False" => Some(Ok(Self::Bool(false))),
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        }
        .or_else(|| Callable::parse(tokens).map(|res| res.map(|fun| Literal::Fun(fun))))
    }
}

impl Parse for Value {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>> {
        assert!(tokens.len() > 0);
        Literal::parse(tokens)
            .map(|res| res.map(|lit| Value::Lit(lit)))
            .or_else(|| Reference::parse(tokens).map(|res| res.map(|r#ref| Value::Ref(r#ref))))
            .or_else(|| {
                let fun_tokens = &tokens.get(0..tokens.len() - 1).unwrap().to_vec();
                if fun_tokens.len() == 0 {
                    return None;
                }
                let fun = Box::new(match Value::parse(fun_tokens) {
                    Some(Ok(val)) => val,
                    Some(Err(err)) => return Some(Err(err)),
                    None => {
                        return Some(Err(ParseError::new(
                            "Cannot parse into function call",
                            fun_tokens.to_vec(),
                        )))
                    }
                });
                let params = if let TokenType::Group {
                    r#type: GroupType::Paren,
                    tokens,
                } = &tokens.last().unwrap().r#type
                {
                    let chain = tokens.split(|token| {
                        if let TokenType::Char(',') = token.r#type {
                            true
                        } else {
                            false
                        }
                    });

                    let mut params = Vec::new();
                    for param in chain {
                        if param.is_empty() {
                            continue;
                        }
                        params.push(match Value::parse(&param.to_vec()) {
                            Some(Ok(val)) => val,
                            Some(Err(err)) => return Some(Err(err)),
                            None => {
                                return Some(Err(ParseError::new(
                                    "Cannot parse into function call",
                                    fun_tokens.to_vec(),
                                )))
                            }
                        });
                    }
                    params
                } else {
                    return None;
                };

                Some(Ok(Self::FunCall {
                    fun,
                    params,
                    pos: tokens.into(),
                }))
            })
    }
}

impl Parse for Var {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>> {
        assert!(tokens.len() > 0);
        if tokens.len() == 1 {
            if let TokenType::String(var) = &tokens.get(0).unwrap().r#type {
                Some(Ok(Self {
                    name: var.to_string(),
                    pos: tokens.into(),
                }))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Parse for Reference {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>> {
        assert!(tokens.len() > 0);

        if tokens.len() == 1 {
            Var::parse(tokens).map(|res| res.map(|var| Reference::Var(var)))
        } else if tokens.len() >= 3 {
            if let TokenType::Char('.') = tokens.get(tokens.len() - 2).unwrap().r#type {
                let set = match Value::parse(&tokens.get(..tokens.len() - 2).unwrap().to_vec()) {
                    Some(Ok(val)) => Box::new(val),
                    Some(Err(err)) => return Some(Err(err)),
                    None => return None,
                };
                let field = match Var::parse(&vec![tokens.last().unwrap().clone()]) {
                    Some(Ok(var)) => var,
                    Some(Err(err)) => return Some(Err(err)),
                    None => return None,
                };
                Some(Ok(Reference::Member {
                    set,
                    field,
                    pos: tokens.into(),
                }))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Parse for Statement {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>> {
        assert!(tokens.len() > 0);
        let out = {
            if let TokenType::String(s) = &tokens.get(0).unwrap().r#type {
                if s == "let" {
                    let (var, val) = tokens.split_at(
                        tokens
                            .iter()
                            .position(|token| {
                                if let TokenType::Char('=') = token.r#type {
                                    true
                                } else {
                                    false
                                }
                            })
                            .unwrap(),
                    );
                    // let var = Reference::parse(var.get(1..).unwrap().to_vec())?.map_err(|err| Some(err))?;
                    let var = Reference::parse(&var.get(1..).unwrap().to_vec())?.ok()?;
                    let val = Value::parse(&val.get(1..).unwrap().to_vec())?.ok()?;

                    Some(Ok(Self::Assignment {
                        var,
                        val,
                        pos: tokens.into(),
                    }))
                } else {
                    None
                }
            } else {
                None
            }
        }
        .or_else(|| {
            if let TokenType::String(s) = &tokens.get(0).unwrap().r#type {
                if s == "include" {
                    // TODO: Allow for multiple files per include statement?
                    if let Some(token) = tokens.get(1) {
                        if let TokenType::StringLit(str) = &token.r#type {
                            let file = token.pos.file.parent().unwrap().join(str).canonicalize().unwrap();
                            Some(Ok(Self::Include(file)))
                        } else {
                            unimplemented!("Handle error if provided input argument isn't a string")
                        }
                    } else {
                        unimplemented!("Handle error if include has no argument")
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .or_else(|| Value::parse(tokens).map(|res| res.map(|val| Statement::Value(val))))
        .or_else(|| {
            Some(Err(ParseError::new(
                "Cannot parse into statement",
                tokens.to_vec(),
            )))
        });
        out
    }
}

impl Parse for Statements {
    fn parse(tokens: &Vec<Token>) -> Option<Result<Self, ParseError>> {
        assert!(tokens.len() > 0);
        let mut statements = Vec::new();
        let mut statement = Vec::new();
        for token in tokens.iter() {
            match token.r#type {
                TokenType::Char(';') => {
                    statements.push(match Statement::parse(&statement) {
                        Some(Ok(s)) => s,
                        Some(Err(err)) => return Some(Err(err)),
                        None => {
                            return Some(Err(ParseError::new(
                                "Cannot parse into a statement",
                                statement,
                            )))
                        }
                    });
                    statement = Vec::new();
                }
                _ => statement.push(token.clone()),
            }
        }
        if statement.is_empty() {
            statements.push(Statement::Value(Value::Lit(Literal::Nope)))
        } else {
            statements.push(match Statement::parse(&statement) {
                Some(Ok(s)) => s,
                Some(Err(err)) => return Some(Err(err)),
                None => {
                    return Some(Err(ParseError::new(
                        "Cannot parse into block",
                        tokens.to_vec(),
                    )))
                }
            });
        }
        Some(Ok(Self(statements)))
    }
}

pub fn parse_file(tokens: Vec<Token>) -> Result<Statements, SandError> {
    Statements::parse(&tokens)
        .ok_or_else(|| {
            SandError::from(ParseError::new(
                "File cannot be parsed",
                tokens,
            ))
        })?
        .map_err(|err| SandError::from(err))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // TODO: Add tests which *should* fail to check that parsing functions fail when they should

    // #[test]
    // fn parse_block() {
    //     let str = r#"{
    //         "Hello World!"
    //     }"#;
    //     let tokens = tokenize_str(str, &PathBuf::new(), 1, 1).unwrap();
    //     let block = Block::parse(&tokens).unwrap().unwrap();
    //     assert!(block.statements.len() == 1)
    // }
    //
    // #[test]
    // fn parse_block_multiple_statements() {
    //     let str = r#"{
    //         69;
    //         "Hello World!";
    //         420;
    //     }"#;
    //     let tokens = tokenize_str(str, &PathBuf::new(), 1, 1).unwrap();
    //     let block = Block::parse(&tokens).unwrap().unwrap();
    //     assert!(block.statements.len() == 4) // There is an impliced Literal::Nope as the last statement
    // }

    #[test]
    fn parse_statement_assignment() {
        let tokens = tokenize_str("let foo = \"hello\"", &PathBuf::new(), 1, 1).unwrap();
        let statement = Statement::parse(&tokens).unwrap().unwrap();
        match statement {
            Statement::Assignment { .. } => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_statement_value() {
        let tokens = tokenize_str("foo()", &PathBuf::new(), 1, 1).unwrap();
        let statement = Statement::parse(&tokens).unwrap().unwrap();
        match statement {
            Statement::Value(..) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_statement_include() {
        let tokens = tokenize_str("include \"std.sand\"", &PathBuf::new(), 1, 1).unwrap();
        let statement = Statement::parse(&tokens).unwrap().unwrap();
        assert!(statement == Statement::Include(PathBuf::from("std.sand")))
    }

    #[test]
    fn parse_reference_var() {
        let tokens = tokenize_str("foo", &PathBuf::new(), 1, 1).unwrap();
        let reference = Reference::parse(&tokens).unwrap().unwrap();
        match reference {
            Reference::Var(..) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_reference_member() {
        let tokens = tokenize_str("Foo.bar", &PathBuf::new(), 1, 1).unwrap();
        let reference = Reference::parse(&tokens).unwrap().unwrap();
        match reference {
            Reference::Member { .. } => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_reference_member_complex() {
        let tokens = tokenize_str("Foo.bar().baz", &PathBuf::new(), 1, 1).unwrap();
        let reference = Reference::parse(&tokens).unwrap().unwrap();
        match reference {
            Reference::Member { .. } => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_value_literal() {
        let tokens = tokenize_str("5", &PathBuf::new(), 1, 1).unwrap();
        let val = Value::parse(&tokens).unwrap().unwrap();
        match val {
            Value::Lit(..) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_value_reference() {
        let tokens = tokenize_str("foo", &PathBuf::new(), 1, 1).unwrap();
        let val = Value::parse(&tokens).unwrap().unwrap();
        match val {
            Value::Ref(..) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_value_funcall() {
        let tokens = tokenize_str("foo()", &PathBuf::new(), 1, 1).unwrap();
        let val = Value::parse(&tokens).unwrap().unwrap();
        match val {
            Value::FunCall { params, .. } => assert!(params.len() == 0),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_value_funcall_complex() {
        let tokens = tokenize_str("foo(1, \"hello\", bar())", &PathBuf::new(), 1, 1).unwrap();
        let val = Value::parse(&tokens).unwrap().unwrap();
        match val {
            Value::FunCall { params, .. } => assert!(params.len() == 3),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_literal_nope() {
        let tokens = tokenize_str("Nope", &PathBuf::new(), 1, 1).unwrap();
        let lit = Literal::parse(&tokens).unwrap().unwrap();
        assert!(lit == Literal::Nope)
    }

    #[test]
    fn parse_literal_string() {
        let tokens = tokenize_str("\"hello\"", &PathBuf::new(), 1, 1).unwrap();
        let lit = Literal::parse(&tokens).unwrap().unwrap();
        assert!(lit == Literal::Str("hello".to_string()))
    }

    #[test]
    fn parse_literal_number() {
        let tokens = tokenize_str("5", &PathBuf::new(), 1, 1).unwrap();
        let lit = Literal::parse(&tokens).unwrap().unwrap();
        assert!(lit == Literal::Num(5.0))
    }

    #[test]
    fn parse_literal_number_float() {
        let tokens = tokenize_str("5.0", &PathBuf::new(), 1, 1).unwrap();
        let lit = Literal::parse(&tokens).unwrap().unwrap();
        assert!(lit == Literal::Num(5.0))
    }

    #[test]
    fn parse_literal_bool() {
        let tokens = tokenize_str("True", &PathBuf::new(), 1, 1).unwrap();
        let lit = Literal::parse(&tokens).unwrap().unwrap();
        assert!(lit == Literal::Bool(true))
    }

    #[test]
    fn parse_literal_fun() {
        let tokens = tokenize_str("() { foo(); }", &PathBuf::new(), 1, 1).unwrap();
        let lit = Literal::parse(&tokens).unwrap().unwrap();
        match lit {
            Literal::Fun { .. } => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_function() {
        let tokens = tokenize_str("() { foo(); }", &PathBuf::new(), 1, 1).unwrap();
        let fun = Function::parse(&tokens).unwrap().unwrap();
        assert!(fun.args.len() == 0)
    }

    #[test]
    fn parse_function_complex() {
        let tokens = tokenize_str(
            "(foo, bar, baz) { foo(); \"hello\"; }",
            &PathBuf::new(),
            1,
            1,
        )
        .unwrap();
        let fun = Function::parse(&tokens).unwrap().unwrap();
        assert!(fun.args.len() == 3)
    }

    #[test]
    fn parse_variable1() {
        let tokens = tokenize_str("foo", &PathBuf::new(), 1, 1).unwrap();
        let fun = Var::parse(&tokens).unwrap().unwrap();
        assert!(fun.name == "foo")
    }

    #[test]
    fn parse_variable2() {
        let tokens = tokenize_str("foo_bar", &PathBuf::new(), 1, 1).unwrap();
        let fun = Var::parse(&tokens).unwrap().unwrap();
        assert!(fun.name == "foo_bar")
    }

    #[test]
    fn parse_variable_bad1() {
        let tokens = tokenize_str("5foo", &PathBuf::new(), 1, 1).unwrap();
        assert!(Var::parse(&tokens) == None)
    }

    #[test]
    fn parse_variable_bad2() {
        let tokens = tokenize_str("fo.o", &PathBuf::new(), 1, 1).unwrap();
        assert!(Var::parse(&tokens) == None)
    }
}
