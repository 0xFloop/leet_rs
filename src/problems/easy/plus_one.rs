pub fn solve() -> Vec<i32> {
    let nums = vec![9];
    let mut digits = nums.clone();
    let mut carry = false;

    for (i, digit) in digits.iter_mut().rev().enumerate() {
        carry = false;
        if digit < &mut 9 {
            *digit += 1;
            break;
        }

        carry = true;
        *digit = 0;
    }
    if carry {
        digits.insert(0, 1);
    }
    digits
}
