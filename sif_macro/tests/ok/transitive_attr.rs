#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case(-1)]
#[case(0)]
#[case(1)]
#[case(2)]
#[case(3)]
#[should_panic]
fn my_test(number: i32) {
    panic!("panics with {}", number)
}

fn main() {}
