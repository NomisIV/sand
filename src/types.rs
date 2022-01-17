use crate::{objects::*, Interpretable, SandInterpretingError, SandParseError, Scope};
use std::{collections::HashMap, fmt, rc::Rc, str::FromStr};

/* ======== ASSIGNMENT ======== */
#[derive(Debug, Clone)]
pub struct Assignment {
    var: Var,
    value: Value,
}

impl FromStr for Assignment {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing assignment:\n{:?}", string);
        let (before, after) = s.split_once('=').ok_or(SandParseError::Unidentifiable(
            s.into(),
            "assignment".into(),
        ))?;
        let var = Var::from_str(
            before
                .strip_prefix("let")
                .ok_or(SandParseError::Unidentifiable(
                    s.into(),
                    "assignment".into(),
                ))?
                .trim(),
        )?;
        let value = Value::from_str(after.trim())?;
        Ok(Assignment { var, value })
    }
}

impl Interpretable for Assignment {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting assignment:\n{:?}", self);
        scope.insert(
            self.var.clone(),
            self.value.clone().interpret(&mut scope.clone())?,
        );
        Ok(Literal::Nope)
    }
}

/* ======== BLOCK ======== */
#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Statement>,
}

impl FromStr for Block {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing block:\n{:?}", string);
        if !s.starts_with('{') || !s.ends_with('}') {
            return Err(SandParseError::Unidentifiable(s.into(), "block".into()));
        }

        let mut curly_lvl: usize = 0;
        let mut statements = Vec::new();
        let mut line = String::new();
        let chars = s
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .chars();

        for char in chars {
            line.push(char);
            match char {
                ';' => {
                    if curly_lvl == 0 {
                        let statement_str = line.trim().strip_suffix(';').unwrap();
                        let statement = Statement::from_str(statement_str)?;
                        statements.push(statement);
                        line = String::new();
                    }
                }
                '{' => curly_lvl += 1,
                '}' => curly_lvl -= 1,
                _ => (),
            }
        }

        Ok(Self { statements })
    }
}

impl Interpretable for Block {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting block:\n{:?}", self);
        for statement in &self.statements {
            statement.interpret(scope)?;
        }
        // TODO: Body implement a way to return values
        Ok(Literal::Nope)
    }
}

/* ======== CALL ======== */
#[derive(Debug, Clone)]
pub struct Call {
    callable: Box<Value>,
    parameters: Vec<Value>,
}

impl Call {
    pub fn new(callable: Value, parameters: Vec<Value>) -> Self {
        Self {
            callable: Box::new(callable),
            parameters,
        }
    }
}

impl FromStr for Call {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("\n== Parsing call:\n{:?}", s);
        if !(s.contains('(') && s.ends_with(')')) {
            return Err(SandParseError::Unidentifiable(s.into(), "call".into()));
        }
        let (before, after) = s.split_once('(').unwrap();
        let callable = Box::new(Value::from_str(before.trim())?);
        let mut parameters = Vec::new();
        for parameter_str in after.trim().strip_suffix(')').unwrap().split(',') {
            parameters.push(Value::from_str(parameter_str)?);
        }

        Ok(Call {
            callable,
            parameters,
        })
    }
}

impl Interpretable for Call {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting call:\n{:?}", self);
        let function = &self.callable.interpret(scope)?.as_callable()?;
        let arguments = function.get_arguments();

        if arguments.len() != self.parameters.len() {
            return Err(SandInterpretingError::MismatchedParameters);
        }
        // println!("#### Arguments:\n{:#?}", arguments);

        let mut function_scope = scope.clone();

        for n in 0..arguments.len() {
            function_scope.insert(
                arguments.get(n).unwrap().clone(),
                self.parameters.get(n).unwrap().clone().interpret(scope)?,
            );
        }

        // println!("Scope for calling the function:\n{:#?}", function_scope);

        function.interpret(&mut function_scope)
    }
}

/* ======== CALLABLE ======== */
// TODO: Implement this as a trait instead
#[derive(Debug, Clone)]
pub enum Callable {
    Function(Function),
    Intrinsic(Intrinsic),
}

impl Callable {
    pub fn new(intrinsic: Intrinsic) -> Self {
        Self::Intrinsic(intrinsic)
    }

    pub fn get_arguments(&self) -> Vec<Var> {
        match self {
            Callable::Function(function) => function.get_arguments(),
            Callable::Intrinsic(intrinsic) => intrinsic.get_arguments(),
        }
    }
}

impl FromStr for Callable {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing callable:\n{:?}", string);
        Err(SandParseError::Unidentifiable)
            .or(Function::from_str(s).map(|function| Callable::Function(function)))
    }
}

impl Interpretable for Callable {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        match self {
            Callable::Function(function) => function.interpret(scope),
            Callable::Intrinsic(intrinsic) => intrinsic.interpret(scope),
        }
    }
}

/* ======== FUNCTION ======== */
#[derive(Debug, Clone)]
pub struct Function {
    arguments: Vec<Var>,
    body: Block,
}

impl Function {
    pub fn get_arguments(&self) -> Vec<Var> {
        self.arguments.clone()
    }
}

impl FromStr for Function {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing function:\n{:?}", string);
        let (before, after) = s
            .split_once(')')
            .ok_or(SandParseError::Unidentifiable(s.into(), "function".into()))?;
        let mut arguments = Vec::new();
        for argument_str in before
            .trim()
            .strip_prefix('(')
            .ok_or(SandParseError::Unidentifiable(s.into(), "function".into()))?
            .split(',')
        {
            if argument_str.is_empty() {
                continue; // TODO: This might cause bugs with empty arguments
            }

            arguments.push(Var::from_str(argument_str)?);
        }
        let body = Block::from_str(after.trim())?;

        Ok(Function { arguments, body })
    }
}

impl Interpretable for Function {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("Interpreting function:\n{:?}", self);
        self.body.interpret(scope)
    }
}

/* ======== INTRINSIC ======== */
#[derive(Clone)]
pub struct Intrinsic {
    arguments: Vec<Var>,
    function: Rc<dyn Fn(&mut Scope) -> Literal>,
}

impl fmt::Debug for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Intrinsic")
            .field("arguments", &self.arguments)
            .field("function", &"rust code lol")
            .finish()
    }
}

impl Intrinsic {
    pub fn new(arguments: Vec<Var>, function: Rc<dyn Fn(&mut Scope) -> Literal>) -> Self {
        Self {
            arguments,
            function,
        }
    }

    pub fn get_arguments(&self) -> Vec<Var> {
        self.arguments.clone()
    }
}

impl Interpretable for Intrinsic {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting intrinsic:\n{:?}", self);
        let function = &*self.function;
        Ok(function(scope))
    }
}

/* ======== LITERAL ======== */
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

/* ======== MEMBER ======== */
#[derive(Debug, Clone)]
pub struct Member {
    object: Box<Value>,
    field: Var,
}

impl FromStr for Member {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing member:\n{:?}", string);
        if !s.contains('.') {
            return Err(SandParseError::Unidentifiable(s.into(), "member".into()));
        }
        let (value_str, field_str) = s.rsplit_once('.').unwrap();
        let value = Value::from_str(value_str)?;
        let object = Box::new(value);
        let field = Var::from_str(field_str)?;
        Ok(Member { object, field })
    }
}

impl Interpretable for Member {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting method:\n{:?}", self);
        let object = Object::from_literal(self.object.interpret(scope)?);
        object.interpret(scope)?;
        if let Some(literal) = object.get_member(&self.field) {
            Ok(literal.clone())
        } else {
            return Err(SandInterpretingError::NoMember);
        }
    }
}

/* ======== OBJECT ======== */
// TODO: Objects are basically just sets, so rename objects to sets instead
#[derive(Debug, Clone)]
pub struct Object {
    members: HashMap<Var, Literal>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
        }
    }

    pub fn from_literal(literal: Literal) -> Self {
        match literal {
            // Literal::Nope => (),
            // Literal::String(string) => (),
            Literal::Number(number) => init_num_obj(number),
            // Literal::Boolean(bool) => (),
            // Literal::Callable(callable) => (),
            Literal::Object(object) => object,
            _ => {
                let mut object = Object::new();
                object.add_member(Var::new("self"), literal);
                object
            }
        }
    }

    pub fn add_member(&mut self, name: Var, value: Literal) {
        self.members.insert(name, value);
    }

    pub fn get_member(&self, name: &Var) -> Option<&Literal> {
        self.members.get(name)
    }
}

impl FromStr for Object {
    type Err = SandParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing object:\n{:?}", string);
        unimplemented!()
    }
}

impl Interpretable for Object {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        for (var, value) in self.members.iter() {
            scope.insert(var.clone(), value.clone());
        }
        Ok(Literal::Object(self.clone()))
    }
}

/* ======== STATEMENT ======== */
#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Assignment),
    Value(Value),
}

impl FromStr for Statement {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing statement:\n{:?}", string);
        Assignment::from_str(s)
            .map(|assignment| Statement::Assignment(assignment))
            .or(Value::from_str(s).map(|value| Statement::Value(value)))
    }
}

impl Interpretable for Statement {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting statement:\n{:?}", self);
        match self {
            Self::Assignment(assignment) => assignment.interpret(scope),
            Self::Value(call) => call.interpret(scope),
        }
    }
}

/* ======== VALUE ======== */
#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Variable(Var),
    Member(Member),
    Call(Call),
}

impl Value {}

impl FromStr for Value {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("\n== Parsing value:\n{:?}", s);
        Literal::from_str(s)
            .map(|literal| Value::Literal(literal))
            .or(Var::from_str(s).map(|var| Value::Variable(var)))
            .or(Member::from_str(s).map(|member| Value::Member(member)))
            .or(Call::from_str(s).map(|call| Value::Call(call)))
        // .map_err(|err| match err {
        //     SandParseError::ParseErr(msg) => SandParseError::ParseErr(format!(
        //         "Cannot parse the following string into a value:\n{}\nbecause of:\n{}",
        //         s, msg
        //     )),
        //     SandParseError::Unidentifiable(_, _) => err,
        // })
    }
}

impl Interpretable for Value {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        match self {
            Value::Literal(literal) => Ok(literal.clone()),
            Value::Variable(variable) => variable.interpret(scope),
            Value::Member(member) => member.interpret(scope),
            Value::Call(call) => call.interpret(scope),
        }
    }
}

/* ======== VARIABLE ======== */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl FromStr for Var {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing variable:\n{:?}", string);
        let allowed_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVXYZ123456789_-";
        if let Some(_) = s.chars().find(|char| !allowed_chars.contains(*char)) {
            return Err(SandParseError::Unidentifiable(s.into(), "variable".into()));
        }
        let name = s.to_string();
        Ok(Var { name })
    }
}

impl Interpretable for Var {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("Interpreting variable:\n{:?}", self);
        scope
            .get(&self)
            .ok_or(SandInterpretingError::NotInScope)
            .map(|value| value.clone())
    }
}
