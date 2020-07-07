#[macro_use]
extern crate sif_macro;

// we want to copy the visibility from the fn

#[parameterized]
#[case("a", 1)]
#[case("b", 2)]
pub(crate) fn my_test(v: &str, w: i32) {}

// which should generate (something along these lines):
//
// ```
// pub(crate) mod fn my_test {
//      #[test]
//      pub(crate) my_test_0(v: &str, w: i32) {}
//
//      #[test]
//      pub(crate) my_test_0(v: &str, w: i32) {}
// }
// ```

fn main() {}
