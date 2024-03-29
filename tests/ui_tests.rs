#[test]
fn test_developer_experience() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
