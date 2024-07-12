use std::rc::Rc;

type FnBox = Rc<dyn Fn(i32) -> i32>;
type ChurchNumeral = Rc<dyn Fn(FnBox) -> FnBox>;

fn main() {
    // Church numeral 0
    let zero: ChurchNumeral = Rc::new(|_f| Rc::new(move |x| x));

    // Successor function: λn.λf.λx.f (n f x)
    let succ = |n: ChurchNumeral| -> ChurchNumeral {
        Rc::new(move |f: FnBox| -> FnBox {
            let n_clone = n.clone();
            Rc::new(move |x: i32| f((n_clone)(f.clone())(x)))
        })
    };

    let one = succ(zero.clone());
    let two = succ(one.clone());
    let three = succ(two.clone());

    let to_int = |cn: ChurchNumeral| -> i32 { (cn)(Rc::new(|x| x + 1))(0) };

    println!("zero: {}", to_int(zero));
    println!("one: {}", to_int(one));
    println!("two: {}", to_int(two));
    println!("three: {}", to_int(three));
}
