use crate::*;

use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub enum Literal {
    Nope,
    String(String),
    Number(isize), // TODO: Allow for all numbers
    Boolean(bool),
    Callable(Callable),
    Object(Object),
}

impl Literal {
    fn parse_string(string: &str) -> Option<Result<Self>> {
        let str = string
            .trim()
            .strip_prefix('"')?
            .strip_suffix('"')?
            .to_string();
        Some(Ok(Literal::String(str)))
    }

    fn parse_number(string: &str) -> Option<Result<Self>> {
        let num = string.parse().ok()?;
        Some(Ok(Literal::Number(num)))
    }

    fn parse_bool(string: &str) -> Option<Result<Self>> {
        let bool = string.parse().ok()?;
        Some(Ok(Literal::Boolean(bool)))
    }

    pub fn as_string(&self) -> Result<String> {
        match self {
            Literal::String(val) => Ok(val.clone()),
            _ => Err(Error::msg("Value is not a string"))
        }
    }

    pub fn as_number(&self) -> Result<isize> {
        match self {
            Literal::Number(val) => Ok(val.clone()),
            _ => Err(Error::msg("Value is not a number"))
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        match self {
            Literal::Boolean(val) => Ok(val.clone()),
            _ => Err(Error::msg("Value is not a boolean"))
        }
    }

    pub fn as_callable(&self) -> Result<Callable> {
        match self {
            Literal::Callable(val) => Ok(val.clone()),
            _ => Err(Error::msg("Value is not a callable"))
        }
    }

    // pub fn as_object(&self) -> Result<Object> {
    //     match self {
    //         Literal::Object(val) => Ok(val.clone()),
    //         _ => Err(Error::msg("Value is not an object"))
    //     }
    // }
}

impl Parseable for Literal {
    fn parse(string: &str) -> Option<Result<Self>> {
        // println!("== Parsing literal:\n{:?}", string);
        if let Some(str_result) = Literal::parse_string(string) {
            let str = match str_result {
                Ok(str) => str,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following string:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            Some(Ok(str))
        } else if let Some(num_result) = Literal::parse_number(string) {
            let num = match num_result {
                Ok(num) => num,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following number:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            Some(Ok(num))
        } else if let Some(bool_result) = Literal::parse_bool(string) {
            let bool = match bool_result {
                Ok(bool) => bool,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following boolean:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            Some(Ok(bool))
        } else if let Some(call_result) = Callable::parse(string) {
            let call = match call_result {
                Ok(call) => call,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following function:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            Some(Ok(Literal::Callable(call)))
        // } else if let Some(object_result) = Object::parse(string) {
        //     let object = match object_result {
        //         Ok(object) => object,
        //         Err(err) => {
        //             return Some(Err(Error::msg(format!(
        //                 "ERROR: Cannot parse the following object:\n{}\nbecause of:\n{}",
        //                 string, err
        //             ))))
        //         }
        //     };
        //     Some(Ok(Literal::Object(object)))
        } else {
            None
        }
    }
}
