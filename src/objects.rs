use crate::{
    types::*,
    Scope,
    Interpretable,
};
use std::{
    rc::Rc
};

pub fn init_main_obj() -> Object {
    let print = Intrinsic::new(
        vec![Var::new("value")],
        Rc::new(|scope: &mut Scope| {
            match scope.get(&Var::new("value")).unwrap() {
                Literal::String(string) => println!("{}", string),
                Literal::Number(number) => println!("{}", number),
                Literal::Boolean(boolean) => println!("{}", boolean),
                Literal::Nope => println!("Nope"),
                _ => eprintln!("Cannot print literal"),
            }
            Literal::Nope
        }),
    );
    let mut main = Object::new();
    main.add_member(
        Var::new("print"),
        Literal::Callable(Callable::Intrinsic(print)),
    );
    main
}

pub fn init_num_obj(number: isize) -> Object {
    let times = Intrinsic::new(
        vec![Var::new("f")],
        Rc::new(|scope: &mut Scope| {
            let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
            for n in 0..selff {
                let f = scope.get(&Var::new("f")).unwrap().as_callable().unwrap();
                let f = Value::Literal(Literal::Callable(f));
                let call = Call::new(f.clone(), vec![Value::Literal(Literal::Number(n))]);
                call.interpret(&mut scope.clone()).unwrap(); // TODO: handle unwrap
            }
            Literal::Nope
        }),
    );
    let pow = Intrinsic::new(
        vec![Var::new("n")],
        Rc::new(|scope: &mut Scope| {
            let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
            let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
            Literal::Number(selff.pow(n as u32))
        })
    );
    let mut num = Object::new();
    num.add_member(Var::new("self"), Literal::Number(number));
    num.add_member(
        Var::new("times"),
        Literal::Callable(Callable::Intrinsic(times)),
    );
    num.add_member(
        Var::new("pow"),
        Literal::Callable(Callable::Intrinsic(pow)),
    );
    num
}
