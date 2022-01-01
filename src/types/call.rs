use super::Value;
use crate::Parseable;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
pub struct Call {
    function: String,
    parameters: Vec<Value>,
}

impl Parseable for Call {
    fn parse(string: &str) -> Option<Result<Self>> {
        let (before, after) = string.split_once('(')?;
        let function = before.trim().to_string();
        let mut parameters = Vec::new();
        for parameter_str in after.trim().strip_suffix(')')?.split(',') {
            match Value::parse(parameter_str) {
                Some(Ok(parameter)) => parameters.push(parameter),
                Some(Err(err)) => return Some(Err(err)),
                None => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following value:\n{}",
                        parameter_str
                    ))))
                }
            };
        }

        Some(Ok(Call {
            function,
            parameters,
        }))
    }
}
