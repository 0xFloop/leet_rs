pub fn solve() -> bool {
    let s = String::from("A man, a plan, a canal: Panama");

    let parsed_s = s.to_lowercase().chars().fold(String::new(), |mut fold, c| {
        println!("c: {c}");
        if c.is_alphabetic() || c.is_ascii_digit() {
            fold.push(c);
        }
        fold
    });

    if parsed_s == parsed_s.chars().rev().collect::<String>() {
        return true;
    };

    false
}
