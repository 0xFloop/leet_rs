pub fn solve() -> bool {
    let x = 894;

    let x_str = x.to_string();
    let rev_x = x_str.clone().chars().rev().collect::<String>();

    if x_str == rev_x {
        return true;
    }
    false
}
