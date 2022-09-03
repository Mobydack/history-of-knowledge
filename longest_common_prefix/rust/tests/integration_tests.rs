use longest_common_prefix::*;
use rstest::rstest;

#[rstest]
#[case(["flight", "flow", "flower"], "fl")]
fn longest_common_prefix_test(#[case] input: Vec<impl AsRef<String>>, #[case] expected: impl AsRef<String>) {
    let result = longest_common_prefix(input.iter().map(|value| value.as_ref()).collect()));

    assert_eq!(result, expected.as_ref())
}
