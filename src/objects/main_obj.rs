use crate::*;

pub fn init() -> Object {
    let print = Intrinsic::new(
        vec![Var::new("value")],
        Rc::new(|scope: &mut Scope| {
            match scope.get(&Var::new("value")).unwrap() {
                Literal::String(string) => println!("{}", string),
                Literal::Number(number) => println!("{}", number),
                Literal::Boolean(boolean) => println!("{}", boolean),
                Literal::Nope => println!("Nope"),
                _ => eprintln!("Cannot print literal")
            }
            Literal::Nope
        }),
    );
    let mut main = Object::new();
    main.add_member(Var::new("print"), Literal::Callable(Callable::Intrinsic(print)));
    main
}
