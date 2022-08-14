use std::collections::HashMap;

pub fn roman_to_integer(s: String) -> u32 {
    let dictionary = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut result: i32 = 0;
    let mut symbols = s.chars().peekable();

    while let Some(symbol) = symbols.next() {
        let value = dictionary.get(&symbol).unwrap_or(&0);
        let next_value = match symbols.peek() {
            Some(symbol) => dictionary.get(&symbol).unwrap_or(&0),
            _ => &0,
        };

        if value < next_value {
            result -= value;
        } else {
            result += value;
        }
    }

    result as u32
}
