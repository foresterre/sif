#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case(0)]
#[rename(after_case_on_nothing)]
pub(crate) fn my_test(a: u8) {}

fn main() {}
