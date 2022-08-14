use roman_to_integer::*;
use rstest::rstest;

#[rstest]
#[case("", 0)]
#[case("I", 1)]
#[case("III", 3)]
#[case("IV", 4)]
#[case("IX", 9)]
#[case("XX", 20)]
#[case("XL", 40)]
#[case("XC", 90)]
#[case("XCIV", 94)]
#[case("CD", 400)]
#[case("LVIII", 58)]
#[case("MCMXCIV", 1994)]
fn roman_to_integer_test(#[case] input: impl AsRef<str>, #[case] expected: u32) {
    // Action
    let result = roman_to_integer(String::from(input.as_ref().to_owned()));

    // Assert
    assert_eq!(result, expected);
}

