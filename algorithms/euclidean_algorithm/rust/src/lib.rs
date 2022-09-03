pub fn gcd(a: u32, b: u32) -> u32 {
    let r = a % b;

    if r > 0 {
        gcd(b, r)
    } else {
        b
    }
}
