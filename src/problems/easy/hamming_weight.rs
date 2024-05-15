pub fn solve() -> i32 {
    let n = 2147483645;
    let mut count = 0;

    (0..32).for_each(|val| {
        if (n >> val & 1) == 1 {
            count += 1;
        }
    });

    count
}
