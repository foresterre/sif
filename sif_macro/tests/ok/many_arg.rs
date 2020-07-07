#[macro_use]
extern crate sif_macro;

#[parameterized]
#[case("a", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("b", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("c", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("d", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("e", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("f", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("g", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("h", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("i", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("j", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("k", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("l", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("m", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("n", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("o", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("p", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("q", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("r", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("s", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("t", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("u", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("v", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("w", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("x", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("y", 0, 1, 2, 3, 4, 5, 6, 7, false)]
#[case("z", 0, 1, 2, 3, 4, 5, 6, 7, false)]
fn my_test(a: &str, b: i64, c: i32, d: i16, e: i8, f: u64, g: u32, h: u16, i: u8, j: bool) {
    assert!(true);
}

fn main() {}
