use std::u128;

pub fn solve() -> i32 {
    let x = 2147395600;
    let mut left: i128 = 1;
    let mut right = x;
    let mut ans = 0;

    while left <= right {
        let middle = left + (right - left) / 2;
        println!("{middle}");

        if middle * middle <= x {
            ans = middle;
            left = middle + 1
        } else {
            right = middle - 1;
        }
    }
    ans as i32
}
pub fn naive_solve() -> i32 {
    let x = 2147395600;

    let mut guess: u128 = 1;
    loop {
        println!(
            "guess: {guess}, square: {}, target value {}",
            guess * guess,
            x
        );
        if guess * guess > x as u128 {
            return guess as i32 - 1;
        }
        guess += 1;
    }
}
