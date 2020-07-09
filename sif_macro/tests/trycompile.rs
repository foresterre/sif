use trycompile::Runner;

#[test]
fn trycompile_with_test_env() {
    let runner = trycompile::TestCases::new()
        // these are important, since trybuild doesn't use a test environment!
        .should_fail("tests/fail/rename_duplicate_name.rs")
        // these also are covered by trybuild
        .should_fail("tests/fail/rename_without_followup_case_1.rs")
        .should_fail("tests/fail/rename_without_followup_case_2.rs")
        .should_fail("tests/fail/rename_without_followup_case_3.rs")
        .should_pass("tests/ok/comment_in_test_case.rs")
        .should_pass("tests/ok/enum_arg.rs")
        .should_pass("tests/ok/enum_with_variant_value_arg.rs")
        .should_pass("tests/ok/import.rs")
        .should_pass("tests/ok/import_rename.rs")
        .should_pass("tests/ok/many_arg.rs")
        .should_pass("tests/ok/multiline.rs")
        .should_pass("tests/ok/multiple_ids.rs")
        .should_pass("tests/ok/negative_number_arg.rs")
        .should_pass("tests/ok/option_arg.rs")
        .should_pass("tests/ok/result_arg.rs")
        .should_pass("tests/ok/trailing_comma1.rs")
        .should_pass("tests/ok/trailing_comma2.rs")
        .should_pass("tests/ok/transitive_attr.rs")
        .should_pass("tests/ok/visibility_1.rs")
        .should_pass("tests/ok/visibility_2.rs")
        .into_runner()
        .unwrap();

    let res = runner.run_test_cases();
    assert!(res.is_ok());
}
