/*
* Each new term in the Fibonacci sequence is generated by adding the previous two terms.
* By starting with 1 and 2, the first 10 terms will be:
*
* 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
*
* By considering the terms in the Fibonacci sequence whose values do not exceed four million,
* find the sum of the even-valued terms.
*
*/

fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

pub fn main() -> f64 {
    const SENTINEL: i32 = 4_000_000;

    let mut x = 1;
    let mut y = 2;
    let mut fib = vec![x, y];

    while y <= SENTINEL {
        let new = x + y;
        x = y;
        y = new;
        fib.push(new);
    }

    let mut total = 0;
    for elem in fib.iter() {
        if is_even(elem) {
            total = total + elem;
        }
    }
    return total as f64;
}
