pub fn solve() -> String {
    let mut column_number = 2147483647;
    let mut s = String::new();

    println!("col_number: {}", column_number);

    recurse(column_number, &mut s);

    s
}
fn recurse(val: u32, s: &mut String) {
    println!("{val}");
    let new_letter = (val - 1) % 26;

    if val < 27 {
        s.push((new_letter + 65) as u8 as char);
        return;
    }
    recurse((val - 1) / 26, s);

    s.push((new_letter + 65) as u8 as char);
}
