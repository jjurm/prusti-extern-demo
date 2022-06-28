extern crate prusti_contracts;
extern crate demo_lib;

use prusti_contracts::*;

use demo_lib::Opt;

// Task: allow importing the following specification
// #[extern_spec]
// impl<T> Option<T> {
//     #[pure]
//     #[ensures(matches!(*self, Some(_)) == result)]
//     fn is_some(&self) -> bool;
// }

// #[requires(arg1 == arg2)]
// fn foo<T: Ord>(arg1: T, arg2: T) {
//     arg1 == arg2;
// }


#[extern_spec]
impl<T> Opt<T> {
    #[ensures(matches!(*self, Opt::OSome(_)) == result)]
    fn is_some(&self) -> bool;
}


fn main() {
    let a = Opt::OSome(42);
    let b = Opt::ONone::<i32>;


    assert!(a.is_some() == true);
    assert!(b.is_some() == false);

    //print!("Done.")

    //foo(5, 5);
}
