use std::collections::HashMap;

pub fn roman_to_integer(s: String) -> u32 {
    let dictionary = HashMap::from([
        ("I", 1),
        ("IV", 4),
        ("V", 5),
        ("IX", 9),
        ("X", 10),
        ("XL", 40),
        ("L", 50),
        ("XC", 90),
        ("C", 100),
        ("CD", 400),
        ("D", 500),
        ("CM", 900),
        ("M", 1000),
    ]);

    if s.len() == 0 {
        return 0;
    }

    let mut i = 0;
    let mut result = 0;

    while i < s.len() {
        let mut summand = 1;
        let symbol = s.get(i..i + 1).unwrap_or("");
        let next_symbol = s.get(i + 1..i + 2).unwrap_or("N");

        if dictionary.contains_key(&format!("{symbol}{next_symbol}").as_str()) {
            summand = 2;
        }

        result += dictionary
            .get(&format!("{symbol}{next_symbol}").as_str())
            .unwrap_or(dictionary.get(&symbol).unwrap_or(&0));

        i += summand;
    }

    result
}
