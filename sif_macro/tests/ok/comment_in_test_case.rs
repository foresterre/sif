#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case("1", // line 4 comment
/* line 5 (inline) comment */1,
)]
#[case("2"
    // line 8 comment
, 2)]
fn my_test(v: &str, w: i32) {}

fn main() {}
