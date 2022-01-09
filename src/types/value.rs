use crate::*;
use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub enum Value {
    Nope,
    Str(Str),
    Num(Num),
    Fun(Fun),
    Var(Var),
    Met(Met),
    Obj(Obj),
    Intrinsic(Intrinsic),
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
        } else if let Some(met_result) = Met::parse(string) {
            let met = match met_result {
                Ok(met) => met,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following method:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            return Some(Ok(Value::Met(met)));
        // } else if let Some(obj_result) = Obj::parse(string) {
        //     let obj = match obj_result {
        //         Ok(obj) => obj,
        //         Err(err) => {
        //             return Some(Err(Error::msg(format!(
        //                 "ERROR: Cannot parse the following object:\n{}\nbecause of:\n{}",
        //                 string, err
        //             ))))
        //         }
        //     };
        //     return Some(Ok(Value::Obj(obj)));
        } else {
            return None;
        }
    }
}

impl Interpretable for Value {
    fn interpret(&self, _scope: &mut Scope) -> Result<Value> {
        // println!("== Interpreting value:\n{:?}", self);
        // match self {
        //     Self::Nope => Ok(Self::Nope),
        //     Self::Str(str) => Ok(Value::Str(str.clone())),
        //     Self::Num(num) => Ok(Value::Num(num.clone())),
        //     Self::Obj(obj) => Ok(Value::Obj(obj.clone())),
        //     Self::Fun(fun) => fun.interpret(scope),
        //     Self::Var(var) => var.interpret(scope),
        //     Self::Met(met) => met.interpret(scope),
        //     Self::Intrinsic(intrinsic) => intrinsic.interpret(scope),
        // }
        Ok(self.clone())
    }
}
