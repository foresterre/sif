#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case(Some(-1))]
#[case(None)]
fn my_test(v: Option<i32>) {}

fn main() {}
