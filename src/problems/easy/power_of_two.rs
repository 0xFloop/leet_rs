pub fn solve() -> u32 {
    let n: i32 = -2147483648;
    let ret = n.count_ones() == 1 && n.leading_zeros() > 0;
    println!("{:?}", ret);
    0
}
