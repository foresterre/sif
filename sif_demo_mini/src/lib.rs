//! Removing the doc comments below will enable failures, for you to observe (and enjoy, if you wish <3)

#[cfg(test)]
mod test_environment_scoped {
    use sif::parameterized;

    fn _zero() -> u8 {
        0
    }

    fn one() -> u8 {
        1
    }

    #[parameterized]
    #[case(2 - 1)]
    #[case(1)]
    #[case(one())]
    //#[case(_zero())] // case_3 should fail
    //#[case(0)] // case_4 should fail
    fn show_expect_one(inner: u8) {
        assert_eq!(inner, 1)
    }

    #[parameterized]
    #[case(None)]
    //#[case(Some(()))] // case_1 should fail
    #[should_panic]
    fn show_expect_panic(input: Option<()>) {
        input.unwrap()
    }

    #[parameterized]
    #[rename(test_joes)]
    #[case("Joes")]
    #[rename(test_joey)]
    #[case("Joey")]
    fn show_rename_next_case(name: &str) {
        assert!(name.starts_with("Joe"))
    }
}
