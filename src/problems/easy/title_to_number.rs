pub fn solve() -> i32 {
    let mut column_title = String::from("FXSHRXW");
    let mut sum = 0;

    for (i, letter) in column_title.chars().rev().enumerate() {
        let letter_val = letter as i32 - 64;
        sum += letter_val * i32::pow(26, i as u32);
    }
    sum
}
