pub fn main() -> f64 {
    let mut sum: i32 = 0;
    for index in 0i32..1000 {
        if (index % 3 == 0) || (index % 5 == 0) {
            sum = sum + index;
        }
    }
    return sum as f64;
}
