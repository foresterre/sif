#[macro_use]
extern crate sif_macro;

enum Color {
    Red,
    Yellow,
    Blue,
}

#[parameterized]
#[case(Color::Red)]
#[case(Color::Yellow)]
#[case(Color::Blue)]
#[case(Color::Red)]
fn my_test(v: Color) {}

fn main() {}
