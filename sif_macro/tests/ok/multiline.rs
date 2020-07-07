#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case("v", 1)]
#[case("w", 2)]
fn my_test(v: &str, w: i32) {}

fn main() {}
