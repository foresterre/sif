#[macro_use]
extern crate sif_macro;

// a trailing comma after v's arguments
#[parameterized]
#[case(1)]
fn my_test(v: u32) {
    assert_ne!(v, 0);
}

fn main() {}
