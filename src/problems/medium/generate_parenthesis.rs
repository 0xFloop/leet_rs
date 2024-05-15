pub fn solve() -> Vec<String> {
    let n = 3;

    let mut all_strings: Vec<String> = Vec::new();

    recurse(String::new(), &mut all_strings, n, n);

    all_strings
}
pub fn recurse(s: String, all_strings: &mut Vec<String>, num_open: i32, num_closed: i32) {
    if num_open == 0 && num_closed == 0 {
        all_strings.push(s);
        return;
    }
    if num_open < 0 || num_closed < 0 {
        return;
    }

    if num_open != 0 {
        recurse(s.clone() + "(", all_strings, num_open - 1, num_closed);
    }
    if num_closed > num_open {
        recurse(s + ")", all_strings, num_open, num_closed - 1);
    }
}
