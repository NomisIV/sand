use crate::*;

pub fn init() -> Obj {
    let print = Intrinsic::new(
        vec![Var::new("value")],
        Rc::new(|scope: &mut Scope| {
            match scope.get(&Var::new("value")).unwrap() {
                Value::Str(string) => println!("{}", string),
                Value::Num(number) => println!("{}", number),
                Value::Var(variable) => match scope.get(variable).unwrap() {
                    Value::Str(string) => println!("{}", string),
                    Value::Num(number) => println!("{}", number),
                    value => eprintln!("{:?} cannot be printed", value),
                },
                value => eprintln!("{:?} cannot be printed", value),
            }
            Value::Nope
        }),
    );
    let mut main = Obj::new(Value::Nope);
    main.add_member(Var::new("print"), Value::Intrinsic(print));
    main
}
