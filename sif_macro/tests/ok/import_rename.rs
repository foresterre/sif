use sif_macro::parameterized as pm;

#[pm]
#[case(())]
fn my_test(v: ()) {
    assert_eq!(v, ());
}

fn main() {}
