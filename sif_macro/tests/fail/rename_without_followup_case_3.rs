#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case(1)]
#[rename(rename_on_rename)]
#[rename(ok_rename)]
#[case(0)]
pub(crate) fn my_test(_a: u8) {}

fn main() {}
