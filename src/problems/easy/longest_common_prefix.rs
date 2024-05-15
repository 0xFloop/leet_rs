pub fn solve() -> String {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    let mut curr_longest = "";

    let first_word = strs.get(0).unwrap();

    first_word.chars().enumerate().find_map(|(i, c)| {
        let x = strs
            .iter()
            .all(|s| s.starts_with(&first_word[..first_word.len() - i]));
        if x {
            curr_longest = &first_word[..first_word.len() - i];
            return Some(x);
        }
        None
    });

    curr_longest.to_string()
}
