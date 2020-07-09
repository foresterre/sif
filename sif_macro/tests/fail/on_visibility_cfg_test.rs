// see also 'tests/ok/visibility_2'

#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case(0)]
#[case(1)]
fn my_test(a: u8) {}

#[cfg(not(test))]
fn cant_call_since_in_cfg_test_mod() {
    my_test::case_0(); // this is not ok
}

fn main() {}
