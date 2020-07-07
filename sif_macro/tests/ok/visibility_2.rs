#[macro_use]
extern crate sif_macro;

pub mod a {
    #[parameterized]
    #[case(Some(- 1))]
    #[case(None)]
    pub(in crate::b) fn my_test(v: Option<i32>) {}
}

mod b {
    #[cfg(test)]
    fn call() {
        a::my_test::case_0(); // this is ok
    }
}

fn main() {}
