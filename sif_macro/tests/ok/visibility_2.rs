#[macro_use]
extern crate sif_macro;

mod b {
    #[cfg(test)]
    fn call() {
        a::my_test::case_0(); // this is ok
    }

    mod a {
        #[parameterized]
        #[case(Some(0))]
        #[case(None)]
        pub(in crate::b) fn my_test(v: Option<i32>) {}
    }
}

fn main() {}
