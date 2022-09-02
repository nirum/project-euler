/*
*
* If we list all the natural numbers below 10 that are multiples of 3 or 5,
* we get 3, 5, 6 and 9. The sum of these multiples is 23.
*
* Find the sum of all the multiples of 3 or 5 below 1000.
*
*/

pub fn main() -> f64 {
    let mut sum: i32 = 0;
    for index in 0i32..1000 {
        if (index % 3 == 0) || (index % 5 == 0) {
            sum = sum + index;
        }
    }
    return sum as f64;
}
