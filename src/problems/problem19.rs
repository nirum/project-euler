/*
*
* How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
*
*/

const MONTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            }
            return false;
        }
        return true;
    }
    return false;
}

pub fn main() -> f64 {

    let mut current_day = 2;  // Starts on a Monday.
    let mut num_sundays = 0;
    
    for year in 1901..=2000 {
        for (index, num_days) in MONTHS.iter().enumerate() {
            if current_day % 7 == 0 {
                num_sundays += 1;
            }

            current_day += num_days;
            if index == 1 && is_leap_year(year) {
                current_day += 1;
            }
        }
    }

    return num_sundays as f64;
}
