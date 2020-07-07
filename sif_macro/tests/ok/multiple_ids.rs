#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case("a", 1)]
#[case("b", 2)]
fn my_test(v: &str, w: i32) {}

fn main() {}
