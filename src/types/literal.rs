use crate::*;

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
    fn parse_string(s: &str) -> Result<Self, SandParseError> {
        let str = s
            .trim()
            .strip_prefix('"')
            .ok_or(SandParseError::Unidentifiable(s.into(), "string".into()))?
            .strip_suffix('"')
            .ok_or(SandParseError::Unidentifiable(s.into(), "string".into()))?
            .to_string();
        Ok(Literal::String(str))
    }

    fn parse_number(s: &str) -> Result<Self, SandParseError> {
        let num = isize::from_str(s)
            .map_err(|_| SandParseError::Unidentifiable(s.into(), "number".into()))?;
        Ok(Literal::Number(num))
    }

    fn parse_bool(s: &str) -> Result<Self, SandParseError> {
        let bool = bool::from_str(s)
            .map_err(|_| SandParseError::Unidentifiable(s.into(), "bool".into()))?;
        Ok(Literal::Boolean(bool))
    }

    pub fn as_string(&self) -> Result<String, SandInterpretingError> {
        match self {
            Literal::String(val) => Ok(val.clone()),
            _ => Err(SandInterpretingError::BadValue),
        }
    }

    pub fn as_number(&self) -> Result<isize, SandInterpretingError> {
        match self {
            Literal::Number(val) => Ok(val.clone()),
            _ => Err(SandInterpretingError::BadValue),
        }
    }

    pub fn as_bool(&self) -> Result<bool, SandInterpretingError> {
        match self {
            Literal::Boolean(val) => Ok(val.clone()),
            _ => Err(SandInterpretingError::BadValue),
        }
    }

    pub fn as_callable(&self) -> Result<Callable, SandInterpretingError> {
        match self {
            Literal::Callable(val) => Ok(val.clone()),
            _ => Err(SandInterpretingError::BadValue),
        }
    }

    pub fn as_object(&self) -> Result<Object, SandInterpretingError> {
        match self {
            Literal::Object(val) => Ok(val.clone()),
            _ => Err(SandInterpretingError::BadValue),
        }
    }
}

impl FromStr for Literal {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing literal:\n{:?}", string);
        Literal::parse_string(s)
            .or(Literal::parse_number(s))
            .or(Literal::parse_bool(s))
            .or(Callable::from_str(s).map(|callable| Literal::Callable(callable)))
            // .or(Object::from_str(s).map(|object| Literal::Object(object)))
            // .or(Err(SandParseError::Unidentifiable(
            //     s.into(),
            //     "literal".into(),
            // )))
    }
}
