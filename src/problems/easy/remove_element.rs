pub fn solve() -> i32 {
    let nums: &mut Vec<i32> = &mut vec![0, 4, 4, 0, 4, 4, 4, 0, 2];
    let val = 4;

    let mut k = 0usize;
    for i in 0..nums.len() {
        let num = nums.get(i - k).unwrap().clone();
        // println!("num: {num}, k: {k}");
        if num == val {
            // println!("found val");
            let len = nums.len();
            nums.swap(i - k, len - 1 - k);
            k += 1;
        }
    }
    (nums.len() - k) as i32
}
