use crate::{types::*, Interpretable, Scope};
use std::rc::Rc;

/* ======== MAIN ======== */
pub fn init_main_obj() -> Object {
    let mut main = Object::new();

    main.add_member(
        Var::new("print"),
        Literal::Callable(Callable::Intrinsic(Intrinsic::new(
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
        ))),
    );
    main
}

/* ======== NUMBER ======== */
pub fn init_num_obj(number: isize) -> Object {
    let mut num = Object::new();
    num.add_member(Var::new("self"), Literal::Number(number));

    num.add_member(
        Var::new("times"),
        Literal::Callable(Callable::Intrinsic(Intrinsic::new(
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
        ))),
    );

    num.add_member(
        Var::new("add"),
        Literal::Callable(Callable::Intrinsic(Intrinsic::new(
            vec![Var::new("n")],
            Rc::new(|scope: &mut Scope| {
                let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
                let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
                Literal::Number(selff + n)
            }),
        ))),
    );

    num.add_member(
        Var::new("sub"),
        Literal::Callable(Callable::Intrinsic(Intrinsic::new(
            vec![Var::new("n")],
            Rc::new(|scope: &mut Scope| {
                let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
                let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
                Literal::Number(selff - n)
            }),
        ))),
    );

    num.add_member(
        Var::new("mul"),
        Literal::Callable(Callable::Intrinsic(Intrinsic::new(
            vec![Var::new("n")],
            Rc::new(|scope: &mut Scope| {
                let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
                let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
                Literal::Number(selff * n)
            }),
        ))),
    );

    num.add_member(
        Var::new("mul"),
        Literal::Callable(Callable::Intrinsic(Intrinsic::new(
            vec![Var::new("n")],
            Rc::new(|scope: &mut Scope| {
                let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
                let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
                Literal::Number(selff / n)
            }),
        ))),
    );

    num.add_member(
        Var::new("pow"),
        Literal::Callable(Callable::Intrinsic(Intrinsic::new(
            vec![Var::new("n")],
            Rc::new(|scope: &mut Scope| {
                let selff = scope.get(&Var::new("self")).unwrap().as_number().unwrap();
                let n = scope.get(&Var::new("n")).unwrap().as_number().unwrap();
                Literal::Number(selff.pow(n as u32))
            }),
        ))),
    );

    num
}
