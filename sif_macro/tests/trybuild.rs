#[test]
fn trybuild_regular_environment() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ok/comment_in_test_case.rs");
    t.pass("tests/ok/enum_arg.rs");
    t.pass("tests/ok/enum_with_variant_value_arg.rs");
    t.pass("tests/ok/import.rs");
    t.pass("tests/ok/import_rename.rs");
    t.pass("tests/ok/many_arg.rs");
    t.pass("tests/ok/multiline.rs");
    t.pass("tests/ok/multiple_ids.rs");
    t.pass("tests/ok/negative_number_arg.rs");
    t.pass("tests/ok/option_arg.rs");
    t.pass("tests/ok/result_arg.rs");
    t.pass("tests/ok/trailing_comma1.rs");
    t.pass("tests/ok/trailing_comma2.rs");
    t.pass("tests/ok/transitive_attr.rs");
    t.pass("tests/ok/visibility_1.rs");
    t.pass("tests/ok/visibility_2.rs");

    t.compile_fail("tests/fail/inequal_amount_of_arg.rs");
    t.compile_fail("tests/fail/not_a_fn.rs");
    t.compile_fail("tests/fail/on_visibility.rs");
    t.compile_fail("tests/fail/rename_without_followup_case_1.rs");
    t.compile_fail("tests/fail/rename_without_followup_case_2.rs");
    t.compile_fail("tests/fail/rename_without_followup_case_3.rs");

    // these ones can't be handled by the current version of trycompile, since we always have a cfg(test)
    // environment
    t.compile_fail("tests/fail/on_visibility_cfg_test.rs");
}
