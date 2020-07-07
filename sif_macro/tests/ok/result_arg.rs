#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case(Ok(()))]
#[case(Err(()))]
fn my_test(v: Result<(), ()>) {}

fn main() {}
