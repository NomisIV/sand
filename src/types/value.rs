use super::Fun;
use super::Num;
use super::Str;
use super::Var;
use crate::Parseable;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
pub enum Value {
    Str(Str),
    Num(Num),
    Fun(Fun),
    Var(Var),
}

impl Parseable for Value {
    fn parse(string: &str) -> Option<Result<Self>> {
        if let Some(str_result) = Str::parse(string) {
            let str = match str_result {
                Ok(str) => str,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following string:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            return Some(Ok(Value::Str(str)));
        } else if let Some(num_result) = Num::parse(string) {
            let num = match num_result {
                Ok(num) => num,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following number:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            return Some(Ok(Value::Num(num)));
        } else if let Some(fun_result) = Fun::parse(string) {
            let fun = match fun_result {
                Ok(fun) => fun,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following function:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            return Some(Ok(Value::Fun(fun)));
        } else if let Some(var_result) = Var::parse(string) {
            let var = match var_result {
                Ok(var) => var,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following variable:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            return Some(Ok(Value::Var(var)));
        } else {
            return None;
        }
    }
}
