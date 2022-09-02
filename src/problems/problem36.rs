/*
* The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
*
* Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
*
*/
use std::iter::zip;

fn is_palindrome(num: &str) -> bool {
    for (x, y) in zip(num.chars(), num.chars().rev()) {
        if x != y {
            return false;
        }
    };
    return true;
}

pub fn main() -> f64 {
    const MAX: i32 = 1_000_000;
    let mut total = 0;
    for elem in 1..MAX {
        if is_palindrome(elem.to_string().as_str()) {
            let bin = format!("{elem:b}");
            if is_palindrome(bin.as_str()) {
                total = total + elem;
            }
        }
    }
    return total as f64;
}
