pub fn solve() -> u32 {
    let mut num = 38;
    loop {
        let digits: Vec<u32> = num.to_string().chars().map(|c| c as u32 - 48).collect();
        let mut curr_sum = 0;
        for digit in &digits {
            curr_sum += digit;
        }
        if curr_sum < 10 {
            return curr_sum;
        }
        num = curr_sum;
    }
    0
}
