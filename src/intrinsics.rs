use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::Interpret;
use crate::interpreter::InterpretingError;
use crate::interpreter::Scope;
use crate::types::*;
use crate::FilePos;

/* ======== MAIN ======== */
pub fn init_main() -> Literal {
    let mut main: Scope = HashMap::new();

    main.insert("STDOUT".to_string(), Literal::Num(1.0));
    main.insert("STDERR".to_string(), Literal::Num(2.0));

    main.insert(
        "write".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("stream"), Var::new("string")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let stream = scope.get("stream").unwrap().clone().as_int().unwrap();
                let string = scope.get("string").unwrap().clone().as_str().unwrap();
                match stream {
                    1 => println!("{}", string),
                    2 => eprintln!("{}", string),
                    _ => unreachable!(),
                }
                Ok(Literal::Nope)
            }),
        })),
    );

    main.insert(
        "dump".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                println!("Dumping scope:");
                for (var, val) in scope.iter() {
                    println!("== {}: {:?}", var, val)
                }
                Err(InterpretingError::new(
                    "Exiting after dumping scope",
                    &FilePos::internal(),
                ))
            }),
        })),
    );

    Literal::Set(main)
}

/* ======== NOPE ======== */
pub fn init_nope() -> Literal {
    let mut nope = HashMap::new();
    Literal::Set(nope)
}

/* ======== STRING ======== */
pub fn init_str() -> Literal {
    let mut str = HashMap::new();
    Literal::Set(str)
}

/* ======== NUMBER ======== */
pub fn init_num() -> Literal {
    let mut num = HashMap::new();
    // num.add_member(Var::new("self"), Literal::Number(number));

    num.insert(
        "add".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("n")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_num().unwrap();
                let n = scope.get("n").unwrap().clone().as_num().unwrap();
                Ok(Literal::Num(selff + n))
            }),
        })),
    );

    num.insert(
        "sub".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("n")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_num().unwrap();
                let n = scope.get("n").unwrap().clone().as_num().unwrap();
                Ok(Literal::Num(selff - n))
            }),
        })),
    );

    num.insert(
        "mul".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("n")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_num().unwrap();
                let n = scope.get("n").unwrap().clone().as_num().unwrap();
                Ok(Literal::Num(selff * n))
            }),
        })),
    );

    num.insert(
        "div".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("n")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_num().unwrap();
                let n = scope.get("n").unwrap().clone().as_num().unwrap();
                Ok(Literal::Num(selff / n))
            }),
        })),
    );

    num.insert(
        "mod".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("n")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_num().unwrap();
                let n = scope.get("n").unwrap().clone().as_num().unwrap();
                Ok(Literal::Num(selff % n))
            }),
        })),
    );

    num.insert(
        "pow".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("n")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_num().unwrap();
                let n = scope.get("n").unwrap().clone().as_num().unwrap();
                Ok(Literal::Num(selff.powf(n)))
            }),
        })),
    );
    num.insert(
        "eq".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("n")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_num().unwrap();
                let n = scope.get("n").unwrap().clone().as_num().unwrap();
                Ok(Literal::Bool(selff == n))
            }),
        })),
    );
    //
    // num.insert(
    //     "ne".to_string(),
    //     Literal::Fun(Callable::Intr(Intrinsic {
    //         args: vec!["n"],
    //         fun_interpret: Rc::new(|scope: &mut Scope| {
    //             let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
    //             let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
    //             Literal::Number(selff != n)
    //         }),
    //     })),
    // );
    //
    // num.add_member(
    //     Var::new("lt"),
    //     Literal::Callable(Callable::Intrinsic(Intrinsic::new(
    //         vec![Var::new("n")],
    //         Rc::new(|scope: &mut Scope| {
    //             let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
    //             let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
    //             Literal::Boolean(selff < n)
    //         }),
    //     ))),
    // );
    //
    // num.add_member(
    //     Var::new("gt"),
    //     Literal::Callable(Callable::Intrinsic(Intrinsic::new(
    //         vec![Var::new("n")],
    //         Rc::new(|scope: &mut Scope| {
    //             let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
    //             let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
    //             Literal::Boolean(selff > n)
    //         }),
    //     ))),
    // );
    //
    num.insert(
        "times".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("f")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_int().unwrap();
                for n in 0..selff {
                    let call = Value::FunCall {
                        fun: Box::new(Value::Lit(scope.get("f").unwrap().clone())),
                        params: vec![Value::Lit(Literal::Num(n as f64))],
                        pos: FilePos::internal(),
                    };
                    call.interpret(&mut scope.clone()).unwrap(); // TODO: handle unwrap
                }
                Ok(Literal::Nope)
            }),
        })),
    );

    // TODO: Implement this in the standard library instead
    num.insert(
        "to_str".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: Vec::new(),
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_num().unwrap();
                Ok(Literal::Str(selff.to_string()))
            }),
        })),
    );

    Literal::Set(num)
}

/* ======== BOOLEAN ======== */
pub fn init_bool() -> Literal {
    let mut bool = HashMap::new();

    bool.insert(
        "then".to_string(),
        Literal::Fun(Callable::Intr(Intrinsic {
            args: vec![Var::new("f")],
            fun_interpret: Rc::new(|scope: &mut Scope| {
                let selff = scope.get("self").unwrap().clone().as_bool().unwrap();
                if selff {
                    let call = Value::FunCall {
                        fun: Box::new(Value::Lit(scope.get("f").unwrap().clone())),
                        params: Vec::new(),
                        pos: FilePos::internal(),
                    };
                    call.interpret(&mut scope.clone()).unwrap(); // TODO: handle unwrap
                }
                Ok(Literal::Nope)
            }),
        })),
    );
    Literal::Set(bool)
}

/* ======== CALLABLE ======== */
pub fn init_fun() -> Literal {
    let mut fun = HashMap::new();
    Literal::Set(fun)
}

pub fn init_scope() -> Scope {
    let mut scope = HashMap::new();
    scope.insert("Main".to_string(), init_main());
    scope.insert("Nope".to_string(), init_nope());
    scope.insert("Str".to_string(), init_str());
    scope.insert("Num".to_string(), init_num());
    scope.insert("Bool".to_string(), init_bool());
    scope.insert("Fun".to_string(), init_fun());
    scope
}
