use crate::*;

pub fn init(number: Num) -> Obj {
    let times = Intrinsic::new(
        vec![Var::new("f")],
        Rc::new(|scope: &mut Scope| {
            let selff = scope.get(&Var::new("self")).unwrap();
            if let Value::Num(num) = selff {
                let num = num.get_num();
                for n in 0..(num as i64) {
                    // let mut new_scope = scope.clone();
                    // new_scope.insert(Var::new("n"), Value::Num(Num::new(n as f64)));
                    let f = scope.get(&Var::new("f")).unwrap();
                    let call = Call::new(f.clone(), vec![Value::Num(Num::new(n as f64))]);
                    call.interpret(&mut scope.clone()).unwrap(); // TODO: handle unwrap
                }
            } else {
                unreachable!()
            }
            selff.clone()
        }),
    );
    let mut num = Obj::new(Value::Num(number));
    num.add_member(Var::new("times"), Value::Intrinsic(times));
    num
}
