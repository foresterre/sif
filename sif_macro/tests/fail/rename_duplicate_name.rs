#[macro_use]
extern crate sif_macro;

#[parameterized]
#[rename(one)]
#[case(1)]
#[rename(one)]
#[case(2)]
pub(crate) fn my_test(_a: u8) {}

fn main() {}
