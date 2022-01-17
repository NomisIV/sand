use crate::*;

pub fn init(number: isize) -> Object {
    let times = Intrinsic::new(
        vec![Var::new("f")],
        Rc::new(|scope: &mut Scope| {
            let selff = scope.get(&Var::new("self")).unwrap();
            if let Literal::Number(num) = selff {
                for n in 0..*num {
                    let f = scope.get(&Var::new("f")).unwrap().as_callable().unwrap();
                    let f = Value::Literal(Literal::Callable(f));
                    let call = Call::new(f.clone(), vec![Value::Literal(Literal::Number(n))]);
                    call.interpret(&mut scope.clone()).unwrap(); // TODO: handle unwrap
                }
            } else {
                unreachable!()
            }
            selff.clone()
        }),
    );
    let mut num = Object::new();
    num.add_member(Var::new("self"), Literal::Number(number));
    num.add_member(
        Var::new("times"),
        Literal::Callable(Callable::Intrinsic(times)),
    );
    num
}
