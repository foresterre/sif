#[macro_use]
extern crate sif_macro;

// a trailing comma after w's arguments (multiple inputs)
#[parameterized]
#[case(0, 1)]
#[case(2, 3)]
fn my_test(v: u32, w: u32) {
    assert_ne!(v, w);
}

fn main() {}
