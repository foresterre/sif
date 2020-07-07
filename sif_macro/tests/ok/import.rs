#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case(1)]
#[case(2)]
fn my_test(_v: i32) {}

fn main() {}
