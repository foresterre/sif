use sif_macro::parameterized;

#[parameterized]
#[case("hello", 1)]
#[case("hello")]
pub(crate) fn my_test(v: &str, w: i32) {}

fn main() {}
