use gcd::*;
use rstest::rstest;

#[rstest]
#[case(1071, 462, 21)]
fn gcd_test(#[case] a: u32, #[case] b: u32, #[case] expected: u32) {
    // Action
    let result = gcd(a, b);

    // Assert
    assert_eq!(result, expected);
}

