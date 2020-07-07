#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case(-1)]
#[case(0)]
fn my_test(v: i32) {}

fn main() {}
