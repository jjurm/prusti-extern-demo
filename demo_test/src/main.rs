extern crate prusti_contracts;
extern crate demo_lib;

use demo_lib::Opt;

// Task: allow importing the following specification

// use prusti_contracts::*;
// #[extern_spec]
// impl<T> Opt<T> {
//     #[ensures(matches!(*self, Opt::OSome(_)) == result)]
//     fn is_some(&self) -> bool;
// }

fn main() {
    let a = Opt::OSome(42);
    let b = Opt::ONone::<i32>;

    assert!(a.is_some() == true);
    assert!(b.is_some() == false);
}
