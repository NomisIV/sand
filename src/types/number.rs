use std::fmt::Display;

use crate::Parseable;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Num {
    num: f64,
}

impl Num {
    pub fn new(num: f64) -> Self {
        Self { num }
    }

    pub fn get_num(&self) -> f64 {
        self.num
    }
}

impl Parseable for Num {
    fn parse(string: &str) -> Option<Result<Self>> {
        let num = string.parse().ok()?;
        Some(Ok(Num { num }))
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}
